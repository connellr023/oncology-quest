use crate::{models::user::User, utilities::parsable::PlainTextPassword};
use crate::services::prelude::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AdminDeleteUserQuery {
    user_id: i32
}

#[derive(Deserialize)]
struct DeleteSelfQuery {
    password: PlainTextPassword
}

#[actix_web::delete("/delete-other-user")]
pub(super) async fn delete_other_user(session: Session, pool: Data<PgPool>, admin_delete_user_query: Json<AdminDeleteUserQuery>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    if User::delete(&pool, admin_delete_user_query.user_id, false).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

#[actix_web::delete("/delete-self")]
pub(super) async fn delete_self(session: Session, pool: Data<PgPool>, delete_self_query: Json<DeleteSelfQuery>) -> impl Responder {
    auth_user_session_with_id!(user_id, session);

    let user = match User::fetch_by_id(&pool, user_id).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::NoContent().finish()
    };

    if !user.validate_password(delete_self_query.password.as_str()) {
        return HttpResponse::Unauthorized().finish();
    }

    if User::delete(&pool, user_id, true).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    session.remove("uid");

    HttpResponse::Ok().finish()
}