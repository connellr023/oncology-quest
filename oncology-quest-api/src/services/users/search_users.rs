use crate::models::client_user::ClientUser;
use crate::services::prelude::*;

const SEARCH_LIMIT: i64 = 10;

#[actix_web::get("/search/{query}")]
pub(super) async fn search_users(claim: JwtUserClaim, pool: Data<PgPool>, query: Path<String>) -> impl Responder {
    if !claim.sub.is_admin {
        return HttpResponse::Unauthorized().finish();
    }

    let users = match ClientUser::text_search_as_map(&pool, query.as_str(), SEARCH_LIMIT).await {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    HttpResponse::Ok().json(users)
}