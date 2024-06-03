use crate::auth_not_admin_session;
use crate::models::user_task::UserTask;
use crate::utilities::parsable::Comment;
use actix_session::Session;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateUserTaskQuery {
    id: i32,
    is_completed: bool,
    comment: Comment
}

#[actix_web::patch("/update")]
pub(super) async fn update_user_task(session: Session, pool: Data<PgPool>, update_user_task_query: Json<UpdateUserTaskQuery>) -> impl Responder {
    auth_not_admin_session!(user_id, session, pool);

    let update_user_task_query = update_user_task_query.into_inner();

    if UserTask::update(&pool, update_user_task_query.id, user_id, update_user_task_query.is_completed, update_user_task_query.comment.as_str()).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}