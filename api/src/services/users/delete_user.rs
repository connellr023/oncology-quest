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
pub(super) async fn delete_other_user(claim: JwtUserClaim, pool: Data<PgPool>, admin_delete_user_query: Json<AdminDeleteUserQuery>) -> impl Responder {
    if !claim.sub.is_admin {
        return HttpResponse::Unauthorized().finish();
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
pub(super) async fn delete_self(claim: JwtUserClaim, pool: Data<PgPool>, delete_self_query: Json<DeleteSelfQuery>) -> impl Responder {    
    // fetching entire user may not be necessary
    // just need to check password
    let user = match User::fetch_by_id(&pool, claim.sub.id).await {
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

    // TODO: Invalidate JWT

    HttpResponse::Ok().finish()
}