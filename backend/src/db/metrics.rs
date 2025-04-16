use crate::db::models::RepositoryMetrics;
use crate::db::DbPool;
use crate::error::AppError;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[allow(clippy::too_many_arguments)]
pub async fn record_repository_metrics(
    pool: &DbPool,
    repository_id: Uuid,
    stargazers_count: i32,
    watchers_count: i32,
    forks_count: i32,
    open_issues_count: i32,
    open_pull_requests_count: i32,
    commit_count: i32,
    contributor_count: i32,
) -> Result<RepositoryMetrics, AppError> {
    let metrics = sqlx::query_as!(
        RepositoryMetrics,
        r#"
        INSERT INTO repository_metrics (
            repository_id, stargazers_count, watchers_count, forks_count,
            open_issues_count, open_pull_requests_count, commit_count, contributor_count
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, repository_id, stargazers_count, watchers_count, forks_count,
                  open_issues_count, open_pull_requests_count, commit_count, contributor_count, recorded_at
        "#,
        repository_id,
        stargazers_count,
        watchers_count,
        forks_count,
        open_issues_count,
        open_pull_requests_count,
        commit_count,
        contributor_count
    )
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(metrics)
}

pub async fn get_repository_metrics(
    pool: &DbPool,
    repository_id: Uuid,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    limit: i64,
) -> Result<Vec<RepositoryMetrics>, AppError> {
    let metrics = sqlx::query_as!(
        RepositoryMetrics,
        r#"
        SELECT id, repository_id, stargazers_count, watchers_count, forks_count,
               open_issues_count, open_pull_requests_count, commit_count, contributor_count, recorded_at
        FROM repository_metrics
        WHERE repository_id = $1 AND recorded_at BETWEEN $2 AND $3
        ORDER BY recorded_at DESC
        LIMIT $4
        "#,
        repository_id,
        start_time,
        end_time,
        limit
    )
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(metrics)
}

// TODO: Implement aggregation query using the RepositoryMetricsAggregation struct
// pub use crate::models::RepositoryMetricsAggregation; // Import needed here
// pub async fn get_aggregated_repository_metrics(
//     pool: &DbPool,
//     repository_id: Uuid,
//     period: &str, // e.g., 'daily', 'weekly', 'monthly'
//     start_date: DateTime<Utc>,
//     end_date: DateTime<Utc>,
// ) -> Result<RepositoryMetricsAggregation, AppError> {
//     // SQL query with window functions or GROUP BY to calculate aggregations
//     // ...
// }
