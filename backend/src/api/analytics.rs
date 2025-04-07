use crate::AppState;
use actix_web::{web, HttpResponse};
use serde_json::json;

pub fn configure_analytics_routes(cfg: &mut web::ServiceConfig, app_state: &web::Data<AppState>) {
    cfg.service(
        web::scope("/api/analytics")
            .app_data(app_state.clone())
            .route(
                "/repository/{owner}/{repo}",
                web::get().to(get_repository_analytics),
            )
            .route(
                "/trends/{owner}/{repo}",
                web::get().to(get_repository_trends),
            ),
    );
}

async fn get_repository_analytics(
    path: web::Path<(String, String)>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let (owner, repo) = path.into_inner();
    match app_state
        .analytics
        .get_repository_analytics(&owner, &repo)
        .await
    {
        Ok(analytics) => HttpResponse::Ok().json(analytics),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to get repository analytics: {}", e)
        })),
    }
}

async fn get_repository_trends(
    path: web::Path<(String, String)>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let (owner, repo) = path.into_inner();
    match app_state
        .analytics
        .get_repository_trends(&owner, &repo)
        .await
    {
        Ok(trends) => HttpResponse::Ok().json(trends),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to get repository trends: {}", e)
        })),
    }
}
