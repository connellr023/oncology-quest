use super::user_session::session_response_json;
use crate::models::{model::Model, user::User};
use actix_web::{web::{Json, Data}, HttpResponse, Responder};
use actix_session::Session;
use serde::Deserialize;
use redis::Client;

#[derive(Deserialize)]
struct LoginUser {
    pub username: String,
    pub password: String
}

#[actix_web::post("/api/user/login")]
pub(super) async fn login(session: Session, redis: Data<Client>, login_user: Json<LoginUser>) -> impl Responder {
    let login_user = login_user.into_inner();

    let mut connection = match redis.get_connection() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let user = match User::fetch(&mut connection, &login_user.username) {
        Some(user) => user,
        None => return HttpResponse::NotFound().finish()
    };

    if !user.validate_password(login_user.password.as_str()) {
        return HttpResponse::Unauthorized().finish();
    }

    match session.insert("username", login_user.username.clone()) {
        Ok(_) => session_response_json(&mut connection, user),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}