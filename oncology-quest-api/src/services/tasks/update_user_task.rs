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
pub(super) async fn update_user_task(claim: JwtUserClaim, pool: Data<PgPool>, update_user_task_query: Json<UpdateUserTaskQuery>) -> impl Responder {
    if claim.sub.is_admin {
        return HttpResponse::Unauthorized().finish();
    }
    
    let update_user_task_query = update_user_task_query.into_inner();

    if UserTask::update(&pool, update_user_task_query.id, claim.sub.id, update_user_task_query.is_completed, update_user_task_query.comment.as_str()).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}