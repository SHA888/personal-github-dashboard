use crate::db::models::RepositoryMetrics;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[allow(clippy::too_many_arguments)]
pub async fn record_repository_metrics(
    pool: &PgPool,
    repository_id: Uuid,
    stargazers_count: i32,
    watchers_count: i32,
    forks_count: i32,
    open_issues_count: i32,
    open_pull_requests_count: i32,
    commit_count: i32,
    contributor_count: i32,
) -> Result<RepositoryMetrics, sqlx::Error> {
    let metrics = sqlx::query_as!(
        RepositoryMetrics,
        r#"
        INSERT INTO repository_metrics (
            repository_id,
            stargazers_count,
            watchers_count,
            forks_count,
            open_issues_count,
            open_pull_requests_count,
            commit_count,
            contributor_count,
            recorded_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#,
        repository_id,
        stargazers_count,
        watchers_count,
        forks_count,
        open_issues_count,
        open_pull_requests_count,
        commit_count,
        contributor_count,
        OffsetDateTime::now_utc()
    )
    .fetch_one(pool)
    .await?;

    Ok(metrics)
}

pub async fn get_repository_metrics(
    pool: &PgPool,
    repository_id: Uuid,
    start_time: OffsetDateTime,
    end_time: OffsetDateTime,
    limit: i64,
) -> Result<Vec<RepositoryMetrics>, sqlx::Error> {
    let metrics = sqlx::query_as!(
        RepositoryMetrics,
        r#"
        SELECT 
            id,
            repository_id,
            stargazers_count as "stargazers_count!: i32",
            watchers_count as "watchers_count!: i32",
            forks_count as "forks_count!: i32",
            open_issues_count as "open_issues_count!: i32",
            open_pull_requests_count as "open_pull_requests_count!: i32",
            commit_count as "commit_count!: i32",
            contributor_count as "contributor_count!: i32",
            recorded_at as "recorded_at!: OffsetDateTime"
        FROM repository_metrics
        WHERE repository_id = $1
        AND recorded_at BETWEEN $2 AND $3
        ORDER BY recorded_at DESC
        LIMIT $4
        "#,
        repository_id,
        start_time,
        end_time,
        limit
    )
    .fetch_all(pool)
    .await?;

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
