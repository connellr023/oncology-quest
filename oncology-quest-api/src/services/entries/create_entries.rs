use crate::auth_admin_session;
use crate::{models::entry_structure::{Supertask, Task, Subtask}, utilities::parsable::EntryTitle};
use crate::models::rotation::Rotation;
use actix_session::Session;
use actix_web::{web::{Data, Json}, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

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

macro_rules! validate_session_and_entry_id {
    ($varname:ident, $session:ident, $pool:ident, $entry_id:expr) => {
        auth_admin_session!(user_id, $session, $pool);

        if !Rotation::exists(&$pool, $entry_id).await {
            return HttpResponse::NoContent().finish();
        }
    };
}

#[actix_web::post("/create")]
pub(super) async fn create_supertask(session: Session, pool: Data<Pool<Postgres>>, create_entry_query: Json<CreateSupertaskEntryQuery>) -> impl Responder {
    validate_session_and_entry_id!(rotation, session, pool, create_entry_query.rotation_id);

    match Supertask::insert_from(&pool, create_entry_query.title.as_str(), create_entry_query.rotation_id).await {
        Ok(entry_id) => HttpResponse::Ok().json(CreateEntryResponse { entry_id }),
        Err(err) => {
            eprintln!("{:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::post("/create")]
pub(super) async fn create_task(session: Session, pool: Data<Pool<Postgres>>, create_entry_query: Json<CreateLowerEntryQuery>) -> impl Responder {
    validate_session_and_entry_id!(rotation, session, pool, create_entry_query.rotation_id);

    match Task::insert_from(&pool, create_entry_query.title.as_str(), create_entry_query.rotation_id, create_entry_query.parent_id).await {
        Ok(entry_id) => HttpResponse::Ok().json(CreateEntryResponse { entry_id }),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[actix_web::post("/create")]
pub(super) async fn create_subtask(session: Session, pool: Data<Pool<Postgres>>, create_entry_query: Json<CreateLowerEntryQuery>) -> impl Responder {
    validate_session_and_entry_id!(rotation, session, pool, create_entry_query.rotation_id);

    match Subtask::insert_from(&pool, create_entry_query.title.as_str(), create_entry_query.rotation_id, create_entry_query.parent_id).await {
        Ok(entry_id) => HttpResponse::Ok().json(CreateEntryResponse { entry_id }),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}