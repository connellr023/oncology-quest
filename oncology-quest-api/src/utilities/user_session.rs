use super::phantom_data::Synced;
use crate::models::{client_user::ClientUser, rotation::Rotation};
use std::collections::HashMap;
use actix_web::HttpResponse;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct UserSession {
    pub user: ClientUser,
    pub rotations: HashMap<i32, Rotation<Synced>>
}

impl UserSession {
    /// Builds a `UserSession` to be returned as a JSON response.
    pub async fn respond(pool: &PgPool, user: ClientUser) -> HttpResponse {        
        let rotations = match Rotation::fetch_all_as_map(pool).await {
            Ok(rotations) => rotations,
            Err(_) => return HttpResponse::InternalServerError().finish()
        };

        let response = Self {
            user,
            rotations
        };

        HttpResponse::Ok().json(response)
    }
}