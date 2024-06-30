use crate::models::entry_structure::{Supertask, Task, Subtask};
use crate::entry_wrapper;
use crate::services::prelude::*;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeleteEntryQuery {
    pub entry_id: i32
}

#[actix_web::delete("/delete")]
pub(super) async fn delete_supertask(claim: JwtUserClaim, pool: Data<PgPool>, delete_entry_query: Json<DeleteEntryQuery>) -> impl Responder {
    entry_wrapper! {pool, claim, {
        if Supertask::delete(&pool, delete_entry_query.entry_id).await.is_err() {
            return HttpResponse::InternalServerError().finish();
        }
    }}
}

#[actix_web::delete("/delete")]
pub(super) async fn delete_task(claim: JwtUserClaim, pool: Data<PgPool>, delete_entry_query: Json<DeleteEntryQuery>) -> impl Responder {
    entry_wrapper! {pool, claim, {
        if Task::delete(&pool, delete_entry_query.entry_id).await.is_err() {
            return HttpResponse::InternalServerError().finish();
        }
    }}
}

#[actix_web::delete("/delete")]
pub(super) async fn delete_subtask(claim: JwtUserClaim, pool: Data<PgPool>, delete_entry_query: Json<DeleteEntryQuery>) -> impl Responder {
    entry_wrapper! {pool, claim, {
        if Subtask::delete(&pool, delete_entry_query.entry_id).await.is_err() {
            return HttpResponse::InternalServerError().finish();
        }
    }}
}