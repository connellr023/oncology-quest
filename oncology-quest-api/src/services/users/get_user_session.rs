use crate::models::{client_user::ClientUser, rotation::Rotation, user::User};
use crate::services::prelude::*;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct UserSessionResponse {
    pub user: ClientUser,
    pub rotations: HashMap<i32, Rotation<Synced>>
}

impl UserSessionResponse {
    /// Builds a user session response from a user, task cache timestamp, and a list of rotations which are fetched from the database.
    /// 
    /// # Arguments
    /// 
    /// * `pool` - A connection pool to the database.
    /// * `user` - The user to build the response from.
    /// * `task_cache_timestamp` - An optional timestamp for validating the user's task cache.
    /// 
    /// # Returns
    /// 
    /// A user session response if successful, otherwise an error.
    pub async fn new(pool: &PgPool, user: User<Synced>) -> anyhow::Result<Self> {        
        let user = ClientUser::from(user);
        let rotations = Rotation::fetch_all_as_map(pool).await?;

        let response = Self {
            user,
            rotations
        };

        Ok(response)
    }
}

#[actix_web::get("/session")]
pub(super) async fn get_user_session(session: Session, pool: Data<PgPool>) -> impl Responder {
    let user_id = match handle_any_session_validation(&session) {
        Ok(user_id) => user_id,
        Err(response) => return response
    };

    let user = match User::fetch_by_id(&pool, user_id).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().finish()
    };

    let response = match UserSessionResponse::new(&pool, user).await {
        Ok(response) => response,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    HttpResponse::Ok().json(response)
}