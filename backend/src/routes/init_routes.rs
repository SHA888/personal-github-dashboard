use crate::handlers::auth::{callback, login, pat_auth};
use crate::handlers::repositories;
use actix_cors::Cors;
use actix_web::{dev::ServiceRequest, web, Error, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use personal_github_dashboard::error::AppError;
use personal_github_dashboard::utils::config::Config;
use personal_github_dashboard::utils::jwt::validate_jwt;
use personal_github_dashboard::utils::redis::RedisClient;
use sqlx::PgPool;

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

    // API endpoints with JWT guard and JSON error handler
    let auth = HttpAuthentication::bearer(validator);
    cfg.service(
        web::scope("/api")
            .wrap(auth)
            .route(
                "/repositories",
                web::get().to(
                    |pool: web::Data<PgPool>, redis: web::Data<RedisClient>, query| async move {
                        repositories::get_repositories(pool, redis, query).await
                    },
                ),
            )
            .route("/organizations", web::get().to(get_organizations))
            .route("/security", web::get().to(get_security_alerts))
            .route("/user", web::get().to(get_user_info)),
    );
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().body("OK")
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

async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let config = Config::from_env();
    match validate_jwt(credentials.token(), &config.jwt_secret) {
        Ok(_) => Ok(req),
        Err(_) => Err((
            AppError::Unauthorized("Invalid or missing token".into()).into(),
            req,
        )),
    }
}
