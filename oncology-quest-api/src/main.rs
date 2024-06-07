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

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Load from .env file
    dotenv().ok();

    // Load environment variables.
    let env = Environment::new().expect("Failed to read environment variables");
    let env_clone = env.clone();

    // Setup Postgres connection pool
    let pool = PgPool::connect(env.database_url())
        .await
        .expect("Failed to create database connection pool");

    // Print server details.
    println!("Starting server at http://{}:{}", env.host_ip(), env.host_port());

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
        env_clone.host_ip(),
        env_clone.host_port()
    ))?
    .run()
    .await
}

fn session_middleware(key: &[u8]) -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(
        CookieSessionStore::default(),
        Key::from(key)
    )
    .cookie_name(String::from("oncology-quest-session"))
    .cookie_secure(false)
    .cookie_same_site(SameSite::None)
    .cookie_http_only(true)
    .session_lifecycle(
        PersistentSession::default()
            .session_ttl(Duration::hours(6))
    )
    .build()
}

fn cors() -> Cors {
    Cors::permissive() // For now
        .supports_credentials()
}