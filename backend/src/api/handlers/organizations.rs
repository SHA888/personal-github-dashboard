use crate::db::{DbPool, Organization};
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

pub async fn list_organizations(
    pool: web::Data<DbPool>,
    query: web::Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    let limit = query.limit.unwrap_or(20);
    let offset = query.offset.unwrap_or(0);

    // Get total count
    let total = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM organizations"
    )
    .fetch_one(&**pool)
    .await?;

    // Get organizations
    let organizations = sqlx::query_as!(
        Organization,
        r#"
        SELECT * FROM organizations
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
        SELECT * FROM organizations
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

#[derive(serde::Deserialize)]
pub struct QueryParams {
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
} 