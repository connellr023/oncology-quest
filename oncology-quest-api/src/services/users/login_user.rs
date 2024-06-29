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
pub(super) async fn login_user(claim: JwtUserClaim, pool: Data<PgPool>, login_user_query: Json<LoginUserQuery>) -> impl Responder {
    let user = match User::login(&pool, login_user_query.username.as_str(), login_user_query.password.as_str()).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().finish()
    };

    // TODO: Implement login_user

    UserSession::respond(&pool, claim.sub).await
}