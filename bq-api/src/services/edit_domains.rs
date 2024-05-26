use crate::auth_admin_session;
use crate::models::domain::Domain;
use crate::utilities::parsable::Name;
use actix_session::Session;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::{Pool, Postgres};

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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeleteDomainQuery {
    domain_id: i32
}

#[actix_web::post("/api/domains/create")]
pub(super) async fn create_domain(session: Session, pool: Data<Pool<Postgres>>, create_domain_query: Json<CreateDomainQuery>) -> impl Responder {
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

#[actix_web::delete("/api/domains/delete")]
pub(super) async fn delete_domain(session: Session, pool: Data<Pool<Postgres>>, delete_domain_query: Json<DeleteDomainQuery>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    if Domain::delete(&pool, delete_domain_query.domain_id).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}