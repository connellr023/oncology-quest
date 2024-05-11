// Author: Connell Reffo
#![crate_name="bq_api"]

mod services;
mod models;
mod utilities;

use std::io;
use rand::{thread_rng, RngCore};
use dotenv::dotenv;
use services::config::config;
use models::environment::Environment;
use redis::Client;
use actix_web::{cookie::{time::Duration, Key, SameSite}, web::Data, App, HttpServer};
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Load environment variables.
    dotenv().ok();
    let env = Environment::new().expect("Failed to read environment variables");
    let env_clone = env.clone();

    // Setup Redis client.
    let redis = Client::open(format!("redis://{}:{}@{}:{}/{}",
        env.redis_user(),
        env.redis_password(),
        env.redis_endpoint(),
        env.redis_port(),
        env.redis_db()
    )).expect("Failed to create Redis client");

    // Print server details.
    println!("Starting server at http://{}:{}", env.host_ip(), env.host_port());

    // Generate session key.
    let mut key = [0u8; 64];
    thread_rng().fill_bytes(&mut key);

    // Start HTTP server.
    HttpServer::new(move || {
        // Initialize the application.
        App::new()
            .app_data(Data::new(redis.clone()))
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
    .cookie_name(String::from("bq-session"))
    .cookie_secure(false) // For now
    .session_lifecycle(
        PersistentSession::default()
            .session_ttl(Duration::hours(6))
    )
    .cookie_same_site(SameSite::None)
    //.cookie_content_security(CookieContentSecurity::Private)
    .cookie_http_only(true) // For now
    .build()
}

fn cors() -> Cors {
    Cors::permissive() // For now
        .supports_credentials()
}