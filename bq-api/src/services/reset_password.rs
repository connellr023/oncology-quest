use crate::{models::{model::Model, user::User}, utilities::parsables::PlainTextPassword};
use crate::utilities::parsables::{Parsable, Username};
use actix_session::Session;
use actix_web::{web::{Data, Json}, HttpResponse, Responder};
use redis::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct ResetPasswordQuery {
    pub username: Username,
    pub password: PlainTextPassword
}

#[actix_web::post("/api/user/reset-password")]
pub(super) async fn reset(redis: Data<Client>, reset_password: Json<ResetPasswordQuery>) -> impl Responder {
    let mut connection = match redis.get_connection() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let mut user = match User::fetch(&mut connection, reset_password.username.as_str()) {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().finish()
    };

    // Only users that can reset their password can reset their password.
    if !user.can_reset_password {
        return HttpResponse::Forbidden().finish();
    }

    let new_password = match User::gen_password_hash(user.salt, reset_password.password.as_str()) {
        Some(new_password) => new_password,
        None => return HttpResponse::InternalServerError().finish()
    };

    user.password = new_password;

    if user.store(&mut connection).is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
struct AllowResetPasswordQuery {
    username: Username,
    allow_reset: bool
}

#[actix_web::patch("/api/user/allow-reset-password")]
pub(super) async fn allow_reset(session: Session, redis: Data<Client>, allow_reset_password: Json<AllowResetPasswordQuery>) -> impl Responder {
    let mut connection = match redis.get_connection() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let username = match session.get::<String>("username") {
        Ok(Some(username)) => username,
        _ => return HttpResponse::Unauthorized().finish()
    };

    // Only admins can allow users to reset their password.
    if !User::validate_is_admin(&mut connection, username.as_str()) {
        return HttpResponse::Forbidden().finish();
    }

    let mut target_user = match User::fetch(&mut connection, allow_reset_password.username.as_str()) {
        Ok(target_user) => target_user,
        Err(_) => return HttpResponse::NotFound().finish()
    };

    // Admins cannot allow themselves to reset their password.
    if target_user.is_admin {
        return HttpResponse::Forbidden().finish();
    }

    target_user.can_reset_password = allow_reset_password.allow_reset;

    if target_user.store(&mut connection).is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    
    HttpResponse::Ok().finish()
}