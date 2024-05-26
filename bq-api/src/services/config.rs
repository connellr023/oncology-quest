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
    cfg.service(edit_entry_structure::create_supertask);
    cfg.service(edit_entry_structure::update_supertask);
    cfg.service(edit_entry_structure::delete_supertask);
    cfg.service(edit_entry_structure::create_task);
    cfg.service(edit_entry_structure::update_task);
    cfg.service(edit_entry_structure::delete_task);
    cfg.service(edit_entry_structure::create_subtask);
    cfg.service(edit_entry_structure::update_subtask);
    cfg.service(edit_entry_structure::delete_subtask);
    cfg.service(edit_domains::create_domain);
    cfg.service(edit_domains::delete_domain);
    cfg.service(edit_user_task::create_user_task);
    cfg.service(edit_user_task::update_user_task);
}