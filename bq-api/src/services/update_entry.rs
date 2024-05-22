use crate::models::{model::Model, tasks_model::TasksModel, user::User, entries::EntryIndex};
use crate::utilities::parsables::SubtaskTitle;
use actix_web::{web::{Json, Data}, HttpResponse, Responder};
use actix_session::Session;
use redis::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct UpdateEntryQuery {
    pub title: SubtaskTitle,
    pub index: EntryIndex
}

#[actix_web::patch("/api/entries/update")]
pub(super) async fn update(session: Session, redis: Data<Client>, entry_update: Json<UpdateEntryQuery>) -> impl Responder {
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

    let mut task_structure = match TasksModel::fetch(&mut connection, "") {
        Ok(task_structure) => task_structure,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let entry_update = entry_update.into_inner();

    if task_structure.update_existing(&mut connection, &entry_update.index, entry_update.title).is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}