use crate::{auth_user_session, models::{domain::Domain, entry_structure::EntryStructure}};
use actix_web::{web::{Data, Path, Query}, HttpResponse, Responder};
use actix_session::Session;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FetchEntryStructureQuery {
    pub entries_cache_timestamp: Option<DateTime<Utc>>
}

#[actix_web::get("/api/domains/{domain_id}")]
pub(super) async fn fetch_with_timestamp(session: Session, pool: Data<Pool<Postgres>>, domain_id: Path<i32>, query: Query<FetchEntryStructureQuery>) -> impl Responder {
    auth_user_session!(session);

    match Domain::is_cache_valid(&pool, *domain_id, query.entries_cache_timestamp).await {
        Err(_) => return HttpResponse::InternalServerError().finish(),
        Ok(true) => return HttpResponse::NotModified().finish(),
        _ => {}
    }
    
    // If the cache is not valid, then fetch the entry structure from the database.
    match EntryStructure::fetch(&pool, *domain_id).await {
        Ok(entry_structure) => HttpResponse::Ok().json(entry_structure),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}