use super::*;
use actix_web::web::ServiceConfig;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(create_user::create);
    cfg.service(login_user::login);
}