use crate::{auth_user, models::entry_structure::EntryStructure};
use actix_web::{web::{Data, Json, Path}, HttpResponse, Responder};
use actix_session::Session;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FetchDomainEntriesQuery {
    pub domain_id: i32,
    pub cache_timestamp: Option<DateTime<Utc>>
}

#[actix_web::get("/api/domains/{domain_id}/{cache_timestamp}/entries")]
pub(super) async fn fetch(session: Session, pool: Data<Pool<Postgres>>, fetch_domain_entries_query: Path<FetchDomainEntriesQuery>) -> impl Responder {
    auth_user!(user_id, session);

    // TODO: Add validation for the cache timestamp

    if EntryStructure::fetch(&pool, fetch_domain_entries_query.domain_id).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    
    HttpResponse::Ok().finish()
}
