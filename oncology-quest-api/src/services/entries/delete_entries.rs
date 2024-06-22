use crate::models::entry_structure::{Supertask, Task, Subtask};
use crate::services::prelude::*;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeleteEntryQuery {
    pub entry_id: i32
}

#[actix_web::delete("/delete")]
pub(super) async fn delete_supertask(session: Session, pool: Data<PgPool>, delete_entry_query: Json<DeleteEntryQuery>) -> impl Responder {
    if let Err(response) = UserSession::validate(&pool, &session, UserSessionRole::Admin).await {
        return response;
    }

    if Supertask::delete(&pool, delete_entry_query.entry_id).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

#[actix_web::delete("/delete")]
pub(super) async fn delete_task(session: Session, pool: Data<PgPool>, delete_entry_query: Json<DeleteEntryQuery>) -> impl Responder {
    if let Err(response) = UserSession::validate(&pool, &session, UserSessionRole::Admin).await {
        return response;
    }

    if Task::delete(&pool, delete_entry_query.entry_id).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

#[actix_web::delete("/delete")]
pub(super) async fn delete_subtask(session: Session, pool: Data<PgPool>, delete_entry_query: Json<DeleteEntryQuery>) -> impl Responder {
    if let Err(response) = UserSession::validate(&pool, &session, UserSessionRole::Admin).await {
        return response;
    }

    if Subtask::delete(&pool, delete_entry_query.entry_id).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}