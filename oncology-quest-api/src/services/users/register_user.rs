use crate::utilities::parsable::{Username, Name, PlainTextPassword};
use crate::models::user::User;
use crate::services::prelude::*;

#[derive(Deserialize)]
struct RegisterUserQuery {
    pub username: Username,
    pub name: Name,
    pub password: PlainTextPassword
}

#[actix_web::post("/register")]
pub(super) async fn register_user(pool: Data<PgPool>, register_user_query: Json<RegisterUserQuery>) -> impl Responder {
    let register_user_query = register_user_query.into_inner();
    
    let user = match User::new(register_user_query.username, register_user_query.name, false, register_user_query.password) {
        Ok(user) => user,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    match user.exists(&pool).await {
        Ok(exists) => if exists {
            return HttpResponse::Conflict().finish();
        },
        Err(_) => return HttpResponse::InternalServerError().finish()
    }

    if user.insert(&pool).await.is_err() {
        return HttpResponse::InternalServerError().body("Failed to insert user into database");
    }

    HttpResponse::Created().finish()
}