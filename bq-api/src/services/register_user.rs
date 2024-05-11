use crate::utilities::parsables::{Username, Name, Email, PlainTextPassword};
use crate::models::{model::Model, user::User};
use actix_web::{web::{Json, Data}, HttpResponse, Responder};
use serde::Deserialize;
use redis::Client;

#[derive(Deserialize)]
struct RegisterUser {
    pub username: Username,
    pub name: Name,
    pub email: Email,
    pub password: PlainTextPassword
}

#[actix_web::post("/api/user/register")]
pub(super) async fn register(redis: Data<Client>, create_user: Json<RegisterUser>) -> impl Responder {
    let create_user = create_user.into_inner();
    let user = match User::new(
        create_user.username,
        create_user.name,
        create_user.email,
        create_user.password,
        false
    ) {
        Some(user) => user,
        None => return HttpResponse::InternalServerError().finish()
    };

    let mut connection = match redis.get_connection() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    if user.store(&mut connection) {
        return HttpResponse::Created().finish();
    }
    
    HttpResponse::InternalServerError().finish()
}