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
    let user_id = match handle_regular_session_validation(&pool, &session).await {
        Ok(user_id) => user_id,
        Err(response) => return response
    };

    let create_user_task_query = create_user_task_query.into_inner();

    let user_task = UserTask::new(
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
        Err(_) => return HttpResponse::InternalServerError().body("Failed to check if user task exists.")
    }

    match Subtask::exists(&pool, user_task.subtask_id()).await {
        Ok(exists) => if !exists {
            return HttpResponse::BadRequest().finish();
        },
        Err(_) => return HttpResponse::InternalServerError().body("Failed to check if subtask exists.")
    }

    match Rotation::exists(&pool, user_task.rotation_id()).await {
        Ok(exists) => if !exists {
            return HttpResponse::BadRequest().finish();
        },
        Err(_) => return HttpResponse::InternalServerError().body("Failed to check if rotation exists.")
    }

    let user_task = match user_task.insert(&pool).await {
        Ok(user_task) => user_task,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to insert user task.")
    };

    HttpResponse::Created().json(CreateUserTaskResponse { id: user_task.id() })
}