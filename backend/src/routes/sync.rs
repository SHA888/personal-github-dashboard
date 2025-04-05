use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use crate::analytics::Analytics;

#[derive(Debug, Deserialize)]
pub struct RepositoryPath {
    owner: String,
    repo: String,
}

pub fn configure_sync_routes() -> actix_web::Scope {
    web::scope("/sync")
        .route("/repository/{owner}/{repo}", web::post().to(sync_repository))
        .route("/organization/{org}", web::post().to(sync_organization))
}

async fn sync_repository(
    path: web::Path<RepositoryPath>,
    analytics: web::Data<Analytics>,
) -> impl Responder {
    match analytics.sync_repository(&path.owner, &path.repo).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Repository data synchronized successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to synchronize repository data",
            "details": e.to_string()
        }))
    }
}

async fn sync_organization(
    path: web::Path<String>,
    analytics: web::Data<Analytics>,
) -> impl Responder {
    let org = path.into_inner();
    
    match analytics.sync_organization(&org).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Organization data synchronized successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to synchronize organization data",
            "details": e.to_string()
        }))
    }
} 