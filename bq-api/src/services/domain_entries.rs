use crate::{auth_user_session, models::{domain::Domain, entry_structure::EntryStructure}};
use actix_web::{web::{Data, Path}, HttpResponse, Responder};
use actix_session::Session;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};

#[actix_web::get("/api/domains/{domain_id}/{entries_cache_timestamp:.*}")]
pub(super) async fn fetch_with_timestamp(session: Session, pool: Data<Pool<Postgres>>, data: Path<(i32, String)>) -> impl Responder {
    auth_user_session!(session);

    let entries_cache_timestamp = data.1.parse::<DateTime<Utc>>().ok();

    match Domain::is_cache_valid(&pool, data.0, entries_cache_timestamp).await {
        Err(_) => return HttpResponse::InternalServerError().finish(),
        Ok(true) => return HttpResponse::NotModified().finish(),
        _ => {}
    }
    
    // If the cache is not valid, then fetch the entry structure from the database.
    match EntryStructure::fetch(&pool, data.0).await {
        Err(_) => HttpResponse::InternalServerError().finish(),
        Ok(entry_structure) => HttpResponse::Ok().json(entry_structure)
    }
}