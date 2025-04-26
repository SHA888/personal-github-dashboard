use crate::db::get_user_by_id_with_cache;
use crate::error::AppError;
use crate::handlers::activities::{create_activity, delete_activity, get_activity_by_id};
use crate::handlers::auth::{callback, login, pat_auth};
use crate::handlers::organizations::{
    create_organization, delete_organization, get_organization_by_id,
    update_organization_description,
};
use crate::handlers::repositories::{
    create_repository, delete_repository, get_repositories, get_repository_by_id,
    update_repository_description,
};
use crate::handlers::users::{create_user, delete_user, update_user_avatar};
use crate::utils::config::Config;
use crate::utils::jwt::validate_jwt;
use crate::utils::redis::RedisClient;
use actix_cors::Cors;
use actix_web::{dev::ServiceRequest, web, Error, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use sqlx::PgPool;
use uuid::Uuid;

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
                        get_repositories(pool, redis, query).await
                    },
                ),
            )
            .route(
                "/repositories/{repo_id}",
                web::get().to(get_repository_by_id),
            )
            .route("/repositories", web::post().to(create_repository))
            .route(
                "/repositories/{repo_id}/description",
                web::put().to(update_repository_description),
            )
            .route(
                "/repositories/{repo_id}",
                web::delete().to(delete_repository),
            )
            .route("/organizations", web::get().to(get_organizations))
            .route(
                "/organizations/{org_id}",
                web::get().to(get_organization_by_id),
            )
            .route("/organizations", web::post().to(create_organization))
            .route(
                "/organizations/{org_id}/description",
                web::put().to(update_organization_description),
            )
            .route(
                "/organizations/{org_id}",
                web::delete().to(delete_organization),
            )
            .route("/security", web::get().to(get_security_alerts))
            .route(
                "/activities/{activity_id}",
                web::get().to(get_activity_by_id),
            )
            .route("/activities", web::post().to(create_activity))
            .route(
                "/activities/{activity_id}",
                web::delete().to(delete_activity),
            )
            .route("/user/{user_id}", web::get().to(get_user_info))
            .route("/user", web::post().to(create_user))
            .route("/user/{user_id}/avatar", web::put().to(update_user_avatar))
            .route("/user/{user_id}", web::delete().to(delete_user)),
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

async fn get_user_info(
    pool: web::Data<PgPool>,
    redis: web::Data<RedisClient>,
    user_id: web::Path<Uuid>,
) -> HttpResponse {
    match get_user_by_id_with_cache(&pool, &redis, &user_id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB/Cache error: {}", e)),
    }
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
