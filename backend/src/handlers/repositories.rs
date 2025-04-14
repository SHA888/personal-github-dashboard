use crate::{
    db::{DbPool, Repository},
    error::AppError,
    github::{GitHubAPIService, GitHubSyncService},
};
use actix_web::{web, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct ListResponse<T> {
    pub data: Vec<T>,
    pub meta: Meta,
}

#[derive(Serialize)]
pub struct Meta {
    pub total: Option<i64>,
    pub limit: i64,
    pub offset: i64,
}

#[derive(serde::Deserialize)]
pub struct QueryParams {
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
}

pub async fn list_repositories(
    pool: web::Data<DbPool>,
    query: web::Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    let limit = query.limit.unwrap_or(20);
    let offset = query.offset.unwrap_or(0);

    // Get total count
    let total = sqlx::query_scalar!("SELECT COUNT(*) FROM repositories")
        .fetch_one(&**pool)
        .await?;

    // Get repositories with explicit columns
    let repositories = sqlx::query_as!(
        Repository,
        r#"
        SELECT
            id, github_id, name, full_name, description, private, fork,
            html_url, clone_url, default_branch, language, stargazers_count,
            watchers_count, forks_count, open_issues_count, size, created_at,
            updated_at, pushed_at, last_synced_at
        FROM repositories
        ORDER BY name ASC NULLS LAST, full_name ASC
        LIMIT $1 OFFSET $2
        "#,
        limit,
        offset
    )
    .fetch_all(&**pool)
    .await?;

    Ok(HttpResponse::Ok().json(ListResponse {
        data: repositories,
        meta: Meta {
            total,
            limit,
            offset,
        },
    }))
}

pub async fn get_repository(
    pool: web::Data<DbPool>,
    id: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, AppError> {
    let repository = sqlx::query_as!(
        Repository,
        r#"
        SELECT
            id, github_id, name, full_name, description, private, fork,
            html_url, clone_url, default_branch, language, stargazers_count,
            watchers_count, forks_count, open_issues_count, size, created_at,
            updated_at, pushed_at, last_synced_at
        FROM repositories
        WHERE id = $1
        "#,
        id.into_inner()
    )
    .fetch_optional(&**pool)
    .await?;

    match repository {
        Some(repo) => Ok(HttpResponse::Ok().json(repo)),
        None => Err(AppError::NotFound("Repository not found".to_string())),
    }
}

pub async fn sync_repositories(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    let token = std::env::var("GITHUB_PERSONAL_ACCESS_TOKEN")
        .map_err(|_| AppError::InternalError("GITHUB_PERSONAL_ACCESS_TOKEN not set".to_string()))?;

    // Initialize services
    let api_service = GitHubAPIService::new(token);
    let sync_service = GitHubSyncService::new(pool.get_ref().clone());

    // Fetch repositories for the authenticated user
    let github_repos = api_service
        .list_my_repositories()
        .await
        .map_err(|e| AppError::GitHubError(format!("Failed to list repositories: {}", e)))?;

    // Sync each repository
    let mut synced_repos = Vec::new();
    let mut errors = Vec::new();

    for repo in github_repos {
        match sync_service.sync_repository(repo).await {
            Ok(repo) => synced_repos.push(repo),
            Err(e) => errors.push(e.to_string()),
        }
    }

    if errors.is_empty() {
        Ok(HttpResponse::Ok().json(synced_repos))
    } else {
        Ok(HttpResponse::InternalServerError()
            .json(serde_json::json!({ "synced": synced_repos, "errors": errors })))
    }
}
