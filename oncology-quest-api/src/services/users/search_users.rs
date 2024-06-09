use crate::models::search_result_user::SearchResultUser;
use crate::services::prelude::*;

const SEARCH_LIMIT: i64 = 10;

#[actix_web::get("/search/{query}")]
pub(super) async fn search_users(session: Session, pool: Data<PgPool>, query: Path<String>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    let users = match SearchResultUser::text_search_as_map(&pool, query.as_str(), SEARCH_LIMIT).await {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    HttpResponse::Ok().json(users)
}