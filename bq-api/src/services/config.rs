use super::*;
use actix_web::web::ServiceConfig;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(register_user::register);
    cfg.service(login_user::login);
    cfg.service(logout_user::logout);
    cfg.service(user_session::session);
    cfg.service(update_task::update);
    cfg.service(search_user::search);
    cfg.service(update_entry::update);
    cfg.service(update_structure::push);
    cfg.service(update_structure::pop);
    cfg.service(reset_password::reset);
    cfg.service(reset_password::allow_reset);
}