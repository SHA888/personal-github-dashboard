#[allow(unused_imports)]
use crate::db::{DbPool, Organization};
use crate::{error::AppError, github::GitHubSyncService, services::github_api::GitHubService};
use actix_web::{web, HttpResponse, Responder};
use futures::future::join_all;
use log::error;
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

pub async fn list_organizations(
    pool: web::Data<DbPool>,
    query: web::Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    let limit = query.limit.unwrap_or(20);
    let offset = query.offset.unwrap_or(0);

    // Get total count
    let total = sqlx::query_scalar!("SELECT COUNT(*) FROM organizations")
        .fetch_one(&**pool)
        .await?;

    // Get organizations
    let organizations = sqlx::query_as!(
        Organization,
        r#"
        SELECT
            id,
            github_id,
            login,
            name,
            description,
            avatar_url,
            html_url,
            created_at,
            updated_at,
            last_synced_at
        FROM organizations
        ORDER BY name ASC NULLS LAST, login ASC
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
            login,
            name,
            description,
            avatar_url,
            html_url,
            created_at,
            updated_at,
            last_synced_at
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
    let sync_service = GitHubSyncService::new(pool.get_ref().clone());

    // --- Authorization Check ---
    // Corrected type annotation: Author
    let authenticated_user: octocrab::models::Author =
        api_service.get_authenticated_user().await.map_err(|e| {
            AppError::GitHubError(format!("Failed to get authenticated user: {}", e))
        })?;

    // Author has a login field
    if &authenticated_user.login != org_name.as_ref() {
        return Err(AppError::InternalError(format!(
            "Forbidden: Token user ({}) does not match target organization ({}). Simple check failed.",
            authenticated_user.login,
            org_name
        )));
    }
    // --- End Authorization Check ---

    // Fetch organization from GitHub
    let github_org = api_service
        .get_organization(&org_name)
        .await
        .map_err(|e| AppError::GitHubError(e.to_string()))?;

    // Sync to database
    let organization = sync_service
        .sync_organization(github_org)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(organization))
}

pub async fn sync_my_organizations(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    let token = std::env::var("GITHUB_PERSONAL_ACCESS_TOKEN")
        .map_err(|_| AppError::InternalError("GITHUB_PERSONAL_ACCESS_TOKEN not set".to_string()))?;

    // Initialize services
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let api_service = GitHubService::new(token, redis_url).await.map_err(|e| {
        error!("Failed to create GitHub service: {}", e);
        AppError::InternalError(format!("Failed to create GitHub service: {}", e))
    })?;
    let sync_service = GitHubSyncService::new(pool.get_ref().clone());

    // Fetch organizations for the authenticated user
    let github_orgs = api_service
        .list_my_organizations()
        .await
        .map_err(|e| AppError::GitHubError(format!("Failed to list user organizations: {}", e)))?;

    // Sync each organization concurrently
    let sync_futures = github_orgs.into_iter().map(|org| {
        let sync_service = sync_service.clone(); // Clone service for concurrent use
        async move { sync_service.sync_organization(org).await }
    });

    let results: Vec<Result<Organization, AppError>> = join_all(sync_futures).await;

    // Collect results (or handle errors)
    let mut synced_orgs = Vec::new();
    let mut errors = Vec::new();
    for result in results {
        match result {
            Ok(org) => synced_orgs.push(org),
            Err(e) => errors.push(e.to_string()),
        }
    }

    if errors.is_empty() {
        Ok(HttpResponse::Ok().json(synced_orgs))
    } else {
        // Return an error response if any sync failed
        Ok(HttpResponse::InternalServerError()
            .json(serde_json::json!({ "synced": synced_orgs, "errors": errors })))
    }
}

#[derive(serde::Deserialize)]
pub struct QueryParams {
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
}
