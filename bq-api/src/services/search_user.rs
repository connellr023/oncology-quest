use crate::auth_admin;
use crate::models::{user::User, client_user::ClientUser};
use actix_web::{web::{Data, Path}, HttpResponse, Responder};
use actix_session::Session;
use serde::Deserialize;
use sqlx::{Pool, Postgres};

#[derive(Deserialize)]
pub struct UserSearchQuery {
    pub query: String
}

#[actix_web::get("/api/user/search/{query}")]
pub(super) async fn search(session: Session, search: Path<UserSearchQuery>, pool: Data<Pool<Postgres>>) -> impl Responder {
    auth_admin!(user_id, session, pool);

    let users = match User::text_search(&pool, search.query.as_str()).await {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let users = users
        .into_iter()
        .map(|user| ClientUser::from((*user).to_owned()))
        .collect::<Box<[ClientUser]>>();

    HttpResponse::Ok().json(users)
}