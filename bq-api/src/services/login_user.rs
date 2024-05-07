use super::{user_session::handle_session_response, validatable::Validatable};
use crate::{models::{model::Model, user::User}, utilities::{PASSWORD_REGEX, USERNAME_REGEX}};
use actix_web::{web::{Json, Data}, HttpResponse, Responder};
use actix_session::Session;
use regex::Regex;
use serde::Deserialize;
use redis::Client;

#[derive(Deserialize)]
pub struct UserCredentials {
    pub username: String,
    pub password: String
}

impl Validatable for UserCredentials {
    fn is_valid(&self) -> bool {
        let username_pattern = Regex::new(USERNAME_REGEX).unwrap();
        let password_pattern = Regex::new(PASSWORD_REGEX).unwrap();

        username_pattern.is_match(self.username.as_str()) && password_pattern.is_match(self.password.as_str())
    }
}

#[actix_web::post("/api/user/login")]
pub(super) async fn login(session: Session, redis: Data<Client>, login_user: Json<UserCredentials>) -> impl Responder {
    if !login_user.is_valid() {
        return HttpResponse::BadRequest().finish();
    }
    
    let mut connection = match redis.get_connection() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };
    
    let login_user = login_user.into_inner();
    let user = match User::fetch(&mut connection, login_user.username.as_str()) {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().finish()
    };

    if !user.validate_password(login_user.password.as_str()) {
        return HttpResponse::Unauthorized().finish();
    }

    match session.insert("username", login_user.username) {
        Ok(_) => handle_session_response(&mut connection, user),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}