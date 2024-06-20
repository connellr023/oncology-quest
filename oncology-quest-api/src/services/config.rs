use super::*;
use actix_web::web::{scope, ServiceConfig};
use actix_governor::{governor::{clock::QuantaInstant, middleware::NoOpMiddleware}, Governor, GovernorConfigBuilder, PeerIpKeyExtractor};

#[cfg(feature = "monolith")]
use actix_files::Files;

fn governor(per_second: u64, burst_size: u32) -> Governor<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>> {
    let cfg = match cfg!(feature = "production") {
        true => GovernorConfigBuilder::default()
            .per_second(per_second)
            .burst_size(burst_size)
            .finish()
            .unwrap(),
        false => GovernorConfigBuilder::default()
            .permissive(true)
            .finish()
            .unwrap()
    };
    
    Governor::new(&cfg)
}

/// Configures the services for the application.
/// 
/// # Arguments
/// 
/// * `cfg` - The service configuration to configure.
pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api")
            .service(healthcheck::healthcheck)
            .service(
                scope("/users")
                    .service(users::register_user::register_user)
                        .wrap(governor(4, 2))
                    .service(users::login_user::login_user)
                        .wrap(governor(4, 3))
                    .service(users::logout_user::logout_user)       
                    .service(users::reset_user_password::reset_password)
                        .wrap(governor(3, 2))
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
                    .service(tasks::get_user_tasks::get_own_tasks)
                        .wrap(governor(1, 15))
                    .service(tasks::get_user_tasks::get_user_tasks)
            )
            .service(
                scope("/entries")
                    .service(entries::get_entries::get_entries)
                        .wrap(governor(1, 15))
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

    #[cfg(feature = "monolith")]
    cfg.service(
        Files::new("/", "./dist")
            .index_file("index.html")
    );
}