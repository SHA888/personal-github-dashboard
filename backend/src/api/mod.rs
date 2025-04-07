use crate::AppState;
use actix_web::web;

mod analytics;
mod repository;
mod sync;

pub fn configure_routes(cfg: &mut web::ServiceConfig, app_state: &web::Data<AppState>) {
    cfg.service(web::scope("/api").configure(|cfg| {
        crate::api::analytics::configure_analytics_routes(cfg, app_state);
        crate::api::repository::configure_repository_routes(cfg, app_state);
    }));
}

pub fn configure_sync_routes(
    cfg: &mut web::ServiceConfig,
    github: std::sync::Arc<crate::github::GitHubService>,
) {
    sync::configure_sync_routes(cfg, github);
}
