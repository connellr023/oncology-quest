use crate::models::{model::Model, task_structure::TaskStructure, user::User};
use crate::utilities::parsables::EntryTitle;
use actix_web::{web::{Json, Data}, HttpResponse, Responder};
use actix_session::Session;
use redis::Client;
use serde::Deserialize;

const MAX_ENTRY_DEPTH: usize = 2;

#[derive(Deserialize)]
struct PushEntryQuery {
    pub title: EntryTitle,
    pub index: Vec<u16>
}

#[derive(Deserialize)]
struct PopEntryQuery {
    pub index: Vec<u16>
}

enum EntryAction {
    Push,
    Pop
}

fn handle_update_structure(session: Session, redis: Data<Client>, action: EntryAction, index: &[u16], title: Option<EntryTitle>) -> HttpResponse {
    let username = match session.get::<String>("username") {
        Ok(Some(username)) => username,
        _ => return HttpResponse::Unauthorized().finish()
    };
    
    let mut connection = match redis.get_connection() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    // Only admins can push/pop entries.
    if !User::validate_is_admin(&mut connection, username.as_str()) {
        return HttpResponse::Forbidden().finish();
    };

    let mut task_structure = match TaskStructure::fetch(&mut connection, "") {
        Some(task_structure) => task_structure,
        None => return HttpResponse::InternalServerError().finish()
    };

    match action {
        EntryAction::Push => {
            if !task_structure.push_entry(&mut connection, index, title.unwrap()) {
                return HttpResponse::InternalServerError().finish();
            }

            HttpResponse::Created().finish()
        },
        EntryAction::Pop => {
            if !task_structure.pop_entry(&mut connection, index) {
                return HttpResponse::InternalServerError().finish();
            }

            HttpResponse::NoContent().finish()
        }
    }
}

#[actix_web::patch("/api/entries/update/push")]
pub(super) async fn push(session: Session, redis: Data<Client>, push_entry: Json<PushEntryQuery>) -> impl Responder {
    if !push_entry.index.len() <= MAX_ENTRY_DEPTH {
        return HttpResponse::BadRequest().finish();
    }
    
    handle_update_structure(session, redis, EntryAction::Push, push_entry.index.as_slice(), Some(push_entry.title.clone()))
}

#[actix_web::delete("/api/entries/update/pop")]
pub(super) async fn pop(session: Session, redis: Data<Client>, pop_entry: Json<PopEntryQuery>) -> impl Responder {
    if !pop_entry.index.len() <= MAX_ENTRY_DEPTH {
        return HttpResponse::BadRequest().finish();
    }
    
    handle_update_structure(session, redis, EntryAction::Pop, pop_entry.index.as_slice(), None)
}