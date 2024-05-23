use crate::auth_admin_session;
use crate::{models::{entry_structure::{Supertask, Task, Subtask}, user::User}, utilities::parsable::EntryTitle};
use actix_session::Session;
use actix_web::{web::{Data, Json}, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateEntryQuery {
    pub domain_id: i32,
    pub parent_id: Option<i32>,
    pub title: EntryTitle
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateEntryQuery {
    pub entry_id: i32,
    pub title: EntryTitle,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeleteEntryQuery {
    pub entry_id: i32
}

#[actix_web::post("/api/supertasks/create")]
pub(super) async fn create_supertask(session: Session, pool: Data<Pool<Postgres>>, create_entry_query: Json<CreateEntryQuery>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    // If the parent_id is not None, then the entry is not a supertask.
    if create_entry_query.parent_id.is_some() {
        return HttpResponse::BadRequest().finish();
    }

    if Supertask::insert(&pool, create_entry_query.title.as_str(), create_entry_query.domain_id).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
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
pub(super) async fn delete_supertask(session: Session, pool: Data<Pool<Postgres>>, delete_entry_query: Json<DeleteEntryQuery>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    if Supertask::delete(&pool, delete_entry_query.entry_id).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

#[actix_web::post("/api/tasks/create")]
pub(super) async fn create_task(session: Session, pool: Data<Pool<Postgres>>, create_entry_query: Json<CreateEntryQuery>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    // If the parent_id is None, then the entry is not a task.
    let supertask_id = match create_entry_query.parent_id {
        Some(supertask_id) => supertask_id,
        None => return HttpResponse::BadRequest().finish()
    };

    if Task::insert(&pool, create_entry_query.title.as_str(), create_entry_query.domain_id, supertask_id).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
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
pub(super) async fn delete_task(session: Session, pool: Data<Pool<Postgres>>, delete_entry_query: Json<DeleteEntryQuery>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    if Task::delete(&pool, delete_entry_query.entry_id).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

#[actix_web::post("/api/subtasks/create")]
pub(super) async fn create_subtask(session: Session, pool: Data<Pool<Postgres>>, create_entry_query: Json<CreateEntryQuery>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    // If the parent_id is None, then the entry is not a subtask.
    let task_id = match create_entry_query.parent_id {
        Some(task_id) => task_id,
        None => return HttpResponse::BadRequest().finish()
    };

    if Subtask::insert(&pool, create_entry_query.title.as_str(), create_entry_query.domain_id, task_id).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
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
pub(super) async fn delete_subtask(session: Session, pool: Data<Pool<Postgres>>, delete_entry_query: Json<DeleteEntryQuery>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    if Subtask::delete(&pool, delete_entry_query.entry_id).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}