use crate::models::user::User;
use crate::services::prelude::*;

#[actix_web::get("/session")]
pub(super) async fn get_user_session(session: Session, pool: Data<PgPool>) -> impl Responder {
    let user_id = match UserSession::validate(&pool, &session, UserSessionRole::Any).await {
        Ok(user_id) => user_id,
        Err(response) => return response
    };

    let user = match User::fetch_by_id(&pool, user_id).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().finish()
    };

    UserSession::respond(&pool, user).await
}