use crate::auth_user_with_id;
use crate::models::{client_user::ClientUser, domain::Domain, user::User};
use actix_web::{web::Data, HttpResponse, Responder};
use actix_session::Session;
use sqlx::{Pool, Postgres};
use serde::Serialize;

#[derive(Serialize)]
pub struct UserSessionResponse {
    pub user: ClientUser,
    pub domains: Box<[Domain]>,
}

impl UserSessionResponse {
    /// Builds a user session response from a user and a list of domains which are fetched from the database.
    /// 
    /// # Arguments
    /// 
    /// * `pool` - A connection pool to the database.
    /// * `user` - The user to build the response from.
    /// 
    /// # Returns
    /// 
    /// A user session response containing the user and the list of domains.
    pub async fn build_from_user(pool: &Pool<Postgres>, user: User) -> anyhow::Result<Self> {
        let domains = Domain::fetch_all(&pool).await?;
        let user = ClientUser::from(user);
        let response = Self {
            user,
            domains
        };

        Ok(response)
    }
}

#[actix_web::get("/api/user/session")]
pub(super) async fn session(session: Session, pool: Data<Pool<Postgres>>) -> impl Responder {
    auth_user_with_id!(user_id, session);

    let user = match User::fetch_by_id(&pool, user_id).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().finish()
    };

    let response = match UserSessionResponse::build_from_user(&pool, user).await {
        Ok(response) => response,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    HttpResponse::Ok().json(response)
}