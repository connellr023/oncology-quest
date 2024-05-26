use crate::auth_admin_session;
use crate::models::search_result_user::SearchResultUser;
use actix_web::{web::{Data, Path}, HttpResponse, Responder};
use actix_session::Session;
use sqlx::{Pool, Postgres};

const SEARCH_LIMIT: i64 = 10;

#[actix_web::get("/api/user/search/{query}")]
pub(super) async fn search(session: Session, pool: Data<Pool<Postgres>>, query: Path<String>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    let users = match SearchResultUser::text_search(&pool, query.as_str(), SEARCH_LIMIT).await {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    HttpResponse::Ok().json(users)
}