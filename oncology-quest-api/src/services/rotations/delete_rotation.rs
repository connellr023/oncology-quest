use crate::models::rotation::Rotation;
use crate::services::prelude::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeleteRotationQuery {
    rotation_id: i32
}

#[actix_web::delete("/delete")]
pub(super) async fn delete_rotation(session: Session, pool: Data<PgPool>, delete_rotation_query: Json<DeleteRotationQuery>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    if let Err(e) = Rotation::delete(&pool, delete_rotation_query.rotation_id).await {
        eprintln!("Failed to delete rotation: {:?}", e);
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}