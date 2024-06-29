// Author: Connell Reffo
#![crate_name="oncology_quest_api"]

mod services;
mod models;
mod utilities;
mod middlewares;

use actix_web::{web::Data, App, HttpServer};
use std::{io::Result, env::var};
use dotenv::dotenv;
use services::config::config;
use actix_cors::Cors;
use sqlx::PgPool;

#[cfg(feature = "production")]
mod prod_config {
    use actix_web::http::header::{self, HeaderName};

    pub const ALLOWED_ORIGIN: &str = "https://www.oncologyquest.net";
    pub const ALLOWED_METHODS: [&str; 4] = ["GET", "PATCH", "POST", "DELETE"];
    pub const ALLOWED_HEADERS: [HeaderName; 2] = [header::ACCEPT, header::AUTHORIZATION];
    pub const ALLOWED_HEADER: HeaderName = header::CONTENT_TYPE;
}

#[actix_web::main]
async fn main() -> Result<()> {
    // Load from .env file
    dotenv().ok();

    let host_ip = var("HOST_IP").expect("Expected host IP.");
    let host_port = var("HOST_PORT").expect("Expected host port.");
    let database_url = var("DATABASE_URL").expect("Expected database URL.");

    // Setup Postgres connection pool
    let pool = PgPool::connect(database_url.as_str())
        .await
        .expect("Failed to create database connection pool.");

    // Print server details.
    println!("Server running on: {}:{}", host_ip, host_port);

    // Start HTTP server.
    HttpServer::new(move || {

        // Initialize the application.
        App::new()
            .app_data(Data::new(pool.clone()))
            .configure(config)
            .wrap(cors())
    })
    .bind(format!("{}:{}",
        host_ip,
        host_port
    ))?
    .run()
    .await
}

#[cfg(not(feature = "production"))]
#[inline(always)]
fn cors() -> Cors {
    Cors::permissive()
        .supports_credentials()
}

#[cfg(feature = "production")]
#[inline(always)]
fn cors() -> Cors {
    Cors::default()
        .allowed_origin(prod_config::ALLOWED_ORIGIN)
        .allowed_methods(prod_config::ALLOWED_METHODS)
        .allowed_headers(prod_config::ALLOWED_HEADERS)
        .allowed_header(prod_config::ALLOWED_HEADER)
        .supports_credentials()
        .max_age(3600)
}