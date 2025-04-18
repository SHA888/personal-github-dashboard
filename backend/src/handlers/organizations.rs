#[allow(unused_imports)]
use crate::db::{DbPool, Organization};
use crate::{
    error::AppError, services::github_api::GitHubService, services::sync::GitHubSyncService,
};
use actix_web::{get, post, web, HttpResponse, Responder};
use futures::future::join_all;
use log::error;
use serde::Serialize;
use sqlx::PgPool;
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

#[derive(Debug, serde::Deserialize)]
pub struct QueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[get("/organizations")]
pub async fn list_organizations(
    pool: web::Data<PgPool>,
    query: web::Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    let limit = query.limit.unwrap_or(20);
    let offset = query.offset.unwrap_or(0);

    // Get total count
    let total = sqlx::query_scalar!("SELECT COUNT(*) FROM organizations")
        .fetch_one(&**pool)
        .await?;

    let organizations = sqlx::query_as!(
        Organization,
        r#"
        SELECT
            id,
            github_id,
            login as "login!: String",
            name,
            description,
            avatar_url,
            created_at as "created_at!: OffsetDateTime",
            updated_at as "updated_at!: OffsetDateTime"
        FROM organizations
        ORDER BY name ASC
        LIMIT $1 OFFSET $2
        "#,
        limit,
        offset
    )
    .fetch_all(&**pool)
    .await?;

    Ok(HttpResponse::Ok().json(ListResponse {
        data: organizations,
        meta: Meta {
            total,
            limit,
            offset,
        },
    }))
}

#[get("/organizations/{id}")]
pub async fn get_organization(
    pool: web::Data<DbPool>,
    id: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, AppError> {
    let organization = sqlx::query_as!(
        Organization,
        r#"
        SELECT
            id,
            github_id,
            login as "login!: String",
            name,
            description,
            avatar_url,
            created_at as "created_at!: OffsetDateTime",
            updated_at as "updated_at!: OffsetDateTime"
        FROM organizations
        WHERE id = $1
        "#,
        id.into_inner()
    )
    .fetch_optional(&**pool)
    .await?;

    match organization {
        Some(org) => Ok(HttpResponse::Ok().json(org)),
        None => Err(AppError::NotFound("Organization not found".to_string())),
    }
}

#[post("/organizations/{name}/sync")]
pub async fn sync_organization_by_name(
    pool: web::Data<DbPool>,
    org_name: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    // Get GitHub token from environment
    let token = std::env::var("GITHUB_PERSONAL_ACCESS_TOKEN")
        .map_err(|_| AppError::InternalError("GITHUB_PERSONAL_ACCESS_TOKEN not set".to_string()))?;

    // Initialize services
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let api_service = GitHubService::new(token, redis_url).await.map_err(|e| {
        error!("Failed to create GitHub service: {}", e);
        AppError::InternalError(format!("Failed to create GitHub service: {}", e))
    })?;
    let sync_service = GitHubSyncService::new(pool.get_ref().clone(), api_service.clone());

    // --- Authorization Check ---
    let authenticated_user = api_service.get_authenticated_user().await?;

    if &authenticated_user.login != org_name.as_ref() {
        return Err(AppError::Unauthorized(format!(
            "Forbidden: Token user ({}) does not match target organization ({})",
            authenticated_user.login, org_name
        )));
    }

    // Fetch organization from GitHub
    let github_org = api_service.get_organization(&org_name).await?;

    // Sync to database
    let organization = sync_service.sync_organization(github_org).await?;

    Ok(HttpResponse::Ok().json(organization))
}

#[post("/organizations/sync")]
pub async fn sync_my_organizations(
    pool: web::Data<PgPool>,
    github: web::Data<GitHubService>,
) -> Result<HttpResponse, AppError> {
    let sync_service = GitHubSyncService::new(pool.get_ref().clone(), github.get_ref().clone());
    sync_service.sync_user_organizations().await?;
    Ok(HttpResponse::Ok().finish())
}
