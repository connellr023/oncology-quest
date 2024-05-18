use crate::models::{model::Model, user_model::UserModel};
use actix_session::Session;
use actix_web::{web::{Data, Json}, HttpResponse, Responder};
use redis::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct AdminDeleteUserQuery {
    username: String
}

#[derive(Deserialize)]
struct DeleteSelfQuery {
    password: String
}

#[actix_web::delete("/api/user/delete-user")]
pub(super) async fn delete_user(session: Session, redis: Data<Client>, delete_user: Json<AdminDeleteUserQuery>) -> impl Responder {
    let mut connection = match redis.get_connection() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let username = match session.get::<String>("username") {
        Ok(Some(username)) => username,
        _ => return HttpResponse::Unauthorized().finish()
    };

    // Only admins can delete other users.
    if !UserModel::validate_is_admin(&mut connection, username.as_str()) {
        return HttpResponse::Forbidden().finish();
    }

    if UserModel::delete(&mut connection, delete_user.username.as_str()).is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

#[actix_web::delete("/api/user/delete-self")]
pub(super) async fn delete_self(session: Session, redis: Data<Client>, delete_self: Json<DeleteSelfQuery>) -> impl Responder {
    let mut connection = match redis.get_connection() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let username = match session.get::<String>("username") {
        Ok(Some(username)) => username,
        _ => return HttpResponse::Unauthorized().finish()
    };

    let user = match UserModel::fetch(&mut connection, username.as_str()) {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().finish()
    };

    if !user.validate_password(delete_self.password.as_str()) {
        return HttpResponse::Unauthorized().finish();
    }

    if UserModel::delete(&mut connection, username.as_str()).is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}