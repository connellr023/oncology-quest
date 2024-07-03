use crate::{models::{entry_structure::{Subtask, Supertask, Task}, rotation::Rotation}, utilities::parsable::EntryTitle};
use crate::services::prelude::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateSupertaskEntryQuery {
    pub rotation_id: i32,
    pub title: EntryTitle
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateLowerEntryQuery {
    pub rotation_id: i32,
    pub parent_id: i32,
    pub title: EntryTitle
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateEntryResponse {
    pub entry_id: i32
}

macro_rules! create_entry_wrapper {
    ($claim:ident, $pool:ident, $rotation_id:expr, $block:block) => {
        if !$claim.sub.is_admin {
            return HttpResponse::Unauthorized().finish();
        }

        match Rotation::exists(&$pool, $rotation_id).await {
            Ok(exists) => if !exists {
                return HttpResponse::BadRequest().finish();
            },
            Err(_) => return HttpResponse::InternalServerError().finish()
        }

        $block
    };
}

#[actix_web::post("/create")]
pub(super) async fn create_supertask(claim: JwtUserClaim, pool: Data<PgPool>, create_entry_query: Json<CreateSupertaskEntryQuery>) -> impl Responder {
    create_entry_wrapper! {claim, pool, create_entry_query.rotation_id, {
        match Supertask::insert_from(&pool, create_entry_query.title.as_str(), create_entry_query.rotation_id).await {
            Ok(entry_id) => HttpResponse::Created().json(CreateEntryResponse { entry_id }),
            Err(_) => HttpResponse::InternalServerError().finish()
        }
    }}
}

#[actix_web::post("/create")]
pub(super) async fn create_task(claim: JwtUserClaim, pool: Data<PgPool>, create_entry_query: Json<CreateLowerEntryQuery>) -> impl Responder {
    create_entry_wrapper! {claim, pool, create_entry_query.rotation_id, {
        match Task::insert_from(&pool, create_entry_query.title.as_str(), create_entry_query.rotation_id, create_entry_query.parent_id).await {
            Ok(entry_id) => HttpResponse::Created().json(CreateEntryResponse { entry_id }),
            Err(_) => HttpResponse::InternalServerError().finish()
        }
    }}
}

#[actix_web::post("/create")]
pub(super) async fn create_subtask(claim: JwtUserClaim, pool: Data<PgPool>, create_entry_query: Json<CreateLowerEntryQuery>) -> impl Responder {
    create_entry_wrapper! {claim, pool, create_entry_query.rotation_id, {
        match Subtask::insert_from(&pool, create_entry_query.title.as_str(), create_entry_query.rotation_id, create_entry_query.parent_id).await {
            Ok(entry_id) => HttpResponse::Created().json(CreateEntryResponse { entry_id }),
            Err(_) => HttpResponse::InternalServerError().finish()
        }
    }}
}