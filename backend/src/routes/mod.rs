// Routes module for API endpoint definitions

pub mod auth;
pub mod metrics;
pub mod notifications;
pub mod repositories;

use actix_web::web;

// Configure routes for the application
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(auth::configure)
            .configure(repositories::configure)
            .configure(notifications::configure)
            .configure(metrics::configure),
    );
}
