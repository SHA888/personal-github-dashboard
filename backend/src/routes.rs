use crate::handlers;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // Health check
            .route("/health", web::get().to(handlers::health::health_check))
            // Auth Routes
            .route("/auth/github", web::get().to(handlers::auth::github_login))
            .route(
                "/auth/github/callback",
                web::get().to(handlers::auth::github_callback),
            )
            .route("/auth/logout", web::post().to(handlers::auth::logout))
            .route("/auth/test", web::get().to(handlers::auth::test_auth))
            .route("/auth/me", web::get().to(handlers::auth::get_current_user))
            // User Routes
            .route("/users", web::get().to(handlers::users::list_users))
            .route("/users/{id}", web::get().to(handlers::users::get_user))
            // Organization Routes
            .route(
                "/orgs",
                web::get().to(handlers::organizations::list_organizations),
            )
            .route(
                "/orgs/{id}",
                web::get().to(handlers::organizations::get_organization),
            )
            .route(
                "/orgs/sync",
                web::post().to(handlers::organizations::sync_my_organizations),
            )
            .route(
                "/orgs/{name}/sync",
                web::post().to(handlers::organizations::sync_organization_by_name),
            )
            // Repository Routes
            .route(
                "/repos",
                web::get().to(handlers::repositories::list_repositories),
            )
            .route(
                "/repos/{id}",
                web::get().to(handlers::repositories::get_repository),
            )
            .route(
                "/repos/sync",
                web::post().to(handlers::repositories::sync_repositories),
            )
            // Notification Routes
            .route(
                "/notifications",
                web::get().to(handlers::notifications::list_notifications),
            )
            .route(
                "/notifications/{id}",
                web::get().to(handlers::notifications::get_notification),
            )
            .route(
                "/notifications/{id}/read",
                web::post().to(handlers::notifications::mark_read),
            )
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
