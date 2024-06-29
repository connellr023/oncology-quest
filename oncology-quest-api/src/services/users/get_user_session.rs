use crate::services::prelude::*;

#[actix_web::get("/session")]
pub(super) async fn get_user_session(claim: JwtUserClaim, pool: Data<PgPool>) -> impl Responder {
    UserSession::respond(&pool, claim.sub).await
}