use crate::utilities::parsable::{Username, Name, Email, PlainTextPassword};
use crate::models::user::User;
use actix_web::{web::{Json, Data}, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

#[derive(Deserialize)]
struct RegisterUserQuery {
    pub username: Username,
    pub name: Name,
    pub email: Email,
    pub password: PlainTextPassword
}

#[actix_web::post("/api/user/register")]
pub(super) async fn register(pool: Data<Pool<Postgres>>, register_user_query: Json<RegisterUserQuery>) -> impl Responder {
    let register_user_query = register_user_query.into_inner();
    
    let mut user = match User::new(
        register_user_query.username,
        register_user_query.name,
        register_user_query.email,
        register_user_query.password,
        false
    ) {
        Ok(user) => user,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    if user.exists(&pool).await {
        return HttpResponse::Conflict().finish();
    }

    if user.insert(&pool).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Created().finish()
}