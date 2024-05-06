use crate::models::{model::Model, task_structure::TaskStructure, tasks::Task, user::User};
use actix_session::Session;
use actix_web::{web::{Json, Data}, HttpResponse, Responder};
use redis::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct UpdateEntries {
    pub entries: Vec<Task>
}

#[actix_web::patch("/api/entries/update")]
pub(super) async fn update(session: Session, redis: Data<Client>, entries_update: Json<UpdateEntries>) -> impl Responder {
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
    
    let entries_update = entries_update.into_inner();
    let task_structure = TaskStructure::new(entries_update.entries);

    if !task_structure.update(&mut connection) {
        return HttpResponse::InternalServerError().finish();
    };

    HttpResponse::Ok().finish()
}   