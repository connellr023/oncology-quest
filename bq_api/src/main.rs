mod routes;
mod models;

use routes::*;
use models::environment::Environment;
use std::io;
use dotenv::dotenv;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Load environment variables
    dotenv().ok();
    let env = Environment::new().expect("Failed to read environment variables");

    // Start HTTP server
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index::index))
    })
    .bind(format!("{}:{}", env.host_ip(), env.host_port()))?
    .run()
    .await
}