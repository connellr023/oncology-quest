use crate::{auth_user_session, models::{domain::Domain, entry_structure::EntryStructure}};
use actix_web::{web::{Data, Path}, HttpResponse, Responder};
use actix_session::Session;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FetchDomainEntriesQuery {
    pub domain_id: i32,
    pub entries_cache_timestamp: Option<DateTime<Utc>>
}

#[actix_web::get("/api/domains/{domain_id}/{entries_cache_timestamp}")]
pub(super) async fn fetch(session: Session, pool: Data<Pool<Postgres>>, fetch_domain_entries_query: Path<FetchDomainEntriesQuery>) -> impl Responder {
    auth_user_session!(session);

    match Domain::is_cache_valid(&pool, fetch_domain_entries_query.domain_id, fetch_domain_entries_query.entries_cache_timestamp).await {
        Err(_) => return HttpResponse::InternalServerError().finish(),
        Ok(true) => return HttpResponse::NotModified().finish(),
        _ => {}
    }
    
    // If the cache is not valid, then fetch the entry structure from the database.
    match EntryStructure::fetch(&pool, fetch_domain_entries_query.domain_id).await {
        Err(_) => HttpResponse::InternalServerError().finish(),
        Ok(entry_structure) => HttpResponse::Ok().json(entry_structure)
    }
}
