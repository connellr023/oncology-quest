use crate::auth_admin_session;
use crate::models::domain::Domain;
use actix_session::Session;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeleteDomainQuery {
    domain_id: i32
}

#[actix_web::delete("/delete")]
pub(super) async fn delete_domain(session: Session, pool: Data<PgPool>, delete_domain_query: Json<DeleteDomainQuery>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    if Domain::delete(&pool, delete_domain_query.domain_id).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}