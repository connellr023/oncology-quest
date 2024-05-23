use crate::{models::{entry_structure::Supertask, user::User}, utilities::parsable::EntryTitle};
use actix_session::Session;
use actix_web::{web::{Data, Json}, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateEntryQuery {
    domain_id: i32,
    title: EntryTitle
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateEntryQuery {
    entry_id: i32,
    title: EntryTitle,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeleteEntryQuery {
    entry_id: i32
}

macro_rules! check_user_auth_and_admin {
    ($session:expr, $pool:expr) => {
        let user_id = match $session.get::<i32>("uid") {
            Ok(Some(user_id)) => user_id,
            _ => return HttpResponse::Unauthorized().finish()
        };

        if !User::validate_is_admin(&$pool, user_id).await {
            return HttpResponse::Forbidden().finish();
        }
    };
}

#[actix_web::post("/api/supertasks/create")]
pub(super) async fn create_supertask(session: Session, pool: Data<Pool<Postgres>>, create_entry_query: Json<EntryTitle>) -> impl Responder {
    check_user_auth_and_admin!(session, pool);

    // if Supertask::insert(&pool, create_entry_query.title.as_str()).await.is_err() {
    //     return HttpResponse::InternalServerError().finish();
    // }

    HttpResponse::Ok().finish()
}

#[actix_web::patch("/api/supertasks/update")]
pub(super) async fn update_supertask(session: Session, pool: Data<Pool<Postgres>>, update_entry_query: Json<UpdateEntryQuery>) -> impl Responder {
    check_user_auth_and_admin!(session, pool);

    if Supertask::update_title(&pool, update_entry_query.entry_id, update_entry_query.title.as_str()).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

#[actix_web::delete("/api/supertasks/delete")]
pub(super) async fn delete_supertask(session: Session, pool: Data<Pool<Postgres>>, delete_entry_query: Json<DeleteEntryQuery>) -> impl Responder {
    check_user_auth_and_admin!(session, pool);

    if Supertask::delete(&pool, delete_entry_query.entry_id).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}