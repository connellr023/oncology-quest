use actix_web::{HttpResponse, Responder};
use actix_session::Session;

#[actix_web::post("/logout")]
pub(super) async fn logout(session: Session) -> impl Responder {
    session.remove("uid");
    HttpResponse::Ok().finish()
}