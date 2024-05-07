use crate::models::{client_user::ClientUser, model::Model, task_structure::TaskStructure, tasks::Task, user::User};
use actix_web::{web::Data, HttpResponse, Responder};
use actix_session::Session;
use redis::{Client, Connection};
use serde::Serialize;

#[derive(Serialize)]
struct UserSession {
    pub user: ClientUser,
    pub entries: Vec<Task>
}

/// Generates an HTTP response containing the user session data with the task structure.
/// 
/// # Arguments
/// 
/// * `connection` - A mutable reference to a Redis connection.
/// * `user` - The user to generate the session response for.
/// 
/// # Returns
/// 
/// An `HttpResponse` containing the user session data with the task structure or an error response if an error occurred.
pub(super) fn handle_session_response(connection: &mut Connection, user: User) -> HttpResponse {
    let entries = match TaskStructure::fetch(connection, "") {
        Some(task_structure) => task_structure.structure_as_owned(),
        None => return HttpResponse::InternalServerError().finish()
    };

    let user_client = UserSession {
        user: user.into(),
        entries
    };

    HttpResponse::Ok().json(user_client)
}

#[actix_web::get("/api/user/session")]
pub(super) async fn session(session: Session, redis: Data<Client>) -> impl Responder {
    let mut connection = match redis.get_connection() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let username = match session.get::<String>("username") {
        Ok(Some(username)) => username,
        _ => return HttpResponse::Unauthorized().finish()
    };

    let user = match User::fetch(&mut connection, username.as_str()) {
        Some(user) => user,
        None => return HttpResponse::NotFound().finish()
    };

    handle_session_response(&mut connection, user)
}