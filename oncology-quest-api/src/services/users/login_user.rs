use super::get_user_session::UserSessionResponse;
use crate::models::user::User;
use crate::utilities::parsable::{Username, PlainTextPassword};
use crate::services::prelude::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginUserQuery {
    pub username: Username,
    pub password: PlainTextPassword
}

#[actix_web::post("/login")]
pub(super) async fn login_user(session: Session, pool: Data<PgPool>, login_user_query: Json<LoginUserQuery>) -> impl Responder {
    let user = match User::login(&pool, login_user_query.username.as_str(), login_user_query.password.as_str()).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().finish()
    };

    if !User::is_session_insert_ok(&session, user.id()) {
        return HttpResponse::InternalServerError().finish();
    }

    match UserSessionResponse::new(&pool, user).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}