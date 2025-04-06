use actix_web::{web, HttpResponse};
use crate::services::github::GitHubService;
use std::sync::Arc;

pub fn configure_sync_routes(cfg: &mut web::ServiceConfig, github: Arc<GitHubService>) {
    cfg.service(
        web::resource("/sync/repository/{owner}/{repo}")
            .route(web::post().to(sync_repository))
            .app_data(web::Data::new(github)),
    );
}

async fn sync_repository(
    path: web::Path<(String, String)>,
    github: web::Data<Arc<GitHubService>>,
) -> HttpResponse {
    let (owner, repo) = path.into_inner();
    
    match github.sync_repository(&owner, &repo).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "message": format!("Successfully synced {}/{}", owner, repo)
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": format!("Failed to sync repository: {}", e)
        })),
    }
} 