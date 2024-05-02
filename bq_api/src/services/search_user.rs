use crate::models::{model::Model, user::{User, ClientUser}};
use actix_web::{web::{Data, Path}, HttpResponse, Responder};
use actix_session::Session;
use serde::{Serialize, Deserialize};
use redis::Client;

#[derive(Deserialize)]
pub struct UserSearch {
    pub username_query: String
}

#[derive(Serialize)]
pub struct UserSearchResults {
    pub users: Vec<ClientUser>
}

#[actix_web::get("/api/users/search/{query}")]
pub(super) async fn search(session: Session, search: Path<UserSearch>, redis: Data<Client>) -> impl Responder {
    let mut connection = match redis.get_connection() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let username = match session.get::<String>("username") {
        Ok(Some(username)) => username,
        _ => return HttpResponse::Unauthorized().finish()
    };

    let current_user = match User::fetch(&mut connection, username.as_str()) {
        Some(user) => user,
        None => return HttpResponse::NotFound().finish()
    };

    // Only admins can search for users.
    if !current_user.is_admin() {
        return HttpResponse::Forbidden().finish();
    }

    let users = match ClientUser::text_search(&mut connection, search.username_query.as_str()) {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    HttpResponse::Ok().json(UserSearchResults { users })
}