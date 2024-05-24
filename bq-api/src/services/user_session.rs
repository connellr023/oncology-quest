use crate::auth_user_session_with_id;
use crate::models::user_task::UserTask;
use crate::models::{client_user::ClientUser, domain::Domain, user::User};
use actix_web::web::Path;
use actix_web::{web::Data, HttpResponse, Responder};
use actix_session::Session;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use serde::Serialize;

#[derive(Serialize)]
pub struct UserSessionResponse {
    pub user: ClientUser,
    pub domains: Box<[Domain]>,
    pub tasks: Option<Box<[UserTask]>>
}

impl UserSessionResponse {
    /// Builds a user session response from a user, task cache timestamp, and a list of domains which are fetched from the database.
    /// 
    /// # Arguments
    /// 
    /// * `pool` - A connection pool to the database.
    /// * `user` - The user to build the response from.
    /// * `task_cache_timestamp` - An optional timestamp for validating the user's task cache.
    /// 
    /// # Returns
    /// 
    /// A user session response containing the user and the list of domains.
    pub async fn build(pool: &Pool<Postgres>, user: User, task_cache_timestamp: Option<DateTime<Utc>>) -> anyhow::Result<Self> {        
        let domains = Domain::fetch_all(&pool).await?;
        let user = ClientUser::from(user);
        let tasks = UserTask::fetch_all_if_updated(&pool, user.id, task_cache_timestamp).await?;

        let response = Self {
            user,
            domains,
            tasks
        };

        Ok(response)
    }
}

#[actix_web::get("/api/user/session/{task_cache_timestamp:.*}")]
pub(super) async fn session(session: Session, pool: Data<Pool<Postgres>>, task_cache_timestamp: Path<String>) -> impl Responder {
    auth_user_session_with_id!(user_id, session);
    
    let task_cache_timestamp = task_cache_timestamp.parse::<DateTime<Utc>>().ok();

    let user = match User::fetch_by_id(&pool, user_id).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().finish()
    };

    let response = match UserSessionResponse::build(&pool, user, task_cache_timestamp).await {
        Ok(response) => response,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    HttpResponse::Ok().json(response)
}