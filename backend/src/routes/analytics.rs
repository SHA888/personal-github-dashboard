use actix_web::{web, HttpResponse, Responder};
use crate::analytics::Analytics;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AnalyticsQuery {
    pub days: Option<i32>,
}

pub fn configure_analytics_routes() -> actix_web::Scope {
    web::scope("/analytics")
        .route("/repositories/{id}/activity", web::get().to(get_repository_activity))
        .route("/repositories/{id}/velocity", web::get().to(get_repository_velocity))
        .route("/repositories/{id}/growth", web::get().to(get_repository_growth))
        .route("/repositories/{id}/burndown/{milestone_id}", web::get().to(get_burndown_chart))
        .route("/repositories/{id}/release-cycle", web::get().to(get_release_cycle))
        .route("/users/{id}/contributions", web::get().to(get_user_contributions))
        .route("/organizations/{id}/stats", web::get().to(get_organization_stats))
        .route("/organizations/{id}/cross-team", web::get().to(get_cross_team_collaboration))
}

async fn get_repository_activity(
    path: web::Path<i32>,
    query: web::Query<AnalyticsQuery>,
    analytics: web::Data<Analytics>,
) -> impl Responder {
    let repository_id = path.into_inner();
    let days = query.days.unwrap_or(30);
    
    match analytics.get_repository_activity(repository_id, days).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to get repository activity",
            "details": e.to_string()
        }))
    }
}

async fn get_repository_velocity(
    path: web::Path<i32>,
    query: web::Query<AnalyticsQuery>,
    analytics: web::Data<Analytics>,
) -> impl Responder {
    let repository_id = path.into_inner();
    let days = query.days.unwrap_or(30);
    
    match analytics.get_repository_velocity(repository_id, days).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to get repository velocity",
            "details": e.to_string()
        }))
    }
}

async fn get_repository_growth(
    path: web::Path<i32>,
    query: web::Query<AnalyticsQuery>,
    analytics: web::Data<Analytics>,
) -> impl Responder {
    let repository_id = path.into_inner();
    let days = query.days.unwrap_or(30);
    
    match analytics.get_repository_growth(repository_id, days).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to get repository growth",
            "details": e.to_string()
        }))
    }
}

async fn get_burndown_chart(
    path: web::Path<(i32, i32)>,
    analytics: web::Data<Analytics>,
) -> impl Responder {
    let (repository_id, milestone_id) = path.into_inner();
    
    match analytics.get_burndown_chart(milestone_id).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to get burndown chart",
            "details": e.to_string()
        }))
    }
}

async fn get_release_cycle(
    path: web::Path<i32>,
    analytics: web::Data<Analytics>,
) -> impl Responder {
    let repository_id = path.into_inner();
    
    match analytics.get_release_cycle_analysis(repository_id).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to get release cycle analysis",
            "details": e.to_string()
        }))
    }
}

async fn get_user_contributions(
    path: web::Path<i32>,
    query: web::Query<AnalyticsQuery>,
    analytics: web::Data<Analytics>,
) -> impl Responder {
    let user_id = path.into_inner();
    let days = query.days.unwrap_or(30);
    
    match analytics.get_user_contributions(user_id, days).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to get user contributions",
            "details": e.to_string()
        }))
    }
}

async fn get_organization_stats(
    path: web::Path<i32>,
    query: web::Query<AnalyticsQuery>,
    analytics: web::Data<Analytics>,
) -> impl Responder {
    let organization_id = path.into_inner();
    let days = query.days.unwrap_or(30);
    
    match analytics.get_organization_stats(organization_id, days).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to get organization stats",
            "details": e.to_string()
        }))
    }
}

async fn get_cross_team_collaboration(
    path: web::Path<i32>,
    query: web::Query<AnalyticsQuery>,
    analytics: web::Data<Analytics>,
) -> impl Responder {
    let organization_id = path.into_inner();
    let days = query.days.unwrap_or(30);
    
    match analytics.get_cross_team_collaboration(organization_id, days).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to get cross-team collaboration data",
            "details": e.to_string()
        }))
    }
} 