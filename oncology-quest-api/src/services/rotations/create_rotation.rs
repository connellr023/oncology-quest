use crate::auth_admin_session;
use crate::models::rotation::Rotation;
use crate::utilities::parsable::Name;
use actix_session::Session;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;

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

    let mut rotation = Rotation::new(create_rotation_query.into_inner().name);

    if rotation.insert(&pool).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().json(CreateRotationResponse {
        rotation_id: rotation.id(),
        last_updated: rotation.last_updated()
    })
}