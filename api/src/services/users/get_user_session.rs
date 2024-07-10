use crate::services::prelude::*;

#[actix_web::get("/session")]
pub(super) async fn get_user_session(claim: JwtUserClaim, memory_cache: Data<MemoryCache>, pool: Data<PgPool>) -> impl Responder {
    UserSession::respond(&pool, &memory_cache, claim.sub, None).await
}
