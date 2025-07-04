use crate::models::{client_user::ClientUser, user::User};
use crate::utilities::parsable::{Username, PlainTextPassword};
use crate::middlewares::jwt_extractor::JwtClaim;
use crate::services::prelude::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginUserQuery {
    pub username: Username,
    pub password: PlainTextPassword
}

#[actix_web::post("/login")]
pub(super) async fn login_user(pool: Data<PgPool>, memory_cache: Data<MemoryCache>, login_user_query: Json<LoginUserQuery>) -> impl Responder {
    let user = match User::login(&pool, login_user_query.username.as_str(), login_user_query.password.as_str()).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().finish()
    };

    let client_user = ClientUser::from(user);
    let token = JwtClaim::encode(client_user.clone());

    UserSession::respond(&pool, &memory_cache, client_user, Some(token.as_str())).await
}
