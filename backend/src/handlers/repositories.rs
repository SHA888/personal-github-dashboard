use crate::{
    db::{DbPool, Repository},
    error::AppError,
    services::github_api::GitHubService,
    services::sync::GitHubSyncService,
};
use actix_web::{get, post, web, HttpResponse};
use log::error;
use serde::Serialize;
use time::OffsetDateTime;

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

#[get("/repositories")]
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
            id, github_id, owner_id, name as "name!: String", description, is_private as "is_private!: bool", is_fork as "is_fork!: bool",
            created_at as "created_at!: OffsetDateTime", updated_at as "updated_at!: OffsetDateTime"
        FROM repositories
        ORDER BY name ASC
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

#[get("/repositories/{id}")]
pub async fn get_repository(
    pool: web::Data<DbPool>,
    id: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, AppError> {
    let repository = sqlx::query_as!(
        Repository,
        r#"
        SELECT
            id, github_id, owner_id, name as "name!: String", description, is_private as "is_private!: bool", is_fork as "is_fork!: bool",
            created_at as "created_at!: OffsetDateTime", updated_at as "updated_at!: OffsetDateTime"
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

#[post("/repositories/sync")]
pub async fn sync_repositories(
    pool: web::Data<DbPool>,
    github: web::Data<GitHubService>,
) -> Result<HttpResponse, AppError> {
    let sync_service = GitHubSyncService::new(pool.get_ref().clone(), github.get_ref().clone());
    sync_service.sync_user_repositories().await?;
    Ok(HttpResponse::Ok().finish())
}
