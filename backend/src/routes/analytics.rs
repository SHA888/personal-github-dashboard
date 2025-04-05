use actix_web::{web, HttpResponse, Responder};
use crate::analytics::Analytics;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AnalyticsQuery {
    pub days: Option<i32>,
}

pub fn configure_analytics_routes() -> actix_web::Scope {
    web::scope("/analytics")
        .route("/repository/{owner}/{repo}/activity", web::get().to(get_repository_activity))
        .route("/repository/{owner}/{repo}/health", web::get().to(get_repository_health))
        .route("/repository/{owner}/{repo}/quality", web::get().to(get_code_quality_metrics))
        .route("/user/{username}/contributions", web::get().to(get_user_contributions))
        .route("/organization/{org}/stats", web::get().to(get_organization_stats))
        .route("/organization/{org}/team-performance", web::get().to(get_team_performance))
}

#[derive(Debug, Deserialize)]
pub struct RepositoryPath {
    owner: String,
    repo: String,
}

async fn get_repository_activity(
    path: web::Path<RepositoryPath>,
    query: web::Query<AnalyticsQuery>,
    analytics: web::Data<Analytics>,
) -> impl Responder {
    let days = query.days.unwrap_or(30);
    
    match analytics.get_repository_activity(&path.owner, &path.repo, days).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to get repository activity",
            "details": e.to_string()
        }))
    }
}

async fn get_repository_health(
    path: web::Path<RepositoryPath>,
    analytics: web::Data<Analytics>,
) -> impl Responder {
    match analytics.get_repository_health(&path.owner, &path.repo).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to get repository health",
            "details": e.to_string()
        }))
    }
}

async fn get_code_quality_metrics(
    path: web::Path<RepositoryPath>,
    analytics: web::Data<Analytics>,
) -> impl Responder {
    match analytics.get_code_quality_metrics(&path.owner, &path.repo).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to get code quality metrics",
            "details": e.to_string()
        }))
    }
}

async fn get_user_contributions(
    path: web::Path<String>,
    query: web::Query<AnalyticsQuery>,
    analytics: web::Data<Analytics>,
) -> impl Responder {
    let username = path.into_inner();
    let days = query.days.unwrap_or(30);
    
    match analytics.get_user_contributions(&username, days).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to get user contributions",
            "details": e.to_string()
        }))
    }
}

async fn get_organization_stats(
    path: web::Path<String>,
    query: web::Query<AnalyticsQuery>,
    analytics: web::Data<Analytics>,
) -> impl Responder {
    let org = path.into_inner();
    let days = query.days.unwrap_or(30);
    
    match analytics.get_organization_stats(&org, days).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to get organization stats",
            "details": e.to_string()
        }))
    }
}

async fn get_team_performance(
    path: web::Path<String>,
    query: web::Query<AnalyticsQuery>,
    analytics: web::Data<Analytics>,
) -> impl Responder {
    let org = path.into_inner();
    let days = query.days.unwrap_or(30);
    
    match analytics.get_team_performance(&org, days).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to get team performance data",
            "details": e.to_string()
        }))
    }
} 