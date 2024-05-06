use crate::models::{model::Model, task_structure::TaskStructure, user::User};
use actix_session::Session;
use actix_web::{web::{Json, Data}, HttpResponse, Responder};
use redis::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct UpdateEntry {
    pub title: String,
    pub index: Vec<u16>
}

#[actix_web::patch("/api/entries/update")]
pub(super) async fn update(session: Session, redis: Data<Client>, entry_update: Json<UpdateEntry>) -> impl Responder {
    let username = match session.get::<String>("username") {
        Ok(Some(username)) => username,
        _ => return HttpResponse::Unauthorized().finish()
    };
    
    let mut connection = match redis.get_connection() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let user = match User::fetch(&mut connection, username.as_str()) {
        Some(user) => user,
        None => return HttpResponse::NotFound().finish()
    };

    // Only admins can update entries.
    if !user.is_admin {
        return HttpResponse::Forbidden().finish();
    };

    let mut task_structure = match TaskStructure::fetch(&mut connection, "") {
        Some(task_structure) => task_structure,
        None => return HttpResponse::InternalServerError().finish()
    };

    if !task_structure.update_existing(&mut connection, entry_update.index.as_slice(), entry_update.title.as_str()) {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}   