use super::*;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index::index));
    cfg.route("/redis_test", web::get().to(redis_test::redis_test));
}