use crate::{models::entry_structure::{Supertask, Task, Subtask}, utilities::parsable::EntryTitle};
use crate::services::prelude::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateSupertaskEntryQuery {
    pub rotation_id: i32,
    pub title: EntryTitle
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateLowerEntryQuery {
    pub rotation_id: i32,
    pub parent_id: i32,
    pub title: EntryTitle
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateEntryResponse {
    pub entry_id: i32
}

async fn handle_pre_validate(pool: &PgPool, session: &Session, entry_id: i32) -> Result<(), HttpResponse> {
    handle_admin_session_validation(&pool, &session).await?;
    handle_rotation_validation(&pool, entry_id).await?;

    Ok(())
}

#[actix_web::post("/create")]
pub(super) async fn create_supertask(session: Session, pool: Data<PgPool>, create_entry_query: Json<CreateSupertaskEntryQuery>) -> impl Responder {
    if let Err(response) = handle_pre_validate(&pool, &session, create_entry_query.rotation_id).await {
        return response;
    }

    match Supertask::insert_from(&pool, create_entry_query.title.as_str(), create_entry_query.rotation_id).await {
        Ok(entry_id) => HttpResponse::Created().json(CreateEntryResponse { entry_id }),
        Err(err) => {
            eprintln!("{:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::post("/create")]
pub(super) async fn create_task(session: Session, pool: Data<PgPool>, create_entry_query: Json<CreateLowerEntryQuery>) -> impl Responder {
    if let Err(response) = handle_pre_validate(&pool, &session, create_entry_query.rotation_id).await {
        return response;
    }

    match Task::insert_from(&pool, create_entry_query.title.as_str(), create_entry_query.rotation_id, create_entry_query.parent_id).await {
        Ok(entry_id) => HttpResponse::Created().json(CreateEntryResponse { entry_id }),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[actix_web::post("/create")]
pub(super) async fn create_subtask(session: Session, pool: Data<PgPool>, create_entry_query: Json<CreateLowerEntryQuery>) -> impl Responder {
    if let Err(response) = handle_pre_validate(&pool, &session, create_entry_query.rotation_id).await {
        return response;
    }

    match Subtask::insert_from(&pool, create_entry_query.title.as_str(), create_entry_query.rotation_id, create_entry_query.parent_id).await {
        Ok(entry_id) => HttpResponse::Created().json(CreateEntryResponse { entry_id }),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}