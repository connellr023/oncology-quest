use crate::models::{user::User, client_user::ClientUser};
use actix_web::{web::{Data, Path}, HttpResponse, Responder};
use actix_session::Session;
use serde::Deserialize;
use redis::Client;

#[derive(Deserialize)]
pub struct UserSearch {
    pub query: String
}

#[actix_web::get("/api/user/search/{query}")]
pub(super) async fn search(session: Session, search: Path<UserSearch>, redis: Data<Client>) -> impl Responder {
    let mut connection = match redis.get_connection() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let username = match session.get::<String>("username") {
        Ok(Some(username)) => username,
        _ => return HttpResponse::Unauthorized().finish()
    };

    // Only admins can search for users.
    if !User::validate_is_admin(&mut connection, username.as_str()) {
        return HttpResponse::Forbidden().finish();
    };

    let users = match ClientUser::text_search(&mut connection, search.query.as_str()) {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    HttpResponse::Ok().json(users)
}