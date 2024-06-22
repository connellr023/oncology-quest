use crate::models::user_task::UserTask;
use crate::utilities::parsable::Comment;
use crate::services::prelude::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateUserTaskQuery {
    id: i32,
    is_completed: bool,
    comment: Comment
}

#[actix_web::patch("/update")]
pub(super) async fn update_user_task(session: Session, pool: Data<PgPool>, update_user_task_query: Json<UpdateUserTaskQuery>) -> impl Responder {
    let user_id = match handle_regular_session_validation(&pool, &session).await {
        Ok(user_id) => user_id,
        Err(response) => return response
    };
    
    let update_user_task_query = update_user_task_query.into_inner();

    if UserTask::update(&pool, update_user_task_query.id, user_id, update_user_task_query.is_completed, update_user_task_query.comment.as_str()).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}