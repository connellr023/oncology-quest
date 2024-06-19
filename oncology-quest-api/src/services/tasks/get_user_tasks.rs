use crate::models::{user::User, user_task::UserTask};
use crate::services::prelude::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetUserTasksQuery {
    pub tasks_cache_timestamp: Option<DateTime<Utc>>
}

#[actix_web::get("/{rotation_id}")]
pub(super) async fn get_user_tasks(session: Session, pool: Data<PgPool>, rotation_id: Path<i32>, query: Query<GetUserTasksQuery>) -> impl Responder {
    let user_id = match handle_any_session_validation(&session) {
        Ok(user_id) => user_id,
        Err(response) => return response
    };

    match User::is_task_cache_valid(&pool, user_id, query.tasks_cache_timestamp).await {
        Err(_) => return HttpResponse::InternalServerError().finish(),
        Ok(true) => return HttpResponse::NotModified().finish(),
        _ => {}
    }
    
    match UserTask::fetch_as_map(&pool, user_id, *rotation_id).await {
        Ok(user_tasks) => HttpResponse::Ok().json(user_tasks),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}