use crate::AppState;
use actix_web::{web, HttpResponse};
use serde_json::json;

pub fn configure_sync_routes(cfg: &mut web::ServiceConfig, app_state: &web::Data<AppState>) {
    cfg.service(web::scope("/api/sync").app_data(app_state.clone()).route(
        "/repository/{owner}/{repo}",
        web::post().to(sync_repository),
    ));
}

async fn sync_repository(
    path: web::Path<(String, String)>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let (owner, repo) = path.into_inner();

    match app_state.github.sync_repository(&owner, &repo).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": format!("Successfully synced {}/{}", owner, repo)
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to sync repository: {}", e)
        })),
    }
}
