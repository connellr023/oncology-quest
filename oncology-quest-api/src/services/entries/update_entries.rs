use crate::{models::entry_structure::{Supertask, Task, Subtask}, utilities::parsable::EntryTitle};
use crate::entry_wrapper;
use crate::services::prelude::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateEntryQuery {
    pub entry_id: i32,
    pub title: EntryTitle,
}

#[actix_web::patch("/update")]
pub(super) async fn update_supertask(claim: JwtUserClaim, pool: Data<PgPool>, update_entry_query: Json<UpdateEntryQuery>) -> impl Responder {
    entry_wrapper! {pool, claim, {
        if Supertask::update_title(&pool, update_entry_query.entry_id, update_entry_query.title.as_str()).await.is_err() {
            return HttpResponse::InternalServerError().finish();
        }
    }}
}

#[actix_web::patch("/update")]
pub(super) async fn update_task(claim: JwtUserClaim, pool: Data<PgPool>, update_entry_query: Json<UpdateEntryQuery>) -> impl Responder {
    entry_wrapper! {pool, claim, {
        if Task::update_title(&pool, update_entry_query.entry_id, update_entry_query.title.as_str()).await.is_err() {
            return HttpResponse::InternalServerError().finish();
        }
    }}
}

#[actix_web::patch("/update")]
pub(super) async fn update_subtask(claim: JwtUserClaim, pool: Data<PgPool>, update_entry_query: Json<UpdateEntryQuery>) -> impl Responder {
    entry_wrapper! {pool, claim, {
        if Subtask::update_title(&pool, update_entry_query.entry_id, update_entry_query.title.as_str()).await.is_err() {
            return HttpResponse::InternalServerError().finish();
        }
    }}
}