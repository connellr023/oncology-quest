use super::phantom_data::Synced;
use crate::models::{client_user::ClientUser, rotation::Rotation, user::User};
use std::collections::HashMap;
use actix_session::Session;
use actix_web::HttpResponse;
use serde::Serialize;
use sqlx::PgPool;

const SESSION_USER_ID_KEY: &str = "uid";

#[derive(Serialize)]
pub struct UserSession {
    pub user: ClientUser,
    pub rotations: HashMap<i32, Rotation<Synced>>
}

pub enum UserSessionRole {
    Admin,
    Regular,
    Any
}

impl UserSession {
    /// Builds a `UserSession` to be returned as a JSON response.
    pub async fn respond(pool: &PgPool, user: User<Synced>) -> HttpResponse {        
        let user = ClientUser::from(user);
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

    /// Checks a user's role and returns the user's ID if the session role matches the provided role.
    /// If the session role does not match the provided role, an `HttpResponse` is returned.
    pub async fn validate(pool: &PgPool, session: &Session, role: UserSessionRole) -> Result<i32, HttpResponse> {
        let user_id = Self::extract_user_id(session)
            .ok_or_else(|| HttpResponse::Unauthorized().finish())?;

        match role {
            UserSessionRole::Admin | UserSessionRole::Regular => {
                let is_admin = User::is_id_admin(pool, user_id)
                    .await
                    .map_err(|_| HttpResponse::InternalServerError().finish())?;

                match (role, is_admin) {
                    (UserSessionRole::Admin, false) | (UserSessionRole::Regular, true) => return Err(HttpResponse::Unauthorized().finish()),
                    _ => {}
                }
            },
            UserSessionRole::Any => {}
        }

        Ok(user_id)
    }

    /// Checks if the provided session is valid.
    #[inline(always)]
    pub fn extract_user_id(session: &Session) -> Option<i32> {
        session.get::<i32>(SESSION_USER_ID_KEY)
            .ok()?
            .clone()
    }

    #[inline(always)]
    pub fn is_insert_ok(session: &Session, user_id: i32) -> bool {
        session.insert(SESSION_USER_ID_KEY, user_id).is_ok()
    }

    #[inline(always)]
    pub fn clear(session: &Session) {
        session.remove(SESSION_USER_ID_KEY);
    }
}