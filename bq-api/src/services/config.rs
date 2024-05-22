use super::*;
use actix_web::web::ServiceConfig;

/// Configures the services for the application.
/// 
/// # Arguments
/// 
/// * `cfg` - The service configuration to configure.
pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(register_user::register);
    cfg.service(login_user::login);
    cfg.service(logout_user::logout);
    cfg.service(user_session::session);
    cfg.service(search_user::search);
    cfg.service(reset_password::reset);
    cfg.service(reset_password::allow_reset);
    cfg.service(delete_user::delete_user);
    cfg.service(delete_user::delete_self);
}