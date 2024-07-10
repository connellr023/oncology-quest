use crate::models::rotation::Rotation;
use crate::utilities::parsable::Name;
use crate::services::prelude::*;

#[derive(Deserialize)]
struct CreateRotationQuery {
    name: Name
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateRotationResponse {
    rotation_id: i32,
    last_updated: DateTime<Utc>
}

#[actix_web::post("/create")]
pub(super) async fn create_rotation(claim: JwtUserClaim, pool: Data<PgPool>, memory_cache: Data<MemoryCache>, create_rotation_query: Json<CreateRotationQuery>) -> impl Responder {
    if !claim.sub.is_admin {
        return HttpResponse::Unauthorized().finish();
    }

    let rotation = Rotation::new(create_rotation_query.into_inner().name);
    let rotation = match rotation.insert(&pool).await {
        Ok(rotation) => rotation,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let _ = memory_cache.insert_rotation(rotation.clone());

    HttpResponse::Created().json(CreateRotationResponse {
        rotation_id: rotation.id(),
        last_updated: rotation.last_updated()
    })
}
