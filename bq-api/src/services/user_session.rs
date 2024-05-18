use crate::models::{client_user::ClientUser, model::Model, tasks_model::TasksModel, user_model::UserModel};
use actix_web::{web::{Data, Query}, HttpResponse, Responder};
use actix_session::Session;
use chrono::{DateTime, Utc};
use redis::{Client, Connection};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct UserSession {
    pub user: ClientUser,
    pub structure: Option<TasksModel>
}

/// Generates an HTTP response containing the user session data with the task structure.
/// 
/// # Arguments
/// 
/// * `connection` - A mutable reference to a Redis connection.
/// * `structure_cache_timestamp` - The timestamp of the last update to the client's task structure cache.
/// * `user` - The user to generate the session response for.
/// 
/// # Returns
/// 
/// An `HttpResponse` containing the user session data with the task structure or an error response if an error occurred.
pub(super) fn handle_session_response(connection: &mut Connection, structure_cache_timestamp: Option<DateTime<Utc>>, user: UserModel) -> HttpResponse {
    let structure = match TasksModel::fetch(connection, "") {
        Ok(structure) => {
            match structure_cache_timestamp {
                Some(cache_timestamp) => {
                    if structure.last_updated().gt(&cache_timestamp) {
                        // The structure has been updated since last cache timestamp
                        Some(structure)
                    } else {
                        // The structure has not been updated since last cache timestamp
                        None
                    }
                },
                None => {
                    // The client does not have the structure, send it
                    Some(structure)
                }
            }
        },
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let user_client = UserSession {
        user: user.into(),
        structure
    };

    HttpResponse::Ok().json(user_client)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FetchSessionQuery {
    pub structure_cache_timestamp: Option<DateTime<Utc>>
}

#[actix_web::get("/api/user/session")]
pub(super) async fn session(session: Session, redis: Data<Client>, fetch_session: Query<FetchSessionQuery>) -> impl Responder {
    let mut connection = match redis.get_connection() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let username = match session.get::<String>("username") {
        Ok(Some(username)) => username,
        _ => return HttpResponse::Unauthorized().finish()
    };

    let user = match UserModel::fetch(&mut connection, username.as_str()) {
        Ok(user) => user,
        Err(_) => return HttpResponse::NotFound().finish()
    };

    handle_session_response(&mut connection, fetch_session.structure_cache_timestamp, user)
}