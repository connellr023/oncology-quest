use super::{memory_cache::MemoryCache, phantom_data::Synced};
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
    ///
    /// # Arguments
    ///
    /// * `pool` - A connection pool to the database.
    /// * `memory_cache` - A memory cache to store and retrieve cached data to avoid database queries.
    /// * `user` - A `ClientUser` to be included in the session.
    /// * `token` - An optional JWT token to be included in the response headers.
    ///
    /// # Returns
    ///
    /// An `HttpResponse` containing the `UserSession` as a JSON response.
    pub async fn respond(pool: &PgPool, memory_cache: &MemoryCache, user: ClientUser, token: Option<&str>) -> HttpResponse {
        let rotations = match memory_cache.get_rotations_as_clone() {
            Ok(Some(rotations)) => rotations,
            _ => {
                let rotations = match Rotation::fetch_all_as_map(pool).await {
                    Ok(rotations) => rotations,
                    Err(_) => return HttpResponse::InternalServerError().finish()
                };

                let _ = memory_cache.set_rotations(Some(rotations.clone()));

                rotations
            }
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
