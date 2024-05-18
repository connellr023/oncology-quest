use crate::models::{model::Model, tasks_model::TasksModel, user_model::UserModel, entries::EntryIndex};
use crate::utilities::parsables::SubtaskTitle;
use actix_web::{web::{Json, Data}, HttpResponse, Responder};
use actix_session::Session;
use redis::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct PushEntryQuery {
    pub title: SubtaskTitle,
    pub index: EntryIndex
}

#[derive(Deserialize)]
struct PopEntryQuery {
    pub index: EntryIndex
}

enum EntryAction {
    Push,
    Pop
}

fn handle_update_structure(session: Session, redis: Data<Client>, action: EntryAction, index: &EntryIndex, title: Option<SubtaskTitle>) -> HttpResponse {
    let username = match session.get::<String>("username") {
        Ok(Some(username)) => username,
        _ => return HttpResponse::Unauthorized().finish()
    };
    
    let mut connection = match redis.get_connection() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    // Only admins can push/pop entries.
    if !UserModel::validate_is_admin(&mut connection, username.as_str()) {
        return HttpResponse::Forbidden().finish();
    };

    let mut task_structure = match TasksModel::fetch(&mut connection, "") {
        Ok(task_structure) => task_structure,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    match action {
        EntryAction::Push => {
            if task_structure.push_entry(&mut connection, index, title.unwrap()).is_err() {
                return HttpResponse::InternalServerError().finish();
            }

            HttpResponse::Created().finish()
        },
        EntryAction::Pop => {
            if task_structure.pop_entry(&mut connection, index).is_err() {
                return HttpResponse::InternalServerError().finish();
            }

            HttpResponse::NoContent().finish()
        }
    }
}

#[actix_web::patch("/api/entries/update/push")]
pub(super) async fn push(session: Session, redis: Data<Client>, push_entry: Json<PushEntryQuery>) -> impl Responder {
    let push_entry = push_entry.into_inner();
    handle_update_structure(session, redis, EntryAction::Push, &push_entry.index, Some(push_entry.title))
}

#[actix_web::delete("/api/entries/update/pop")]
pub(super) async fn pop(session: Session, redis: Data<Client>, pop_entry: Json<PopEntryQuery>) -> impl Responder {
    handle_update_structure(session, redis, EntryAction::Pop, &pop_entry.index, None)
}