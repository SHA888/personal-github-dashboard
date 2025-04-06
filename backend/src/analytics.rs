use sqlx::PgPool;
use sqlx::Row;
use serde_json::Value;
use chrono::{DateTime, Utc};

/// Filter options for analytics queries
#[derive(Debug, serde::Deserialize)]
pub struct AnalyticsFilter {
    /// Start date for filtering data
    pub start_date: Option<DateTime<Utc>>,
    /// End date for filtering data
    pub end_date: Option<DateTime<Utc>>,
}

impl Default for AnalyticsFilter {
    fn default() -> Self {
        Self {
            start_date: None,
            end_date: None,
        }
    }
}

impl AnalyticsFilter {
    fn build_where_clause(&self) -> String {
        let mut conditions = Vec::new();

        if let Some(start_date) = self.start_date {
            conditions.push(format!("created_at >= '{}'", start_date));
        }
        if let Some(end_date) = self.end_date {
            conditions.push(format!("created_at <= '{}'", end_date));
        }

        if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        }
    }
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
            "#
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
        days: i32,
    ) -> Result<Value, sqlx::Error> {
        let trends = sqlx::query(
            r#"
            SELECT 
                DATE_TRUNC('day', created_at) as date,
                COUNT(*) as commit_count
            FROM commits
            WHERE repository_id = (
                SELECT id FROM repositories 
                WHERE owner = $1 AND name = $2
            )
            AND created_at >= NOW() - ($3 || ' days')::INTERVAL
            GROUP BY DATE_TRUNC('day', created_at)
            ORDER BY date DESC
            "#
        )
        .bind(owner)
        .bind(repo)
        .bind(days)
        .fetch_all(&self.pool)
        .await?;

        Ok(serde_json::json!({
            "dates": trends.iter().map(|row| row.get::<DateTime<Utc>, _>("date")).collect::<Vec<_>>(),
            "commit_counts": trends.iter().map(|row| row.get::<i64, _>("commit_count")).collect::<Vec<_>>(),
        }))
    }
} 