use crate::auth_admin_session;
use crate::{models::user::User, utilities::parsable::PlainTextPassword};
use crate::utilities::parsable::Username;
use actix_session::Session;
use actix_web::{web::{Data, Json}, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

const PASSWORD_RESET_EXPIRATION_HOURS: i32 = 2;

#[derive(Deserialize)]
struct ResetPasswordQuery {
    pub username: Username,
    pub password: PlainTextPassword
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AllowResetPasswordQuery {
    user_id: i32
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AllowResetPasswordResponse {
    pub password_reset_timestamp: DateTime<Utc>
}

#[actix_web::post("/reset-password")]
pub(super) async fn reset_password(pool: Data<PgPool>, reset_password_query: Json<ResetPasswordQuery>) -> impl Responder {
    match User::update_password(&pool, reset_password_query.username.as_str(), reset_password_query.password.as_str()).await.is_err() {
        true => HttpResponse::Forbidden().finish(),
        false => HttpResponse::Ok().finish()
    }
}

#[actix_web::patch("/allow-reset-password")]
pub(super) async fn allow_reset_password(session: Session, pool: Data<PgPool>, allow_reset_password_query: Json<AllowResetPasswordQuery>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    match User::allow_reset_password(&pool, allow_reset_password_query.user_id, PASSWORD_RESET_EXPIRATION_HOURS).await {
        Err(_) => HttpResponse::Forbidden().finish(),
        Ok(password_reset_timestamp) => HttpResponse::Ok().json(
            AllowResetPasswordResponse {
                password_reset_timestamp
            }
        )
    }
}