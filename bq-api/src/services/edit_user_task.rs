use crate::auth_not_admin_session;
use crate::models::user_task::UserTask;
use crate::utilities::parsable::Comment;
use actix_session::Session;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateUserTaskQuery {
    subtask_id: i32,
    is_completed: bool,
    comment: Comment
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateUserTaskQuery {
    id: i32,
    is_completed: bool,
    comment: Comment
}

#[derive(Serialize)]
struct CreateUserTaskResponse {
    id: i32
}

#[actix_web::post("/api/user/tasks/create")]
pub(super) async fn create_user_task(session: Session, pool: Data<Pool<Postgres>>, create_user_task_query: Json<CreateUserTaskQuery>) -> impl Responder {
    auth_not_admin_session!(user_id, session, pool);

    let create_user_task_query = create_user_task_query.into_inner();

    let mut user_task = UserTask::new(
        user_id,
        create_user_task_query.subtask_id,
        create_user_task_query.is_completed,
        create_user_task_query.comment
    );

    match user_task.exists(&pool).await {
        Ok(exists) => if exists {
            return HttpResponse::Conflict().finish();
        },
        Err(_) => return HttpResponse::InternalServerError().finish()
    }

    if user_task.insert(&pool).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().json(CreateUserTaskResponse { id: user_task.id() })
}

#[actix_web::patch("/api/user/tasks/update")]
pub(super) async fn update_user_task(session: Session, pool: Data<Pool<Postgres>>, update_user_task_query: Json<UpdateUserTaskQuery>) -> impl Responder {
    auth_not_admin_session!(user_id, session, pool);

    let update_user_task_query = update_user_task_query.into_inner();

    if UserTask::update(&pool, update_user_task_query.id, user_id, update_user_task_query.is_completed, update_user_task_query.comment.as_str()).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}