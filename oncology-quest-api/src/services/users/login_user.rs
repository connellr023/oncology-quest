use super::get_user_session::UserSessionResponse;
use crate::models::user::User;
use crate::utilities::parsable::{Username, PlainTextPassword};
use crate::services::prelude::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginUserQuery {
    pub username: Username,
    pub password: PlainTextPassword,
    pub task_cache_timestamp: Option<DateTime<Utc>>
}

#[actix_web::post("/login")]
pub(super) async fn login_user(session: Session, pool: Data<PgPool>, login_user_query: Json<LoginUserQuery>) -> impl Responder {
    let user = match User::validate_login(&pool, login_user_query.username.as_str(), login_user_query.password.as_str()).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().finish()
    };

    if session.insert("uid", user.id()).is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    match UserSessionResponse::new(&pool, user, login_user_query.task_cache_timestamp).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}