mod services;
mod models;

use std::io;
use dotenv::dotenv;
use services::*;
use models::environment::Environment;
use actix_web::{web::Data, App, HttpServer};
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
        App::new()
            .app_data(Data::new(redis.clone()))
            .configure(config::config)
    })
    .bind(format!("{}:{}",
        env_clone.host_ip(),
        env_clone.host_port()
    ))?
    .run()
    .await
}