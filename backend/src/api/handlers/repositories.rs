use crate::db::{DbPool, Repository};
use crate::error::AppError;
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

    // Get repositories with explicit columns matching the final struct definition
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
    // Get repository with explicit columns matching the final struct definition
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
