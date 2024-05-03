use crate::models::{model::Model, user::{ClientUser, User}};
use actix_web::{web::Data, HttpResponse, Responder};
use actix_session::Session;
use redis::{Client, Connection};
use serde::Serialize;

#[derive(Serialize)]
struct UserSession {
    pub user: ClientUser,
    pub tasks: String
}

/// Fetches the JSON representation of tasks from Redis.
///
/// # Arguments
///
/// * `connection` - A mutable reference to a Redis connection.
/// * `tasks_key` - The key used to retrieve the tasks from Redis. Should just be `tasks`.
///
/// # Returns
///
/// An `Option<String>` containing the JSON representation of tasks if successful, or `None` if an error occurred.
fn fetch_tasks_json(connection: &mut Connection, tasks_key: &str) -> Option<String> {
    let result = redis::cmd("GET")
        .arg(tasks_key)
        .query::<String>(connection);

    match result {
        Ok(value) => Some(value),
        Err(_) => None
    }
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
pub(super) fn session_response_json(connection: &mut Connection, user: User) -> HttpResponse {
    let tasks_json = match fetch_tasks_json(connection, "tasks") {
        Some(tasks_json) => tasks_json,
        None => return HttpResponse::InternalServerError().finish()
    };

    let user_client = UserSession {
        user: user.into(),
        tasks: tasks_json
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

    session_response_json(&mut connection, user)
}