use crate::models::user_task::UserTask;
use crate::services::prelude::*;

#[actix_web::get("/{user_id}/{rotation_id}")]
pub(super) async fn get_user_tasks(claim: JwtUserClaim, pool: Data<PgPool>, path: Path<(i32, i32)>) -> impl Responder {
    if !claim.sub.is_admin {
        return HttpResponse::Unauthorized().finish();
    }

    let (user_id, rotation_id) = path.into_inner();

    match UserTask::fetch_as_map(&pool, user_id, rotation_id).await {
        Ok(user_tasks) => HttpResponse::Ok().json(user_tasks),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch user tasks.")
    }
}

#[actix_web::get("/{rotation_id}")]
pub(super) async fn get_own_tasks(claim: JwtUserClaim, pool: Data<PgPool>, rotation_id: Path<i32>) -> impl Responder {
    if claim.sub.is_admin {
        return HttpResponse::Unauthorized().finish();
    }
    
    match UserTask::fetch_as_map(&pool, claim.sub.id, *rotation_id).await {
        Ok(user_tasks) => HttpResponse::Ok().json(user_tasks),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch own user tasks.")
    }
}