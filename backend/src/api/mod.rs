use crate::AppState;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

mod analytics;
mod repository;
mod sync;

pub use analytics::configure_analytics_routes;
pub use repository::configure_repository_routes;
pub use sync::configure_sync_routes;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn add_repository(
    data: web::Json<AddRepositoryRequest>,
    state: web::Data<AppState>,
) -> actix_web::HttpResponse {
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

pub fn configure_routes(cfg: &mut web::ServiceConfig, app_state: &web::Data<AppState>) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(health_check))
            .configure(|cfg| configure_analytics_routes(cfg, app_state))
            .configure(|cfg| configure_repository_routes(cfg, app_state))
            .configure(|cfg| configure_sync_routes(cfg, app_state)),
    );
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(json!({ "status": "ok" }))
}
