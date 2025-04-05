use sqlx::PgPool;
use serde_json::Value;
use chrono::{DateTime, Utc, Duration};
use crate::cache::Cache;

pub struct Collaboration {
    pool: PgPool,
    cache: Cache,
}

impl Collaboration {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            cache: Cache::new().expect("Failed to initialize cache"),
        }
    }

    /// Gets team interaction patterns
    pub async fn get_team_interactions(
        &self,
        repository_id: i32,
        period: i32,
    ) -> Result<Value, sqlx::Error> {
        let key = format!("team_interactions:{}:{}", repository_id, period);
        self.cache.get_cached_or_compute(&key, Duration::from_secs(3600), async {
            let result = sqlx::query!(
                r#"
                WITH team_interactions AS (
                    SELECT 
                        u1.login as user1,
                        u2.login as user2,
                        COUNT(*) as interaction_count
                    FROM pull_request_reviews prr1
                    JOIN pull_request_reviews prr2 ON prr1.pull_request_id = prr2.pull_request_id
                    JOIN users u1 ON prr1.user_id = u1.id
                    JOIN users u2 ON prr2.user_id = u2.id
                    WHERE prr1.repository_id = $1
                    AND prr1.created_at >= NOW() - ($2 || ' days')::interval
                    AND prr1.user_id < prr2.user_id
                    GROUP BY u1.login, u2.login
                )
                SELECT 
                    json_agg(
                        json_build_object(
                            'user1', user1,
                            'user2', user2,
                            'interaction_count', interaction_count
                        )
                    ) as interaction_data
                FROM team_interactions
                "#,
                repository_id,
                period
            )
            .fetch_one(&self.pool)
            .await?;

            Ok(serde_json::json!({
                "repository_id": repository_id,
                "period_days": period,
                "interaction_data": result.interaction_data
            }))
        }).await
    }

    /// Gets review response times
    pub async fn get_review_response_times(
        &self,
        repository_id: i32,
        period: i32,
    ) -> Result<Value, sqlx::Error> {
        let key = format!("review_response:{}:{}", repository_id, period);
        self.cache.get_cached_or_compute(&key, Duration::from_secs(3600), async {
            let result = sqlx::query!(
                r#"
                WITH review_times AS (
                    SELECT 
                        AVG(EXTRACT(EPOCH FROM (prr.created_at - pr.created_at))/3600) as avg_response_time,
                        MIN(EXTRACT(EPOCH FROM (prr.created_at - pr.created_at))/3600) as min_response_time,
                        MAX(EXTRACT(EPOCH FROM (prr.created_at - pr.created_at))/3600) as max_response_time,
                        COUNT(*) as total_reviews
                    FROM pull_requests pr
                    JOIN pull_request_reviews prr ON pr.id = prr.pull_request_id
                    WHERE pr.repository_id = $1
                    AND pr.created_at >= NOW() - ($2 || ' days')::interval
                )
                SELECT 
                    avg_response_time,
                    min_response_time,
                    max_response_time,
                    total_reviews,
                    (SELECT COUNT(*) FROM pull_requests 
                     WHERE repository_id = $1 
                     AND created_at >= NOW() - ($2 || ' days')::interval) as total_prs
                FROM review_times
                "#,
                repository_id,
                period
            )
            .fetch_one(&self.pool)
            .await?;

            Ok(serde_json::json!({
                "repository_id": repository_id,
                "period_days": period,
                "average_response_time_hours": result.avg_response_time,
                "minimum_response_time_hours": result.min_response_time,
                "maximum_response_time_hours": result.max_response_time,
                "total_reviews": result.total_reviews,
                "total_pull_requests": result.total_prs,
                "review_coverage": (result.total_reviews as f64 / result.total_prs as f64) * 100.0
            }))
        }).await
    }

    /// Gets code review distribution
    pub async fn get_review_distribution(
        &self,
        repository_id: i32,
        period: i32,
    ) -> Result<Value, sqlx::Error> {
        let key = format!("review_dist:{}:{}", repository_id, period);
        self.cache.get_cached_or_compute(&key, Duration::from_secs(3600), async {
            let result = sqlx::query!(
                r#"
                WITH review_stats AS (
                    SELECT 
                        u.login as reviewer,
                        COUNT(*) as review_count,
                        AVG(EXTRACT(EPOCH FROM (prr.created_at - pr.created_at))/3600) as avg_response_time,
                        COUNT(DISTINCT pr.id) as unique_prs_reviewed
                    FROM pull_request_reviews prr
                    JOIN users u ON prr.user_id = u.id
                    JOIN pull_requests pr ON prr.pull_request_id = pr.id
                    WHERE pr.repository_id = $1
                    AND prr.created_at >= NOW() - ($2 || ' days')::interval
                    GROUP BY u.login
                )
                SELECT 
                    json_agg(
                        json_build_object(
                            'reviewer', reviewer,
                            'review_count', review_count,
                            'average_response_time_hours', avg_response_time,
                            'unique_prs_reviewed', unique_prs_reviewed
                        )
                    ) as distribution_data
                FROM review_stats
                "#,
                repository_id,
                period
            )
            .fetch_one(&self.pool)
            .await?;

            Ok(serde_json::json!({
                "repository_id": repository_id,
                "period_days": period,
                "distribution_data": result.distribution_data
            }))
        }).await
    }

    /// Gets cross-team collaboration metrics
    pub async fn get_cross_team_collaboration(
        &self,
        organization_id: i32,
        period: i32,
    ) -> Result<Value, sqlx::Error> {
        let key = format!("cross_team:{}:{}", organization_id, period);
        self.cache.get_cached_or_compute(&key, Duration::from_secs(3600), async {
            let result = sqlx::query!(
                r#"
                WITH team_collaboration AS (
                    SELECT 
                        r1.name as repo1,
                        r2.name as repo2,
                        COUNT(DISTINCT u.id) as shared_contributors,
                        COUNT(DISTINCT CASE WHEN c1.repository_id = r1.id THEN c1.id END) as repo1_commits,
                        COUNT(DISTINCT CASE WHEN c2.repository_id = r2.id THEN c2.id END) as repo2_commits
                    FROM repositories r1
                    JOIN repositories r2 ON r1.organization_id = r2.organization_id
                    JOIN commits c1 ON r1.id = c1.repository_id
                    JOIN commits c2 ON r2.id = c2.repository_id
                    JOIN users u ON c1.author_id = u.id AND c2.author_id = u.id
                    WHERE r1.organization_id = $1
                    AND r1.id < r2.id
                    AND c1.created_at >= NOW() - ($2 || ' days')::interval
                    AND c2.created_at >= NOW() - ($2 || ' days')::interval
                    GROUP BY r1.name, r2.name
                )
                SELECT 
                    json_agg(
                        json_build_object(
                            'repository_pair', json_build_array(repo1, repo2),
                            'shared_contributors', shared_contributors,
                            'repository1_commits', repo1_commits,
                            'repository2_commits', repo2_commits,
                            'collaboration_score', (shared_contributors::float / 
                                (NULLIF(repo1_commits, 0) + NULLIF(repo2_commits, 0))::float) * 100
                        )
                    ) as collaboration_data
                FROM team_collaboration
                "#,
                organization_id,
                period
            )
            .fetch_one(&self.pool)
            .await?;

            Ok(serde_json::json!({
                "organization_id": organization_id,
                "period_days": period,
                "collaboration_data": result.collaboration_data
            }))
        }).await
    }
} 