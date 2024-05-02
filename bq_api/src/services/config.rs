use super::*;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user::create_user);
    cfg.service(redis_test::redis_test);
}