use super::validatable::Validatable;
use crate::{models::{model::Model, task_structure::TaskStructure, user::User}, utilities::ENTRY_TITLE_REGEX};
use actix_web::{web::{Json, Data}, HttpResponse, Responder};
use actix_session::Session;
use redis::Client;
use serde::Deserialize;
use regex::Regex;

const MAX_ENTRY_DEPTH: usize = 2;

#[derive(Deserialize)]
struct PushEntry {
    title: String,
    index: Vec<u16>
}

#[derive(Deserialize)]
struct PopEntry {
    index: Vec<u16>
}

impl Validatable for PushEntry {
    fn validate(&self) -> bool {
        let title_pattern = Regex::new(ENTRY_TITLE_REGEX).unwrap();
        title_pattern.is_match(&self.title) && self.index.len() <= MAX_ENTRY_DEPTH
    }
}

impl Validatable for PopEntry {
    fn validate(&self) -> bool {
        self.index.len() <= MAX_ENTRY_DEPTH
    }
}

enum EntryAction {
    Push,
    Pop
}

fn handle_update_structure(session: Session, redis: Data<Client>, action: EntryAction, index: &[u16], title: Option<&str>) -> HttpResponse {
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

            return HttpResponse::Created().finish();
        },
        EntryAction::Pop => {
            if !task_structure.pop_entry(&mut connection, index) {
                return HttpResponse::InternalServerError().finish();
            }

            return HttpResponse::NoContent().finish();
        }
    }
}

#[actix_web::patch("/api/entries/update/push")]
pub(super) async fn push(session: Session, redis: Data<Client>, push_entry: Json<PushEntry>) -> impl Responder {
    if !push_entry.validate() {
        return HttpResponse::BadRequest().finish();
    }
    
    handle_update_structure(session, redis, EntryAction::Push, push_entry.index.as_slice(), Some(push_entry.title.as_str()))
}

#[actix_web::delete("/api/entries/update/pop")]
pub(super) async fn pop(session: Session, redis: Data<Client>, pop_entry: Json<PopEntry>) -> impl Responder {
    if !pop_entry.validate() {
        return HttpResponse::BadRequest().finish();
    }
    
    handle_update_structure(session, redis, EntryAction::Pop, pop_entry.index.as_slice(), None)
}

#[cfg(test)]
mod tests {
    use super::Validatable;

    #[test]
    fn test_validate_push_entry() {
        let push_entry = super::PushEntry {
            title: "Test".to_string(),
            index: vec![0, 0]
        };

        assert!(push_entry.validate());
    }

    #[test]
    fn test_validate_push_entry_invalid_title() {
        let push_entry = super::PushEntry {
            title: "Test!".to_string(),
            index: vec![0, 0]
        };

        assert!(!push_entry.validate());
    }

    #[test]
    fn test_validate_push_entry_invalid_depth() {
        let push_entry = super::PushEntry {
            title: "Test".to_string(),
            index: vec![0, 0, 0]
        };

        assert!(!push_entry.validate());
    }

    #[test]
    fn test_validate_pop_entry() {
        let pop_entry = super::PopEntry {
            index: vec![0, 0]
        };

        assert!(pop_entry.validate());
    }

    #[test]
    fn test_validate_pop_entry_invalid_depth() {
        let pop_entry = super::PopEntry {
            index: vec![0, 0, 0]
        };

        assert!(!pop_entry.validate());
    }
}