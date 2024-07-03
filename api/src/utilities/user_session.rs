use super::phantom_data::Synced;
use crate::models::{client_user::ClientUser, rotation::Rotation};
use std::collections::HashMap;
use actix_web::{http::header, HttpResponse};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct UserSession {
    pub user: ClientUser,
    pub rotations: HashMap<i32, Rotation<Synced>>
}

impl UserSession {
    /// Builds a `UserSession` to be returned as a JSON response.
    pub async fn respond(pool: &PgPool, user: ClientUser, token: Option<&str>) -> HttpResponse {        
        let rotations = match Rotation::fetch_all_as_map(pool).await {
            Ok(rotations) => rotations,
            Err(_) => return HttpResponse::InternalServerError().finish()
        };

        let session = Self {
            user,
            rotations
        };

        let mut response = HttpResponse::Ok();

        if let Some(token) = token {
            response.append_header((header::AUTHORIZATION, token));
        }

        response.json(session)
    }
}