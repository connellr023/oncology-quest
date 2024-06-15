// Author: Connell Reffo
#![crate_name="oncology_quest_api"]

mod services;
mod models;
mod utilities;

use rand::{thread_rng, RngCore};
use utilities::environment::Environment;
use actix_web::{cookie::{time::Duration, Key, SameSite}, web::Data, App, HttpServer};
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use std::io;
use dotenv::dotenv;
use services::config::config;
use actix_cors::Cors;
use sqlx::PgPool;

const SESSION_COOKIE_NAME: &str = "oncology-quest-session";
const SESSION_COOKIE_DURATION: i64 = 6;

#[cfg(feature = "production")]
mod prod_config {
    use actix_web::http::header::{self, HeaderName};

    pub const ALLOWED_ORIGIN: &str = "http://oncology-quest-alb-1095195425.us-west-1.elb.amazonaws.com";
    pub const ALLOWED_METHODS: [&str; 4] = ["GET", "PATCH", "POST", "DELETE"];
    pub const ALLOWED_HEADERS: [HeaderName; 2] = [header::ACCEPT, header::AUTHORIZATION];
    pub const ALLOWED_HEADER: HeaderName = header::CONTENT_TYPE;
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Load from .env file
    dotenv().ok();

    // Load environment variables.
    let env = Environment::new().expect("Failed to read environment variables");

    // Setup Postgres connection pool
    let pool = PgPool::connect(env.database_url())
        .await
        .expect("Failed to create database connection pool");

    // Print server details.
    println!("{}", env);

    // Generate session key.
    let mut key = [0u8; 64];
    thread_rng().fill_bytes(&mut key);

    // Start HTTP server.
    HttpServer::new(move || {
        
        // Initialize the application.
        App::new()
            .app_data(Data::new(pool.clone()))
            .configure(config)
            .wrap(session_middleware(&key))
            .wrap(cors())
    })
    .bind(format!("{}:{}",
        env.host_ip(),
        env.host_port()
    ))?
    .run()
    .await
}

#[cfg(not(feature = "production"))]
fn session_middleware(key: &[u8]) -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(
        CookieSessionStore::default(),
        Key::from(key)
    )
    .cookie_name(String::from(SESSION_COOKIE_NAME))
    .cookie_secure(false)
    .cookie_same_site(SameSite::None)
    .cookie_http_only(true)
    .session_lifecycle(
        PersistentSession::default()
            .session_ttl(Duration::hours(SESSION_COOKIE_DURATION))
    )
    .build()
}

#[cfg(feature = "production")]
fn session_middleware(key: &[u8]) -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(
        CookieSessionStore::default(),
        Key::from(key)
    )
    .cookie_name(String::from(SESSION_COOKIE_NAME))
    .cookie_secure(true)
    .cookie_same_site(SameSite::Lax)
    .cookie_http_only(true)
    .session_lifecycle(
        PersistentSession::default()
            .session_ttl(Duration::hours(SESSION_COOKIE_DURATION))
    )
    .build()
}

#[cfg(not(feature = "production"))]
fn cors() -> Cors {
    Cors::permissive()
        .supports_credentials()
}

#[cfg(feature = "production")]
fn cors() -> Cors {
    Cors::default()
        .allowed_origin(prod_config::ALLOWED_ORIGIN)
        .allowed_methods(prod_config::ALLOWED_METHODS)
        .allowed_headers(prod_config::ALLOWED_HEADERS)
        .allowed_header(prod_config::ALLOWED_HEADER)
        .supports_credentials()
        .max_age(3600)
}