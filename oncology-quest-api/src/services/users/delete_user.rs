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
    if let Err(response) = handle_admin_session_validation(&pool, &session).await {
        return response;
    }

    match User::delete_other(&pool, admin_delete_user_query.user_id).await {
        Ok(success) => match success {
            true => return HttpResponse::Ok().finish(),
            false => return HttpResponse::Forbidden().finish()
        },
        Err(_) => HttpResponse::InternalServerError().finish()
    };

    HttpResponse::Ok().finish()
}

#[actix_web::delete("/delete-self")]
pub(super) async fn delete_self(session: Session, pool: Data<PgPool>, delete_self_query: Json<DeleteSelfQuery>) -> impl Responder {    
    let user_id = match handle_any_session_validation(&session).await {
        Ok(user_id) => user_id,
        Err(response) => return response
    };

    let user = match User::fetch_by_id(&pool, user_id).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::NoContent().finish()
    };

    if !user.is_valid_password(delete_self_query.password.as_str()) {
        return HttpResponse::Unauthorized().finish();
    }

    match user.delete_self(&pool).await {
        Ok(success) => match success {
            true => return HttpResponse::Ok().finish(),
            false => return HttpResponse::Forbidden().finish()
        },
        Err(_) => HttpResponse::InternalServerError().finish()
    };

    User::clear_session(&session);

    HttpResponse::Ok().finish()
}