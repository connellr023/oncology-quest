use crate::auth_admin_session;
use crate::{models::user::User, utilities::parsable::PlainTextPassword};
use crate::utilities::parsable::Username;
use actix_session::Session;
use actix_web::{web::{Data, Json}, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

#[derive(Deserialize)]
struct ResetPasswordQuery {
    pub username: Username,
    pub password: PlainTextPassword
}

#[actix_web::post("/api/user/reset-password")]
pub(super) async fn reset(pool: Data<Pool<Postgres>>, reset_password_query: Json<ResetPasswordQuery>) -> impl Responder {
    match User::update_password(&pool, reset_password_query.username.as_str(), reset_password_query.password.as_str()).await.is_err() {
        true => HttpResponse::Forbidden().finish(),
        false => HttpResponse::Ok().finish()
    }
}

#[derive(Deserialize)]
struct AllowResetPasswordQuery {
    user_id: i32,
    allow_reset: bool
}

#[actix_web::patch("/api/user/allow-reset-password")]
pub(super) async fn allow_reset(session: Session, pool: Data<Pool<Postgres>>, allow_reset_password_query: Json<AllowResetPasswordQuery>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    match User::update_allow_reset_password(&pool, allow_reset_password_query.user_id, allow_reset_password_query.allow_reset).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::Forbidden().finish()
    }
}