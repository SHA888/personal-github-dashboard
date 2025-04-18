use crate::handlers;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // Health check
            .route("/health", web::get().to(handlers::health::health_check))
            // Auth Routes
            .service(handlers::auth::github_login)
            .service(handlers::auth::github_callback)
            .service(handlers::auth::logout)
            .route("/auth/test", web::get().to(handlers::auth::test_auth))
            .route("/auth/me", web::get().to(handlers::auth::get_current_user))
            // User Routes
            .route("/users", web::get().to(handlers::users::list_users))
            .route("/users/{id}", web::get().to(handlers::users::get_user))
            // Organization Routes
            .service(handlers::organizations::list_organizations)
            .service(handlers::organizations::get_organization)
            .service(handlers::organizations::sync_my_organizations)
            .service(handlers::organizations::sync_organization_by_name)
            // Repository Routes
            .service(handlers::repositories::list_repositories)
            .service(handlers::repositories::get_repository)
            .service(handlers::repositories::sync_repositories)
            // Notification Routes
            .service(handlers::notifications::list_notifications)
            .service(handlers::notifications::get_notification)
            .service(handlers::notifications::mark_read)
            // Metrics Routes
            .route(
                "/metrics/user",
                web::get().to(handlers::metrics::user_metrics),
            )
            .route(
                "/metrics/repository/{id}",
                web::get().to(handlers::metrics::repository_metrics),
            ),
    );
}
