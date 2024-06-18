use crate::models::entry_structure::Subtask;
use crate::models::rotation::Rotation;
use crate::models::user_task::UserTask;
use crate::utilities::parsable::Comment;
use crate::services::prelude::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateUserTaskQuery {
    subtask_id: i32,
    rotation_id: i32,
    is_completed: bool,
    comment: Comment
}

#[derive(Serialize)]
struct CreateUserTaskResponse {
    id: i32
}

#[actix_web::post("/create")]
pub(super) async fn create_user_task(session: Session, pool: Data<PgPool>, create_user_task_query: Json<CreateUserTaskQuery>) -> impl Responder {
    auth_not_admin_session!(user_id, session, pool);

    let create_user_task_query = create_user_task_query.into_inner();

    let mut user_task = UserTask::new(
        user_id,
        create_user_task_query.subtask_id,
        create_user_task_query.rotation_id,
        create_user_task_query.is_completed,
        create_user_task_query.comment
    );

    match user_task.exists(&pool).await {
        Ok(exists) => if exists {
            return HttpResponse::Conflict().finish();
        },
        Err(_) => return HttpResponse::InternalServerError().finish()
    }

    match Subtask::exists(&pool, user_task.subtask_id()).await {
        Ok(exists) => if !exists {
            return HttpResponse::BadRequest().finish();
        },
        Err(_) => return HttpResponse::InternalServerError().finish()
    }

    match Rotation::exists(&pool, user_task.rotation_id()).await {
        Ok(exists) => if !exists {
            return HttpResponse::BadRequest().finish();
        },
        Err(_) => return HttpResponse::InternalServerError().finish()
    }

    if user_task.insert(&pool).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Created().json(CreateUserTaskResponse { id: user_task.id() })
}