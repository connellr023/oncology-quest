// Author: Connell Reffo
#![crate_name="bq_api"]

mod services;
mod models;
mod utilities;

use std::io;
use dotenv::dotenv;
use services::config::config;
use models::environment::Environment;
use actix_web::{web::Data, cookie::Key, App, HttpServer};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use redis::Client;

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

    // Start HTTP server.
    HttpServer::new(move || {
        // Setup session middleware.
        let session_middleware = SessionMiddleware::builder(
            CookieSessionStore::default(),
            Key::from(&[0; 64])
        ).build();

        // Initialize the application.
        App::new()
            .app_data(Data::new(redis.clone()))
            .configure(config)
            .wrap(session_middleware)
    })
    .bind(format!("{}:{}",
        env_clone.host_ip(),
        env_clone.host_port()
    ))?
    .run()
    .await
}