use super::*;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user::create);
    cfg.service(login_user::login);
}