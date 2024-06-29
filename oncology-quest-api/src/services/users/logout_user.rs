use crate::services::prelude::*;

#[actix_web::post("/logout")]
pub(super) async fn logout_user(claim: JwtUserClaim) -> impl Responder {
    // TODO: Implement logout_user
    HttpResponse::Ok().finish()
}