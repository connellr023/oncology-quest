use actix_web::{web::Data, HttpResponse, Responder};
use redis::Client;

pub async fn redis_test(redis: Data<Client>) -> impl Responder {
    if let Ok(mut con) = redis.get_connection() {
        let _: () = redis::cmd("SET")
            .arg("my_key")
            .arg(42)
            .query(&mut con)
            .expect("Failed to set key");

        let result: i32 = redis::cmd("GET")
            .arg("my_key")
            .query(&mut con)
            .expect("Failed to get key");

        HttpResponse::Ok().body(format!("Value of my_key: {}", result))
    } else {
        HttpResponse::InternalServerError().finish()
    }
}