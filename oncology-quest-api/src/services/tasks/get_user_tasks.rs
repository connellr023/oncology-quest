use crate::models::user_task::UserTask;
use crate::services::prelude::*;

#[actix_web::get("/{user_id}/{rotation_id}")]
pub(super) async fn get_user_tasks(session: Session, pool: Data<PgPool>, path: Path<(i32, i32)>) -> impl Responder {
    if let Err(response) = UserSession::validate(&pool, &session, UserSessionRole::Admin).await {
        return response;
    }

    let (user_id, rotation_id) = path.into_inner();

    match UserTask::fetch_as_map(&pool, user_id, rotation_id).await {
        Ok(user_tasks) => HttpResponse::Ok().json(user_tasks),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch user tasks.")
    }
}

#[actix_web::get("/{rotation_id}")]
pub(super) async fn get_own_tasks(session: Session, pool: Data<PgPool>, rotation_id: Path<i32>) -> impl Responder {
    let user_id = match UserSession::validate(&pool, &session, UserSessionRole::Regular).await {
        Ok(user_id) => user_id,
        Err(response) => return response
    };
    
    match UserTask::fetch_as_map(&pool, user_id, *rotation_id).await {
        Ok(user_tasks) => HttpResponse::Ok().json(user_tasks),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch own user tasks.")
    }
}