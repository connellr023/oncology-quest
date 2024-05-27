use crate::auth_admin_session;
use crate::{models::entry_structure::{Supertask, Task, Subtask}, utilities::parsable::EntryTitle};
use crate::models::domain::Domain;
use actix_session::Session;
use actix_web::{web::{Data, Json}, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateSupertaskEntryQuery {
    pub domain_id: i32,
    pub title: EntryTitle
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateLowerEntryQuery {
    pub domain_id: i32,
    pub parent_id: i32,
    pub title: EntryTitle
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateEntryQuery {
    pub entry_id: i32,
    pub title: EntryTitle,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EntryIdField {
    pub entry_id: i32
}

macro_rules! validate_session_and_entry_id {
    ($varname:ident, $session:ident, $pool:ident, $entry_id:expr) => {
        auth_admin_session!(user_id, $session, $pool);

        if !Domain::exists(&$pool, $entry_id).await {
            return HttpResponse::BadRequest().finish();
        }
    };
}

#[actix_web::post("/api/supertasks/create")]
pub(super) async fn create_supertask(session: Session, pool: Data<Pool<Postgres>>, create_entry_query: Json<CreateSupertaskEntryQuery>) -> impl Responder {
    validate_session_and_entry_id!(domain, session, pool, create_entry_query.domain_id);

    match Supertask::insert_from(&pool, create_entry_query.title.as_str(), create_entry_query.domain_id).await {
        Ok(entry_id) => HttpResponse::Ok().json(EntryIdField { entry_id }),
        Err(err) => {
            eprintln!("{:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::patch("/api/supertasks/update")]
pub(super) async fn update_supertask(session: Session, pool: Data<Pool<Postgres>>, update_entry_query: Json<UpdateEntryQuery>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    if Supertask::update_title(&pool, update_entry_query.entry_id, update_entry_query.title.as_str()).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

#[actix_web::delete("/api/supertasks/delete")]
pub(super) async fn delete_supertask(session: Session, pool: Data<Pool<Postgres>>, delete_entry_query: Json<EntryIdField>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    if Supertask::delete(&pool, delete_entry_query.entry_id).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

#[actix_web::post("/api/tasks/create")]
pub(super) async fn create_task(session: Session, pool: Data<Pool<Postgres>>, create_entry_query: Json<CreateLowerEntryQuery>) -> impl Responder {
    validate_session_and_entry_id!(domain, session, pool, create_entry_query.domain_id);

    match Task::insert_from(&pool, create_entry_query.title.as_str(), create_entry_query.domain_id, create_entry_query.parent_id).await {
        Ok(entry_id) => HttpResponse::Ok().json(EntryIdField { entry_id }),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[actix_web::patch("/api/tasks/update")]
pub(super) async fn update_task(session: Session, pool: Data<Pool<Postgres>>, update_entry_query: Json<UpdateEntryQuery>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    if Task::update_title(&pool, update_entry_query.entry_id, update_entry_query.title.as_str()).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

#[actix_web::delete("/api/tasks/delete")]
pub(super) async fn delete_task(session: Session, pool: Data<Pool<Postgres>>, delete_entry_query: Json<EntryIdField>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    if Task::delete(&pool, delete_entry_query.entry_id).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

#[actix_web::post("/api/subtasks/create")]
pub(super) async fn create_subtask(session: Session, pool: Data<Pool<Postgres>>, create_entry_query: Json<CreateLowerEntryQuery>) -> impl Responder {
    validate_session_and_entry_id!(domain, session, pool, create_entry_query.domain_id);

    match Subtask::insert_from(&pool, create_entry_query.title.as_str(), create_entry_query.domain_id, create_entry_query.parent_id).await {
        Ok(entry_id) => HttpResponse::Ok().json(EntryIdField { entry_id }),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[actix_web::patch("/api/subtasks/update")]
pub(super) async fn update_subtask(session: Session, pool: Data<Pool<Postgres>>, update_entry_query: Json<UpdateEntryQuery>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    if Subtask::update_title(&pool, update_entry_query.entry_id, update_entry_query.title.as_str()).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

#[actix_web::delete("/api/subtasks/delete")]
pub(super) async fn delete_subtask(session: Session, pool: Data<Pool<Postgres>>, delete_entry_query: Json<EntryIdField>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    if Subtask::delete(&pool, delete_entry_query.entry_id).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}