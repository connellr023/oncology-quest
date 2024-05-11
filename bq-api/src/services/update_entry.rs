use crate::models::{model::Model, task_structure::TaskStructure, user::User};
use crate::utilities::parsables::EntryTitle;
use actix_session::Session;
use actix_web::{web::{Json, Data}, HttpResponse, Responder};
use redis::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct UpdateEntry {
    pub title: EntryTitle,
    pub index: Vec<u16>
}

#[actix_web::patch("/api/entries/update")]
pub(super) async fn update(session: Session, redis: Data<Client>, entry_update: Json<UpdateEntry>) -> impl Responder {
    if !(entry_update.index.is_empty() && entry_update.index.len() <= 3) {
        return HttpResponse::BadRequest().finish();
    }
    
    let username = match session.get::<String>("username") {
        Ok(Some(username)) => username,
        _ => return HttpResponse::Unauthorized().finish()
    };
    
    let mut connection = match redis.get_connection() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    // Only admins can update entries.
    if !User::validate_is_admin(&mut connection, username.as_str()) {
        return HttpResponse::Forbidden().finish();
    };

    let mut task_structure = match TaskStructure::fetch(&mut connection, "") {
        Some(task_structure) => task_structure,
        None => return HttpResponse::InternalServerError().finish()
    };

    if !task_structure.update_existing(&mut connection, entry_update.index.as_slice(), entry_update.title.clone()) {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}   