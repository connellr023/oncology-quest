use crate::auth_admin_session;
use crate::models::entry_structure::{Supertask, Task, Subtask};
use actix_session::Session;
use actix_web::{web::{Data, Json}, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeleteEntryQuery {
    pub entry_id: i32
}

#[actix_web::delete("/delete")]
pub(super) async fn delete_supertask(session: Session, pool: Data<PgPool>, delete_entry_query: Json<DeleteEntryQuery>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    if Supertask::delete(&pool, delete_entry_query.entry_id).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

#[actix_web::delete("/delete")]
pub(super) async fn delete_task(session: Session, pool: Data<PgPool>, delete_entry_query: Json<DeleteEntryQuery>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    if Task::delete(&pool, delete_entry_query.entry_id).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

#[actix_web::delete("/delete")]
pub(super) async fn delete_subtask(session: Session, pool: Data<PgPool>, delete_entry_query: Json<DeleteEntryQuery>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    if Subtask::delete(&pool, delete_entry_query.entry_id).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}