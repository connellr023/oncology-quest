use crate::models::{rotation::Rotation, entry_structure::EntryStructure};
use crate::services::prelude::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetEntriesQuery {
    pub entries_cache_timestamp: Option<DateTime<Utc>>
}

#[actix_web::get("/{rotation_id}")]
pub(super) async fn get_entries(session: Session, pool: Data<PgPool>, rotation_id: Path<i32>, query: Query<GetEntriesQuery>) -> impl Responder {
    if let Err(response) = UserSession::validate(&pool, &session, UserSessionRole::Any).await {
        return response;
    }

    match Rotation::is_cache_valid(&pool, *rotation_id, query.entries_cache_timestamp).await {
        Err(_) => return HttpResponse::InternalServerError().finish(),
        Ok(true) => return HttpResponse::NotModified().finish(),
        _ => {}
    }
    
    // If the cache is not valid, then fetch the entry structure from the database.
    match EntryStructure::fetch(&pool, *rotation_id).await {
        Ok(entry_structure) => HttpResponse::Ok().json(entry_structure),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}