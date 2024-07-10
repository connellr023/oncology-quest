use crate::models::rotation::Rotation;
use crate::services::prelude::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeleteRotationQuery {
    rotation_id: i32
}

#[actix_web::delete("/delete")]
pub(super) async fn delete_rotation(claim: JwtUserClaim, pool: Data<PgPool>, memory_cache: Data<MemoryCache>, delete_rotation_query: Json<DeleteRotationQuery>) -> impl Responder {
    if !claim.sub.is_admin {
        return HttpResponse::Unauthorized().finish();
    }

    if Rotation::delete(&pool, delete_rotation_query.rotation_id).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let _ = memory_cache.delete_rotation(delete_rotation_query.rotation_id);

    HttpResponse::Ok().finish()
}
