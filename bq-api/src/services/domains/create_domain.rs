use crate::auth_admin_session;
use crate::models::domain::Domain;
use crate::utilities::parsable::Name;
use actix_session::Session;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;

#[derive(Deserialize)]
struct CreateDomainQuery {
    name: Name
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateDomainResponse {
    domain_id: i32,
    last_updated: DateTime<Utc>
}

#[actix_web::post("/create")]
pub(super) async fn create_domain(session: Session, pool: Data<PgPool>, create_domain_query: Json<CreateDomainQuery>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    let mut domain = Domain::new(create_domain_query.into_inner().name);

    if domain.insert(&pool).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().json(CreateDomainResponse {
        domain_id: domain.id(),
        last_updated: domain.last_updated()
    })
}