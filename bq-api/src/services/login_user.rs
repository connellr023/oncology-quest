use super::user_session::handle_session_response;
use crate::models::{model::Model, user::User};
use crate::utilities::parsable::{Parsable, Username, PlainTextPassword};
use actix_web::{web::{Json, Data}, HttpResponse, Responder};
use actix_session::Session;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use redis::Client;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginUserQuery {
    pub username: Username,
    pub password: PlainTextPassword,
    pub structure_cache_timestamp: Option<DateTime<Utc>>
}

#[actix_web::post("/api/user/login")]
pub(super) async fn login(session: Session, redis: Data<Client>, login_user: Json<LoginUserQuery>) -> impl Responder {
    let mut connection = match redis.get_connection() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let login_user = login_user.into_inner();
    let user = match User::fetch(&mut connection, login_user.username.as_str()) {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().finish()
    };

    if !user.validate_password(login_user.password.as_str()) {
        return HttpResponse::Unauthorized().finish();
    }

    match session.insert("username", login_user.username.as_str()) {
        Ok(_) => handle_session_response(&mut connection, login_user.structure_cache_timestamp, user),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}