use crate::{auth_admin, auth_user};
use crate::{models::user::User, utilities::parsable::PlainTextPassword};
use actix_web::{web::{Data, Json}, HttpResponse, Responder};
use actix_session::Session;
use serde::Deserialize;
use sqlx::{Pool, Postgres};

#[derive(Deserialize)]
struct AdminDeleteUserQuery {
    user_id: i32
}

#[derive(Deserialize)]
struct DeleteSelfQuery {
    password: PlainTextPassword
}

#[actix_web::delete("/api/user/delete-user")]
pub(super) async fn delete_user(session: Session, pool: Data<Pool<Postgres>>, admin_delete_user_query: Json<AdminDeleteUserQuery>) -> impl Responder {
    auth_admin!(user_id, session, pool);

    if User::delete(&pool, admin_delete_user_query.user_id).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

#[actix_web::delete("/api/user/delete-self")]
pub(super) async fn delete_self(session: Session, pool: Data<Pool<Postgres>>, delete_self_query: Json<DeleteSelfQuery>) -> impl Responder {
    auth_user!(user_id, session);

    let user = match User::fetch_by_id(&pool, user_id).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().finish()
    };

    if !user.validate_password(delete_self_query.password.as_str()) {
        return HttpResponse::Unauthorized().finish();
    }

    if User::delete(&pool, user_id).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    session.remove("uid");

    HttpResponse::Ok().finish()
}