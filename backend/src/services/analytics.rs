use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::PgPool;
use sqlx::Row;

/// Filter options for analytics queries
#[derive(Debug, serde::Deserialize, Default)]
pub struct AnalyticsFilter {
    /// Start date for filtering data
    pub start_date: Option<DateTime<Utc>>,
    /// End date for filtering data
    pub end_date: Option<DateTime<Utc>>,
}

/// Analytics service for GitHub data analysis
pub struct Analytics {
    pool: PgPool,
}

impl Analytics {
    /// Creates a new Analytics instance
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Gets repository activity data
    pub async fn get_repository_activity(
        &self,
        owner: &str,
        repo: &str,
        days: i32,
    ) -> Result<Value, sqlx::Error> {
        let activity = sqlx::query(
            r#"
            SELECT
                DATE_TRUNC('day', created_at) as date,
                COUNT(*) as total_activity,
                SUM(CASE WHEN type = 'commit' THEN 1 ELSE 0 END) as commits
            FROM (
                SELECT created_at, 'commit' as type FROM commits
                WHERE repository_id = (
                    SELECT id FROM repositories
                    WHERE owner = $1 AND name = $2
                )
            ) activities
            WHERE created_at >= NOW() - ($3 || ' days')::INTERVAL
            GROUP BY DATE_TRUNC('day', created_at)
            ORDER BY date DESC
            "#,
        )
        .bind(owner)
        .bind(repo)
        .bind(days)
        .fetch_all(&self.pool)
        .await?;

        Ok(serde_json::json!({
            "dates": activity.iter().map(|row| row.get::<DateTime<Utc>, _>("date")).collect::<Vec<_>>(),
            "total_activity": activity.iter().map(|row| row.get::<i64, _>("total_activity")).collect::<Vec<_>>(),
            "commits": activity.iter().map(|row| row.get::<i64, _>("commits")).collect::<Vec<_>>(),
        }))
    }

    /// Gets repository trends data
    pub async fn get_repository_trends(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Value, sqlx::Error> {
        let days = 30; // Default to 30 days
        let query = sqlx::query!(
            r#"
            WITH author_stats AS (
                SELECT
                    COALESCE(c.author_name, 'Unknown') as author,
                    COUNT(*) as commit_count
                FROM commits c
                JOIN repositories r ON c.repository_id = r.id
                WHERE r.owner = $1 AND r.name = $2
                AND c.created_at >= NOW() - make_interval(days => $3)
                GROUP BY c.author_name
                ORDER BY commit_count DESC
                LIMIT 10
            )
            SELECT
                json_build_object(
                    'authors', array_agg(author),
                    'counts', array_agg(commit_count)
                ) as data
            FROM author_stats
            "#,
            owner,
            repo,
            days as i32
        );

        let result = query.fetch_one(&self.pool).await?;
        Ok(result.data.unwrap_or(Value::Null))
    }

    pub async fn get_repository_analytics(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Value, sqlx::Error> {
        let days = 30; // Default to 30 days
        let query = sqlx::query!(
            r#"
            WITH daily_commits AS (
                SELECT
                    date_trunc('day', c.created_at) as commit_date,
                    COUNT(*) as commit_count
                FROM commits c
                JOIN repositories r ON c.repository_id = r.id
                WHERE r.owner = $1 AND r.name = $2
                AND c.created_at >= NOW() - make_interval(days => $3)
                GROUP BY date_trunc('day', c.created_at)
                ORDER BY commit_date
            )
            SELECT
                json_build_object(
                    'dates', array_agg(commit_date),
                    'counts', array_agg(commit_count)
                ) as data
            FROM daily_commits
            "#,
            owner,
            repo,
            days as i32
        );

        let result = query.fetch_one(&self.pool).await?;
        Ok(result.data.unwrap_or(Value::Null))
    }
}
