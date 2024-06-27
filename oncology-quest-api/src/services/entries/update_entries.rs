use crate::{models::entry_structure::{Supertask, Task, Subtask}, utilities::parsable::EntryTitle};
use crate::services::prelude::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateEntryQuery {
    pub entry_id: i32,
    pub title: EntryTitle,
}

macro_rules! update_entry_wrapper {
    ($pool:ident, $session:ident, $block:block) => {
        if let Err(response) = UserSession::validate(&$pool, &$session, UserSessionRole::Admin).await {
            return response;
        }
    
        $block
    
        HttpResponse::Ok().finish()
    };
}

#[actix_web::patch("/update")]
pub(super) async fn update_supertask(session: Session, pool: Data<PgPool>, update_entry_query: Json<UpdateEntryQuery>) -> impl Responder {
    update_entry_wrapper! {pool, session, {
        if Supertask::update_title(&pool, update_entry_query.entry_id, update_entry_query.title.as_str()).await.is_err() {
            return HttpResponse::InternalServerError().finish();
        }
    }}
}

#[actix_web::patch("/update")]
pub(super) async fn update_task(session: Session, pool: Data<PgPool>, update_entry_query: Json<UpdateEntryQuery>) -> impl Responder {
    update_entry_wrapper! {pool, session, {
        if Task::update_title(&pool, update_entry_query.entry_id, update_entry_query.title.as_str()).await.is_err() {
            return HttpResponse::InternalServerError().finish();
        }
    }}
}

#[actix_web::patch("/update")]
pub(super) async fn update_subtask(session: Session, pool: Data<PgPool>, update_entry_query: Json<UpdateEntryQuery>) -> impl Responder {
    update_entry_wrapper! {pool, session, {
        if Subtask::update_title(&pool, update_entry_query.entry_id, update_entry_query.title.as_str()).await.is_err() {
            return HttpResponse::InternalServerError().finish();
        }
    }}
}