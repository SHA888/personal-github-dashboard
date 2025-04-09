use crate::db::{DbPool, User};
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

pub async fn list_users(
    pool: web::Data<DbPool>,
    query: web::Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    let limit = query.limit.unwrap_or(20);
    let offset = query.offset.unwrap_or(0);

    // Get total count
    let total = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(&**pool)
        .await?;

    // Get users
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM users
        ORDER BY login ASC
        LIMIT $1 OFFSET $2
        "#,
        limit,
        offset
    )
    .fetch_all(&**pool)
    .await?;

    Ok(HttpResponse::Ok().json(ListResponse {
        data: users,
        meta: Meta {
            total,
            limit,
            offset,
        },
    }))
}

pub async fn get_user(
    pool: web::Data<DbPool>,
    id: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM users
        WHERE id = $1
        "#,
        id.into_inner()
    )
    .fetch_optional(&**pool)
    .await?;

    match user {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Err(AppError::NotFound("User not found".to_string())),
    }
}
