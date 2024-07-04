use actix_web::{HttpResponse, Responder};

#[actix_web::get("/healthcheck")]
pub(super) async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().message_body("OK")
}