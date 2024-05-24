use crate::auth_admin_session;
use crate::models::{user::User, client_user::ClientUser};
use actix_web::{web::{Data, Path}, HttpResponse, Responder};
use actix_session::Session;
use sqlx::{Pool, Postgres};

#[actix_web::get("/api/user/search/{query}")]
pub(super) async fn search(session: Session, pool: Data<Pool<Postgres>>, query: Path<String>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    let users = match User::text_search(&pool, query.as_str()).await {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let users = users
        .into_iter()
        .map(|user| ClientUser::from((*user).to_owned()))
        .collect::<Box<[ClientUser]>>();

    HttpResponse::Ok().json(users)
}