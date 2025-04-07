use crate::AppState;
use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AnalyticsQuery {
    pub days: Option<i32>,
}

pub fn configure_analytics_routes(cfg: &mut web::ServiceConfig, app_state: &web::Data<AppState>) {
    cfg.service(
        web::scope("/analytics")
            .service(
                web::resource("/repository/{owner}/{repo}/activity")
                    .route(web::get().to(get_repository_activity)),
            )
            .service(
                web::resource("/repository/{owner}/{repo}/trends")
                    .route(web::get().to(get_repository_trends)),
            )
            .app_data(web::Data::new(app_state.clone())),
    );
}

async fn get_repository_activity(
    path: web::Path<(String, String)>,
    query: web::Query<AnalyticsQuery>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let (owner, repo) = path.into_inner();
    let days = query.days.unwrap_or(30);

    match app_state
        .analytics
        .get_repository_activity(&owner, &repo, days)
        .await
    {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

async fn get_repository_trends(
    path: web::Path<(String, String)>,
    query: web::Query<AnalyticsQuery>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let (owner, repo) = path.into_inner();
    let days = query.days.unwrap_or(30);

    match app_state
        .analytics
        .get_repository_trends(&owner, &repo, days)
        .await
    {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}
