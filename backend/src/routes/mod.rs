use actix_web::web;
use crate::AppState;

mod analytics;

pub fn configure_routes(cfg: &mut web::ServiceConfig, app_state: &web::Data<AppState>) {
    cfg.service(
        web::scope("/api")
            .configure(|cfg| {
                crate::routes::analytics::configure_analytics_routes(cfg, app_state);
            })
    );
} 