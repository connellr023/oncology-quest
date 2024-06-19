use crate::models::{rotation::Rotation, user::{User, UserSession}};
use actix_session::Session;
use actix_web::HttpResponse;
use sqlx::PgPool;

pub fn handle_any_session_validation(session: &Session) -> Result<i32, HttpResponse> {
    let user_id = match UserSession::extract_user_id(&session) {
        Some(user_id) => user_id,
        None => return Err(HttpResponse::Unauthorized().finish())
    };

    Ok(user_id)
}

pub async fn handle_regular_session_validation(pool: &PgPool, session: &Session) -> Result<i32, HttpResponse> {
    match User::validate_admin_session(&pool, &session).await {
        Ok(true) => return Err(HttpResponse::Unauthorized().finish()),
        Err(_) => return Err(HttpResponse::InternalServerError().finish()),
        _ => {}
    }

    Ok(UserSession::extract_user_id(&session).unwrap())
}

pub async fn handle_admin_session_validation(pool: &PgPool, session: &Session) -> Result<i32, HttpResponse> {
    match User::validate_admin_session(&pool, &session).await {
        Ok(false) => return Err(HttpResponse::Unauthorized().finish()),
        Err(_) => return Err(HttpResponse::InternalServerError().finish()),
        _ => {}
    }

    Ok(UserSession::extract_user_id(&session).unwrap())
}

pub async  fn handle_rotation_validation(pool: &PgPool, rotation_id: i32) -> Result<(), HttpResponse> {
    match Rotation::exists(&pool, rotation_id).await {
        Ok(exists) => if !exists {
            return Err(HttpResponse::BadRequest().finish());
        },
        Err(_) => return Err(HttpResponse::InternalServerError().finish())
    }

    Ok(())
}