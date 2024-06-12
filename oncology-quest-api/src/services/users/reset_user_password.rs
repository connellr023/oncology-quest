use crate::models::user::User;
use crate::utilities::parsable::{ResetToken, Username, PlainTextPassword};
use crate::services::prelude::*;

const PASSWORD_RESET_EXPIRATION_HOURS: i32 = 2;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResetPasswordQuery {
    pub username: Username,
    pub password: PlainTextPassword,
    pub reset_token: ResetToken
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AllowResetPasswordQuery {
    user_id: i32
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AllowResetPasswordResponse {
    pub password_reset_timestamp: DateTime<Utc>,
    pub reset_token: ResetToken
}

#[actix_web::post("/reset-password")]
pub(super) async fn reset_password(pool: Data<PgPool>, reset_password_query: Json<ResetPasswordQuery>) -> impl Responder {
    match User::update_password(&pool, reset_password_query.username.as_str(), reset_password_query.password.as_str(), reset_password_query.reset_token.as_str()).await {
        Ok(success) => match success {
            true => HttpResponse::Ok().finish(),
            false => HttpResponse::Forbidden().finish()
        },
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[actix_web::patch("/allow-reset-password")]
pub(super) async fn allow_reset_password(session: Session, pool: Data<PgPool>, allow_reset_password_query: Json<AllowResetPasswordQuery>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    match User::allow_reset_password(&pool, allow_reset_password_query.user_id, PASSWORD_RESET_EXPIRATION_HOURS).await {
        Err(_) => HttpResponse::Forbidden().finish(),
        Ok((password_reset_timestamp, reset_token)) => HttpResponse::Ok().json(
            AllowResetPasswordResponse {
                password_reset_timestamp,
                reset_token
            }
        )
    }
}