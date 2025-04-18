use actix_web::web;

pub mod db;
pub mod error;
pub mod handlers;
pub mod models;
pub mod services;
pub mod utils;

pub fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.service(handlers::auth::github_login)
        .service(handlers::auth::github_callback)
        .service(handlers::auth::logout)
        .service(handlers::organizations::list_organizations)
        .service(handlers::organizations::sync_my_organizations)
        .service(handlers::repositories::list_repositories)
        .service(handlers::repositories::sync_repositories)
        .service(handlers::notifications::list_notifications)
        .service(handlers::notifications::get_notification)
        .service(handlers::notifications::mark_read)
        .service(handlers::notifications::get_settings)
        .service(handlers::notifications::update_settings);
}
