use actix_web::{web, HttpResponse};
use serde::Deserialize;
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use crate::db::DbPool;
use crate::error::AppError;

#[derive(Deserialize)]
pub struct RepositoryMetricsQuery {
    start_time: Option<OffsetDateTime>,
    end_time: Option<OffsetDateTime>,
    limit: Option<i64>,
}

pub async fn repository_metrics(
    pool: web::Data<DbPool>,
    repository_id: web::Path<Uuid>,
    query: web::Query<RepositoryMetricsQuery>,
) -> Result<HttpResponse, AppError> {
    let start_time = query.start_time.unwrap_or_else(|| {
        OffsetDateTime::now_utc() - Duration::days(30) // Default to last 30 days
    });
    let end_time = query.end_time.unwrap_or_else(OffsetDateTime::now_utc);
    let limit = query.limit.unwrap_or(100); // Default to 100 records

    let metrics = crate::db::metrics::get_repository_metrics(
        &pool,
        repository_id.into_inner(),
        start_time,
        end_time,
        limit,
    )
    .await?;

    Ok(HttpResponse::Ok().json(metrics))
}

// TODO: Implement user metrics endpoint
pub async fn user_metrics() -> HttpResponse {
    HttpResponse::NotImplemented().finish()
}
