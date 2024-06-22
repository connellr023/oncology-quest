use crate::{models::entry_structure::{Supertask, Task, Subtask}, utilities::parsable::EntryTitle};
use crate::services::prelude::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateEntryQuery {
    pub entry_id: i32,
    pub title: EntryTitle,
}

#[actix_web::patch("/update")]
pub(super) async fn update_supertask(session: Session, pool: Data<PgPool>, update_entry_query: Json<UpdateEntryQuery>) -> impl Responder {
    if let Err(response) = handle_admin_session_validation(&pool, &session).await {
        return response;
    }

    if Supertask::update_title(&pool, update_entry_query.entry_id, update_entry_query.title.as_str()).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

#[actix_web::patch("/update")]
pub(super) async fn update_task(session: Session, pool: Data<PgPool>, update_entry_query: Json<UpdateEntryQuery>) -> impl Responder {
    if let Err(response) = handle_admin_session_validation(&pool, &session).await {
        return response;
    }

    if Task::update_title(&pool, update_entry_query.entry_id, update_entry_query.title.as_str()).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

#[actix_web::patch("/update")]
pub(super) async fn update_subtask(session: Session, pool: Data<PgPool>, update_entry_query: Json<UpdateEntryQuery>) -> impl Responder {
    if let Err(response) = handle_admin_session_validation(&pool, &session).await {
        return response;
    }

    if Subtask::update_title(&pool, update_entry_query.entry_id, update_entry_query.title.as_str()).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}