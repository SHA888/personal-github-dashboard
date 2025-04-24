use crate::db::{
    create_repository_with_cache, delete_repository_with_cache, get_repository_by_id_with_cache,
    update_repository_description_with_cache,
};
use crate::error::AppError;
use crate::utils::config::Config;
use crate::utils::redis::RedisClient;
use actix_web::{web, HttpResponse};
use octocrab::Octocrab;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct RepositoryInfo {
    pub name: String,
    pub description: Option<String>,
    pub html_url: String,
    pub stargazers_count: Option<u32>,
    pub forks_count: Option<u32>,
    pub open_issues_count: Option<u32>,
}

#[derive(Deserialize)]
pub struct RepoParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

#[derive(Deserialize)]
pub struct CreateRepositoryRequest {
    pub org_id: Option<Uuid>,
    pub owner_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_private: bool,
}

#[derive(Deserialize)]
pub struct UpdateRepositoryDescriptionRequest {
    pub description: Option<String>,
}

pub async fn get_repositories(
    _pool: web::Data<PgPool>,
    redis_client: web::Data<RedisClient>,
    query: web::Query<RepoParams>,
) -> Result<HttpResponse, AppError> {
    let cache_key = format!(
        "user_repos:{}:{}",
        query.page.unwrap_or(1),
        query.per_page.unwrap_or(10)
    );
    // Try to get cached value
    let cache_result: redis::RedisResult<Option<String>> =
        redis_client.get::<String>(&cache_key).await;
    if let Ok(Some(cached)) = cache_result {
        if let Ok(repos) = serde_json::from_str::<Vec<RepositoryInfo>>(&cached) {
            return Ok(HttpResponse::Ok().json(repos));
        }
    }

    // Fallback to fetching from GitHub (existing logic)
    let config = Config::from_env();
    let octocrab = Octocrab::builder()
        .personal_token(config.github_personal_access_token.clone())
        .build()
        .map_err(|e| AppError::InternalError(e.to_string()))?;

    let page_num = query.page.unwrap_or(1).clamp(1, 255) as u8;
    let per_page_num = query.per_page.unwrap_or(10).clamp(1, 255) as u8;

    let page = octocrab
        .current()
        .list_repos_for_authenticated_user()
        .page(page_num)
        .per_page(per_page_num)
        .send()
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;

    let repos = page
        .items
        .into_iter()
        .map(|r| RepositoryInfo {
            name: r.name,
            description: r.description,
            html_url: r.html_url.map(|u| u.to_string()).unwrap_or_default(),
            stargazers_count: r.stargazers_count,
            forks_count: r.forks_count,
            open_issues_count: r.open_issues_count,
        })
        .collect::<Vec<_>>();

    // Cache the result
    if let Ok(json) = serde_json::to_string(&repos) {
        redis_client
            .set(&cache_key, json, 60 * 5)
            .await
            .map_err(|e| AppError::InternalError(format!("Redis error: {}", e)))?;
    }

    Ok(HttpResponse::Ok().json(repos))
}

pub async fn get_repository_by_id(
    pool: web::Data<PgPool>,
    redis: web::Data<RedisClient>,
    repo_id: web::Path<Uuid>,
) -> HttpResponse {
    match get_repository_by_id_with_cache(&pool, &redis, &repo_id).await {
        Ok(Some(repo)) => HttpResponse::Ok().json(repo),
        Ok(None) => HttpResponse::NotFound().body("Repository not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB/Cache error: {}", e)),
    }
}

pub async fn create_repository(
    pool: web::Data<PgPool>,
    redis: web::Data<RedisClient>,
    req: web::Json<CreateRepositoryRequest>,
) -> HttpResponse {
    match create_repository_with_cache(
        &pool,
        &redis,
        req.org_id.as_ref(),
        &req.owner_id,
        &req.name,
        req.description.as_deref(),
        req.is_private,
    )
    .await
    {
        Ok(repo) => HttpResponse::Created().json(repo),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB/Cache error: {}", e)),
    }
}

pub async fn update_repository_description(
    pool: web::Data<PgPool>,
    redis: web::Data<RedisClient>,
    repo_id: web::Path<Uuid>,
    req: web::Json<UpdateRepositoryDescriptionRequest>,
) -> HttpResponse {
    match update_repository_description_with_cache(
        &pool,
        &redis,
        &repo_id,
        req.description.as_deref(),
    )
    .await
    {
        Ok(repo) => HttpResponse::Ok().json(repo),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB/Cache error: {}", e)),
    }
}

pub async fn delete_repository(
    pool: web::Data<PgPool>,
    redis: web::Data<RedisClient>,
    repo_id: web::Path<Uuid>,
) -> HttpResponse {
    match delete_repository_with_cache(&pool, &redis, &repo_id).await {
        Ok(affected) if affected > 0 => HttpResponse::NoContent().finish(),
        Ok(_) => HttpResponse::NotFound().body("Repository not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB/Cache error: {}", e)),
    }
}
