use super::*;
use actix_web::web::{scope, ServiceConfig};

/// Configures the services for the application.
/// 
/// # Arguments
/// 
/// * `cfg` - The service configuration to configure.
pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api")
            .service(
                scope("/users")
                    .service(users::register_user::register_user)
                    .service(users::login_user::login_user)
                    .service(users::logout_user::logout_user)
                    .service(users::reset_user_password::reset_password)
                    .service(users::reset_user_password::allow_reset_password)
                    .service(users::delete_user::delete_other_user)
                    .service(users::delete_user::delete_self)
                    .service(users::get_user_session::get_user_session)
                    .service(users::search_users::search_users)
            )
            .service(
                scope("/tasks")
                    .service(tasks::create_user_task::create_user_task)
                    .service(tasks::update_user_task::update_user_task)
            )
            .service(
                scope("/entries")
                    .service(entries::get_entries::get_entries)
                    .service(
                        scope("/supertasks")
                            .service(entries::create_entries::create_supertask)
                            .service(entries::update_entries::update_supertask)
                            .service(entries::delete_entries::delete_supertask)
                    )
                    .service(
                        scope("/tasks")
                            .service(entries::create_entries::create_task)
                            .service(entries::update_entries::update_task)
                            .service(entries::delete_entries::delete_task)
                    )
                    .service(
                        scope("/subtasks")
                            .service(entries::create_entries::create_subtask)
                            .service(entries::update_entries::update_subtask)
                            .service(entries::delete_entries::delete_subtask)
                    )
            )
            .service(
                scope("/rotations")
                    .service(rotations::create_rotation::create_rotation)
                    .service(rotations::delete_rotation::delete_rotation)
            )
    );
}