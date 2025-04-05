use sqlx::PgPool;
use serde_json::Value;
use chrono::{DateTime, Utc, Duration};
use crate::cache::Cache;

pub struct Trends {
    pool: PgPool,
    cache: Cache,
}

impl Trends {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            cache: Cache::new().expect("Failed to initialize cache"),
        }
    }

    /// Gets growth rate metrics for a repository
    pub async fn get_repository_growth(
        &self,
        repository_id: i32,
        period: i32, // days
    ) -> Result<Value, sqlx::Error> {
        let key = format!("repo_growth:{}:{}", repository_id, period);
        self.cache.get_cached_or_compute(&key, Duration::from_secs(3600), async {
            let result = sqlx::query!(
                r#"
                WITH daily_stats AS (
                    SELECT 
                        date_trunc('day', created_at) as date,
                        COUNT(*) as total_activity,
                        COUNT(CASE WHEN type = 'issue' THEN 1 END) as issues,
                        COUNT(CASE WHEN type = 'pull_request' THEN 1 END) as prs,
                        COUNT(CASE WHEN type = 'commit' THEN 1 END) as commits
                    FROM repository_activity
                    WHERE repository_id = $1
                    AND created_at >= NOW() - ($2 || ' days')::interval
                    GROUP BY date
                )
                SELECT 
                    AVG(total_activity) as avg_daily_activity,
                    AVG(issues) as avg_daily_issues,
                    AVG(prs) as avg_daily_prs,
                    AVG(commits) as avg_daily_commits,
                    MAX(total_activity) as peak_activity,
                    MIN(total_activity) as low_activity,
                    (MAX(total_activity) - MIN(total_activity))::float / NULLIF(MIN(total_activity), 0) * 100 as growth_rate
                FROM daily_stats
                "#,
                repository_id,
                period
            )
            .fetch_one(&self.pool)
            .await?;

            Ok(serde_json::json!({
                "average_daily_activity": result.avg_daily_activity,
                "average_daily_issues": result.avg_daily_issues,
                "average_daily_prs": result.avg_daily_prs,
                "average_daily_commits": result.avg_daily_commits,
                "peak_activity": result.peak_activity,
                "low_activity": result.low_activity,
                "growth_rate": result.growth_rate,
                "period_days": period
            }))
        }).await
    }

    /// Gets velocity metrics for a repository
    pub async fn get_repository_velocity(
        &self,
        repository_id: i32,
        period: i32,
    ) -> Result<Value, sqlx::Error> {
        let key = format!("repo_velocity:{}:{}", repository_id, period);
        self.cache.get_cached_or_compute(&key, Duration::from_secs(3600), async {
            let result = sqlx::query!(
                r#"
                WITH velocity_metrics AS (
                    SELECT 
                        COUNT(*) as total_items,
                        COUNT(CASE WHEN state = 'closed' THEN 1 END) as completed_items,
                        AVG(EXTRACT(EPOCH FROM (closed_at - created_at))/3600) as avg_time_to_close,
                        COUNT(DISTINCT user_id) as active_contributors
                    FROM repository_activity
                    WHERE repository_id = $1
                    AND created_at >= NOW() - ($2 || ' days')::interval
                )
                SELECT 
                    total_items,
                    completed_items,
                    avg_time_to_close,
                    active_contributors,
                    (completed_items::float / NULLIF(total_items, 0) * 100) as completion_rate
                FROM velocity_metrics
                "#,
                repository_id,
                period
            )
            .fetch_one(&self.pool)
            .await?;

            Ok(serde_json::json!({
                "total_items": result.total_items,
                "completed_items": result.completed_items,
                "average_time_to_close_hours": result.avg_time_to_close,
                "active_contributors": result.active_contributors,
                "completion_rate": result.completion_rate,
                "period_days": period
            }))
        }).await
    }

    /// Gets burn-down chart data for a milestone
    pub async fn get_burndown_chart(
        &self,
        milestone_id: i32,
    ) -> Result<Value, sqlx::Error> {
        let key = format!("burndown:{}", milestone_id);
        self.cache.get_cached_or_compute(&key, Duration::from_secs(1800), async {
            let result = sqlx::query!(
                r#"
                WITH daily_progress AS (
                    SELECT 
                        date_trunc('day', updated_at) as date,
                        COUNT(*) as total_items,
                        COUNT(CASE WHEN state = 'closed' THEN 1 END) as completed_items
                    FROM milestone_items
                    WHERE milestone_id = $1
                    GROUP BY date
                    ORDER BY date
                )
                SELECT 
                    json_agg(
                        json_build_object(
                            'date', date,
                            'total_items', total_items,
                            'completed_items', completed_items
                        )
                    ) as chart_data
                FROM daily_progress
                "#,
                milestone_id
            )
            .fetch_one(&self.pool)
            .await?;

            Ok(serde_json::json!({
                "milestone_id": milestone_id,
                "chart_data": result.chart_data
            }))
        }).await
    }

    /// Gets release cycle analysis
    pub async fn get_release_cycle_analysis(
        &self,
        repository_id: i32,
    ) -> Result<Value, sqlx::Error> {
        let key = format!("release_cycle:{}", repository_id);
        self.cache.get_cached_or_compute(&key, Duration::from_secs(3600), async {
            let result = sqlx::query!(
                r#"
                WITH release_metrics AS (
                    SELECT 
                        COUNT(*) as total_releases,
                        AVG(EXTRACT(EPOCH FROM (published_at - created_at))/86400) as avg_days_to_release,
                        MIN(EXTRACT(EPOCH FROM (published_at - created_at))/86400) as min_days_to_release,
                        MAX(EXTRACT(EPOCH FROM (published_at - created_at))/86400) as max_days_to_release,
                        COUNT(DISTINCT EXTRACT(YEAR FROM published_at)::int) as years_active
                    FROM releases
                    WHERE repository_id = $1
                )
                SELECT 
                    total_releases,
                    avg_days_to_release,
                    min_days_to_release,
                    max_days_to_release,
                    years_active,
                    (total_releases::float / NULLIF(years_active, 0)) as releases_per_year
                FROM release_metrics
                "#,
                repository_id
            )
            .fetch_one(&self.pool)
            .await?;

            Ok(serde_json::json!({
                "total_releases": result.total_releases,
                "average_days_to_release": result.avg_days_to_release,
                "minimum_days_to_release": result.min_days_to_release,
                "maximum_days_to_release": result.max_days_to_release,
                "years_active": result.years_active,
                "releases_per_year": result.releases_per_year
            }))
        }).await
    }
} 