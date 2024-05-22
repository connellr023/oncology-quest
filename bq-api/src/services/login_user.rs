use super::user_session::UserSessionResponse;
use crate::models::user::User;
use crate::utilities::parsable::{Username, PlainTextPassword};
use actix_web::{web::{Json, Data}, HttpResponse, Responder};
use sqlx::{Pool, Postgres};
use actix_session::Session;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginUserQuery {
    pub username: Username,
    pub password: PlainTextPassword
}

#[actix_web::post("/api/user/login")]
pub(super) async fn login(session: Session, pool: Data<Pool<Postgres>>, login_user_query: Json<LoginUserQuery>) -> impl Responder {
    let user = match User::fetch_by_username(&pool, login_user_query.username.as_str()).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().finish()
    };

    if !user.validate_password(login_user_query.password.as_str()) {
        return HttpResponse::Unauthorized().finish();
    }

    if session.insert("uid", user.id()).is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    match UserSessionResponse::build_from_user(&pool, user).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}