use crate::handlers::auth::{callback, login, pat_auth};
use actix_cors::Cors;
use actix_web::{web, HttpResponse};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    // Health check
    cfg.route("/health", web::get().to(health));

    // Authentication
    cfg.service(
        web::scope("/auth")
            .wrap(Cors::default().supports_credentials())
            .route("/login", web::get().to(login))
            .route("/callback", web::get().to(callback))
            .route("/pat", web::post().to(pat_auth)),
    );

    // API endpoints
    cfg.service(
        web::scope("/api")
            .route("/repositories", web::get().to(get_repositories))
            .route("/organizations", web::get().to(get_organizations))
            .route("/security", web::get().to(get_security_alerts))
            .route("/user", web::get().to(get_user_info)),
    );
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

async fn get_repositories() -> HttpResponse {
    HttpResponse::NotImplemented().finish()
}

async fn get_organizations() -> HttpResponse {
    HttpResponse::NotImplemented().finish()
}

async fn get_security_alerts() -> HttpResponse {
    HttpResponse::NotImplemented().finish()
}

async fn get_user_info() -> HttpResponse {
    HttpResponse::NotImplemented().finish()
}
