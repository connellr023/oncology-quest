use crate::auth_admin_session;
use crate::models::rotation::Rotation;
use actix_session::Session;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeleteRotationQuery {
    rotation_id: i32
}

#[actix_web::delete("/delete")]
pub(super) async fn delete_rotation(session: Session, pool: Data<PgPool>, delete_rotation_query: Json<DeleteRotationQuery>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    if Rotation::delete(&pool, delete_rotation_query.rotation_id).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}