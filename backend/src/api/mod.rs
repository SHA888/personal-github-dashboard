use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

mod analytics;
mod repository;
mod sync;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddRepositoryRequest {
    owner: String,
    name: String,
}

#[derive(Debug, Serialize)]
struct RepositoryResponse {
    id: i32,
    owner: String,
    name: String,
}

pub async fn add_repository(
    data: web::Json<AddRepositoryRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    match sqlx::query!(
        r#"
        INSERT INTO repositories (owner, name)
        VALUES ($1, $2)
        ON CONFLICT (owner, name) DO NOTHING
        RETURNING id, owner, name
        "#,
        data.owner,
        data.name
    )
    .fetch_optional(&state.pool)
    .await
    {
        Ok(Some(repo)) => HttpResponse::Ok().json(RepositoryResponse {
            id: repo.id,
            owner: repo.owner,
            name: repo.name,
        }),
        Ok(None) => HttpResponse::Ok().json(json!({
            "message": "Repository already exists"
        })),
        Err(e) => {
            eprintln!("Failed to add repository: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to add repository"
            }))
        }
    }
}

pub fn configure_routes(cfg: &mut web::ServiceConfig, state: &web::Data<AppState>) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(health_check))
            .configure(|cfg| {
                crate::api::analytics::configure_analytics_routes(cfg, state);
                crate::api::repository::configure_repository_routes(cfg, state);
            })
            .app_data(state.clone())
            .service(web::resource("/repositories").route(web::post().to(add_repository))),
    );
}

pub fn configure_sync_routes(
    cfg: &mut web::ServiceConfig,
    github: std::sync::Arc<crate::github::GitHubService>,
) {
    sync::configure_sync_routes(cfg, github);
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({
            "status": "ok",
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
}
