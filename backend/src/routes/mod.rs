use actix_web::web;
use crate::AppState;

mod analytics;
mod sync;
mod repository;

pub fn configure_routes(cfg: &mut web::ServiceConfig, app_state: &web::Data<AppState>) {
    cfg.service(
        web::scope("/api")
            .configure(|cfg| {
                crate::routes::analytics::configure_analytics_routes(cfg, app_state);
                crate::routes::repository::configure_repository_routes(cfg, app_state);
            })
    );
}

pub fn configure_sync_routes(cfg: &mut web::ServiceConfig, github: std::sync::Arc<crate::services::github::GitHubService>) {
    sync::configure_sync_routes(cfg, github);
} 