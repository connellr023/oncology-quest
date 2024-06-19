use crate::models::rotation::RotationModel;
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
pub(super) async fn create_rotation(session: Session, pool: Data<PgPool>, create_rotation_query: Json<CreateRotationQuery>) -> impl Responder {
    auth_admin_session!(user_id, session, pool);

    let mut rotation = RotationModel::new(create_rotation_query.into_inner().name);

    if rotation.insert(&pool).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Created().json(CreateRotationResponse {
        rotation_id: rotation.id(),
        last_updated: rotation.last_updated()
    })
}