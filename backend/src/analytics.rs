use sqlx::PgPool;
use serde_json::Value;
use chrono::{DateTime, Utc};
use std::time::Duration;
use crate::cache::Cache;

/// Filter options for analytics queries
#[derive(Debug, serde::Deserialize)]
pub struct AnalyticsFilter {
    /// Start date for filtering data
    pub start_date: Option<DateTime<Utc>>,
    /// End date for filtering data
    pub end_date: Option<DateTime<Utc>>,
    /// Filter by state (open, closed, etc.)
    pub state: Option<String>,
    /// Minimum number of additions in commits
    pub min_additions: Option<i32>,
    /// Maximum number of additions in commits
    pub max_additions: Option<i32>,
    /// Minimum number of deletions in commits
    pub min_deletions: Option<i32>,
    /// Maximum number of deletions in commits
    pub max_deletions: Option<i32>,
    /// Minimum number of comments
    pub min_comments: Option<i32>,
    /// Maximum number of comments
    pub max_comments: Option<i32>,
    /// Minimum number of review comments
    pub min_review_comments: Option<i32>,
    /// Maximum number of review comments
    pub max_review_comments: Option<i32>,
    /// Include merged pull requests
    pub include_merged: Option<bool>,
    /// Include open issues/PRs
    pub include_open: Option<bool>,
    /// Include closed issues/PRs
    pub include_closed: Option<bool>,
}

impl Default for AnalyticsFilter {
    fn default() -> Self {
        Self {
            start_date: None,
            end_date: None,
            state: None,
            min_additions: None,
            max_additions: None,
            min_deletions: None,
            max_deletions: None,
            min_comments: None,
            max_comments: None,
            min_review_comments: None,
            max_review_comments: None,
            include_merged: Some(true),
            include_open: Some(true),
            include_closed: Some(true),
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
        if let Some(state) = &self.state {
            conditions.push(format!("state = '{}'", state));
        }
        if let Some(min_additions) = self.min_additions {
            conditions.push(format!("(stats->>'additions')::int >= {}", min_additions));
        }
        if let Some(max_additions) = self.max_additions {
            conditions.push(format!("(stats->>'additions')::int <= {}", max_additions));
        }
        if let Some(min_deletions) = self.min_deletions {
            conditions.push(format!("(stats->>'deletions')::int >= {}", min_deletions));
        }
        if let Some(max_deletions) = self.max_deletions {
            conditions.push(format!("(stats->>'deletions')::int <= {}", max_deletions));
        }
        if let Some(min_comments) = self.min_comments {
            conditions.push(format!("comments_count >= {}", min_comments));
        }
        if let Some(max_comments) = self.max_comments {
            conditions.push(format!("comments_count <= {}", max_comments));
        }
        if let Some(min_review_comments) = self.min_review_comments {
            conditions.push(format!("review_comments_count >= {}", min_review_comments));
        }
        if let Some(max_review_comments) = self.max_review_comments {
            conditions.push(format!("review_comments_count <= {}", max_review_comments));
        }

        let mut state_conditions = Vec::new();
        if self.include_merged.unwrap_or(true) {
            state_conditions.push("merged = true");
        }
        if self.include_open.unwrap_or(true) {
            state_conditions.push("state = 'open'");
        }
        if self.include_closed.unwrap_or(true) {
            state_conditions.push("state = 'closed'");
        }
        if !state_conditions.is_empty() {
            conditions.push(format!("({})", state_conditions.join(" OR ")));
        }

        if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        }
    }
}

/// Analytics service for GitHub data analysis
///
/// This service provides various analytics endpoints for analyzing GitHub data,
/// including repository activity, user contributions, organization stats,
/// code quality metrics, and team performance.
///
/// # Examples
///
/// ```rust
/// use github_dashboard::analytics::{Analytics, AnalyticsFilter};
/// use chrono::Utc;
///
/// #[tokio::main]
/// async fn main() {
///     let pool = sqlx::PgPool::connect("postgres://...").await.unwrap();
///     let analytics = Analytics::new(pool);
///
///     // Get repository activity with filters
///     let filter = AnalyticsFilter {
///         start_date: Some(Utc::now() - chrono::Duration::days(30)),
///         min_additions: Some(100),
///         include_merged: Some(true),
///         ..Default::default()
///     };
///
///     let activity = analytics.get_repository_activity(123, 30, Some(filter)).await.unwrap();
///     println!("Repository activity: {:?}", activity);
/// }
/// ```
pub struct Analytics {
    pool: PgPool,
    cache: Cache,
}

impl Analytics {
    /// Creates a new Analytics instance
    ///
    /// # Arguments
    ///
    /// * `pool` - PostgreSQL connection pool
    ///
    /// # Returns
    ///
    /// A new Analytics instance
    pub fn new(pool: PgPool) -> Self {
        Self { 
            pool,
            cache: Cache::new().expect("Failed to initialize cache"),
        }
    }

    // Helper method to get cached data or compute and cache it
    async fn get_cached_or_compute<T: serde::Serialize + serde::de::DeserializeOwned>(
        &self,
        key: &str,
        ttl: Duration,
        compute: impl std::future::Future<Output = Result<T, sqlx::Error>>,
    ) -> Result<T, sqlx::Error> {
        // Try to get from cache first
        if let Some(cached) = self.cache.get(key).await? {
            return Ok(cached);
        }

        // Compute the result
        let result = compute.await?;

        // Cache the result
        self.cache.set(key, &result, Some(ttl)).await?;

        Ok(result)
    }

    /// Gets repository activity data
    ///
    /// Returns daily activity metrics for a repository, including:
    /// - Total activity count
    /// - Issues created
    /// - Pull requests created
    /// - Commits
    /// - Open issues
    /// - Open PRs
    /// - Merged PRs
    /// - Large commits
    ///
    /// # Arguments
    ///
    /// * `repository_id` - ID of the repository
    /// * `days` - Number of days to look back
    /// * `filter` - Optional filter criteria
    ///
    /// # Returns
    ///
    /// JSON object containing activity metrics
    pub async fn get_repository_activity(
        &self,
        repository_id: i32,
        days: i32,
        filter: Option<AnalyticsFilter>,
    ) -> Result<Value, sqlx::Error> {
        let key = format!("repo_activity:{}:{}:{}", repository_id, days, serde_json::to_string(&filter).unwrap_or_default());
        let ttl = Duration::from_secs(300);

        self.get_cached_or_compute(&key, ttl, async move {
            let where_clause = filter.map(|f| f.build_where_clause()).unwrap_or_default();
            
            let activity = sqlx::query(&format!(
                r#"
                SELECT 
                    DATE_TRUNC('day', created_at) as date,
                    COUNT(*) as total_activity,
                    SUM(CASE WHEN type = 'issue' THEN 1 ELSE 0 END) as issues,
                    SUM(CASE WHEN type = 'pr' THEN 1 ELSE 0 END) as pull_requests,
                    SUM(CASE WHEN type = 'commit' THEN 1 ELSE 0 END) as commits,
                    SUM(CASE WHEN type = 'issue' AND state = 'open' THEN 1 ELSE 0 END) as open_issues,
                    SUM(CASE WHEN type = 'pr' AND state = 'open' THEN 1 ELSE 0 END) as open_prs,
                    SUM(CASE WHEN type = 'pr' AND merged = true THEN 1 ELSE 0 END) as merged_prs,
                    SUM(CASE WHEN type = 'commit' AND (stats->>'additions')::int > 1000 THEN 1 ELSE 0 END) as large_commits
                FROM (
                    SELECT created_at, 'issue' as type, state, false as merged, '{{}}'::jsonb as stats FROM issues WHERE repository_id = $1
                    UNION ALL
                    SELECT created_at, 'pr' as type, state, merged, '{{}}'::jsonb as stats FROM pull_requests WHERE repository_id = $1
                    UNION ALL
                    SELECT created_at, 'commit' as type, 'closed' as state, false as merged, stats FROM commits WHERE repository_id = $1
                ) activities
                WHERE created_at >= NOW() - ($2 || ' days')::INTERVAL
                {}
                GROUP BY DATE_TRUNC('day', created_at)
                ORDER BY date DESC
                "#,
                where_clause
            ))
            .bind(repository_id)
            .bind(days)
            .fetch_all(&self.pool)
            .await?;

            Ok(serde_json::json!(activity))
        }).await
    }

    /// Gets user contribution data
    ///
    /// Returns contribution metrics for a user across all repositories, including:
    /// - Issues created
    /// - Pull requests created
    /// - Commits
    /// - Code additions/deletions
    /// - PR review participation
    ///
    /// # Arguments
    ///
    /// * `user_id` - ID of the user
    /// * `filter` - Optional filter criteria
    ///
    /// # Returns
    ///
    /// JSON object containing contribution metrics
    pub async fn get_user_contributions(
        &self,
        user_id: i32,
        filter: Option<AnalyticsFilter>,
    ) -> Result<Value, sqlx::Error> {
        let key = format!("user_contributions:{}:{}", user_id, serde_json::to_string(&filter).unwrap_or_default());
        let ttl = Duration::from_secs(600);

        self.get_cached_or_compute(&key, ttl, async move {
            let where_clause = filter.map(|f| f.build_where_clause()).unwrap_or_default();
            
            let contributions = sqlx::query(&format!(
                r#"
                SELECT 
                    r.name as repository,
                    COUNT(DISTINCT i.id) as issues_created,
                    COUNT(DISTINCT CASE WHEN i.state = 'open' THEN i.id END) as open_issues,
                    COUNT(DISTINCT pr.id) as prs_created,
                    COUNT(DISTINCT CASE WHEN pr.state = 'open' THEN pr.id END) as open_prs,
                    COUNT(DISTINCT CASE WHEN pr.merged = true THEN pr.id END) as merged_prs,
                    COUNT(DISTINCT c.id) as commits,
                    SUM((c.stats->>'additions')::int) as total_additions,
                    SUM((c.stats->>'deletions')::int) as total_deletions,
                    AVG(EXTRACT(EPOCH FROM (pr.merged_at - pr.created_at))/3600) as avg_pr_merge_time_hours,
                    COUNT(DISTINCT CASE WHEN pr.review_comments_count > 0 THEN pr.id END) as reviewed_prs
                FROM repositories r
                LEFT JOIN issues i ON i.repository_id = r.id AND i.author->>'id' = $1::text
                LEFT JOIN pull_requests pr ON pr.repository_id = r.id AND pr.author->>'id' = $1::text
                LEFT JOIN commits c ON c.repository_id = r.id AND c.author->>'id' = $1::text
                {}
                GROUP BY r.id, r.name
                ORDER BY (issues_created + prs_created + commits) DESC
                "#,
                where_clause
            ))
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

            Ok(serde_json::json!(contributions))
        }).await
    }

    /// Gets organization statistics
    ///
    /// Returns overall statistics for an organization, including:
    /// - Total repositories
    /// - Total issues and PRs
    /// - Total commits
    /// - Open issues and PRs
    /// - Merged PRs
    /// - Average PR merge time
    /// - Code changes statistics
    ///
    /// # Arguments
    ///
    /// * `org_id` - ID of the organization
    /// * `filter` - Optional filter criteria
    ///
    /// # Returns
    ///
    /// JSON object containing organization statistics
    pub async fn get_organization_stats(&self, org_id: i32) -> Result<Value, sqlx::Error> {
        let key = format!("org_stats:{}", org_id);
        let ttl = Duration::from_secs(900); // Cache for 15 minutes

        self.get_cached_or_compute(&key, ttl, async move {
            let stats = sqlx::query(
                r#"
                SELECT 
                    COUNT(DISTINCT r.id) as total_repos,
                    COUNT(DISTINCT i.id) as total_issues,
                    COUNT(DISTINCT pr.id) as total_prs,
                    COUNT(DISTINCT c.id) as total_commits,
                    COUNT(DISTINCT CASE WHEN i.state = 'open' THEN i.id END) as open_issues,
                    COUNT(DISTINCT CASE WHEN pr.state = 'open' THEN pr.id END) as open_prs,
                    COUNT(DISTINCT CASE WHEN pr.merged = true THEN pr.id END) as merged_prs,
                    AVG(EXTRACT(EPOCH FROM (pr.merged_at - pr.created_at))/3600) as avg_pr_merge_time_hours,
                    SUM((c.stats->>'additions')::int) as total_additions,
                    SUM((c.stats->>'deletions')::int) as total_deletions,
                    COUNT(DISTINCT CASE WHEN c.stats->>'additions'::text > '1000' THEN c.id END) as large_commits,
                    COUNT(DISTINCT CASE WHEN pr.review_comments_count > 0 THEN pr.id END) as reviewed_prs,
                    COUNT(DISTINCT u.id) as total_contributors,
                    COUNT(DISTINCT CASE WHEN pr.merged = true AND pr.review_comments_count > 0 THEN pr.id END) as reviewed_and_merged_prs
                FROM repositories r
                LEFT JOIN issues i ON i.repository_id = r.id
                LEFT JOIN pull_requests pr ON pr.repository_id = r.id
                LEFT JOIN commits c ON c.repository_id = r.id
                LEFT JOIN users u ON u.id = COALESCE(
                    (i.author->>'id')::int,
                    (pr.author->>'id')::int,
                    (c.author->>'id')::int
                )
                WHERE r.organization_id = $1
                "#
            )
            .bind(org_id)
            .fetch_one(&self.pool)
            .await?;

            Ok(serde_json::json!(stats))
        }).await
    }

    /// Gets code quality metrics
    ///
    /// Returns code quality metrics for a repository, including:
    /// - Commit size analysis
    /// - PR review coverage
    /// - Merge time metrics
    /// - Review participation
    ///
    /// # Arguments
    ///
    /// * `repository_id` - ID of the repository
    /// * `filter` - Optional filter criteria
    ///
    /// # Returns
    ///
    /// JSON object containing code quality metrics
    pub async fn get_code_quality_metrics(&self, repository_id: i32) -> Result<Value, sqlx::Error> {
        let key = format!("code_quality:{}", repository_id);
        let ttl = Duration::from_secs(1200); // Cache for 20 minutes

        self.get_cached_or_compute(&key, ttl, async move {
            let metrics = sqlx::query(
                r#"
                SELECT 
                    COUNT(DISTINCT c.id) as total_commits,
                    COUNT(DISTINCT CASE WHEN c.stats->>'additions'::text > '1000' THEN c.id END) as large_commits,
                    AVG((c.stats->>'additions')::int + (c.stats->>'deletions')::int) as avg_changes_per_commit,
                    COUNT(DISTINCT CASE WHEN pr.review_comments_count > 0 THEN pr.id END) as reviewed_prs,
                    COUNT(DISTINCT pr.id) as total_prs,
                    COUNT(DISTINCT CASE WHEN pr.merged = true THEN pr.id END) as merged_prs,
                    AVG(EXTRACT(EPOCH FROM (pr.merged_at - pr.created_at))/3600) as avg_pr_merge_time_hours,
                    COUNT(DISTINCT CASE WHEN pr.merged = true AND pr.review_comments_count > 0 THEN pr.id END) as reviewed_and_merged_prs,
                    COUNT(DISTINCT CASE WHEN pr.merged = true AND pr.review_comments_count = 0 THEN pr.id END) as unreviewed_merged_prs,
                    COUNT(DISTINCT CASE WHEN pr.state = 'open' AND pr.review_comments_count > 0 THEN pr.id END) as open_reviewed_prs,
                    COUNT(DISTINCT CASE WHEN pr.state = 'open' AND pr.review_comments_count = 0 THEN pr.id END) as open_unreviewed_prs,
                    AVG(pr.comments_count) as avg_comments_per_pr,
                    AVG(pr.review_comments_count) as avg_review_comments_per_pr
                FROM repositories r
                LEFT JOIN commits c ON c.repository_id = r.id
                LEFT JOIN pull_requests pr ON pr.repository_id = r.id
                WHERE r.id = $1
                "#
            )
            .bind(repository_id)
            .fetch_one(&self.pool)
            .await?;

            Ok(serde_json::json!(metrics))
        }).await
    }

    /// Gets team performance metrics
    ///
    /// Returns performance metrics for team members in a repository, including:
    /// - Individual contribution counts
    /// - PR review participation
    /// - Code change statistics
    /// - Review and merge metrics
    ///
    /// # Arguments
    ///
    /// * `repository_id` - ID of the repository
    /// * `filter` - Optional filter criteria
    ///
    /// # Returns
    ///
    /// JSON object containing team performance metrics
    pub async fn get_team_performance(&self, repository_id: i32) -> Result<Value, sqlx::Error> {
        let key = format!("team_performance:{}", repository_id);
        let ttl = Duration::from_secs(1800); // Cache for 30 minutes

        self.get_cached_or_compute(&key, ttl, async move {
            let performance = sqlx::query(
                r#"
                SELECT 
                    u.login as username,
                    COUNT(DISTINCT i.id) as issues_created,
                    COUNT(DISTINCT CASE WHEN i.state = 'open' THEN i.id END) as open_issues,
                    COUNT(DISTINCT pr.id) as prs_created,
                    COUNT(DISTINCT CASE WHEN pr.state = 'open' THEN pr.id END) as open_prs,
                    COUNT(DISTINCT CASE WHEN pr.merged = true THEN pr.id END) as merged_prs,
                    COUNT(DISTINCT c.id) as commits,
                    SUM((c.stats->>'additions')::int) as total_additions,
                    SUM((c.stats->>'deletions')::int) as total_deletions,
                    AVG(EXTRACT(EPOCH FROM (pr.merged_at - pr.created_at))/3600) as avg_pr_merge_time_hours,
                    COUNT(DISTINCT CASE WHEN pr.review_comments_count > 0 THEN pr.id END) as reviewed_prs,
                    COUNT(DISTINCT CASE WHEN pr.merged = true AND pr.review_comments_count > 0 THEN pr.id END) as reviewed_and_merged_prs,
                    COUNT(DISTINCT CASE WHEN pr.merged = true AND pr.review_comments_count = 0 THEN pr.id END) as unreviewed_merged_prs,
                    AVG(pr.comments_count) as avg_comments_per_pr,
                    AVG(pr.review_comments_count) as avg_review_comments_per_pr
                FROM repositories r
                LEFT JOIN issues i ON i.repository_id = r.id
                LEFT JOIN pull_requests pr ON pr.repository_id = r.id
                LEFT JOIN commits c ON c.repository_id = r.id
                LEFT JOIN users u ON u.id = COALESCE(
                    (i.author->>'id')::int,
                    (pr.author->>'id')::int,
                    (c.author->>'id')::int
                )
                WHERE r.id = $1
                GROUP BY u.id, u.login
                ORDER BY (issues_created + prs_created + commits) DESC
                "#
            )
            .bind(repository_id)
            .fetch_all(&self.pool)
            .await?;

            Ok(serde_json::json!(performance))
        }).await
    }

    /// Invalidates cache for a repository
    ///
    /// # Arguments
    ///
    /// * `repository_id` - ID of the repository
    pub async fn invalidate_repository_cache(&self, repository_id: i32) -> Result<(), sqlx::Error> {
        let patterns = vec![
            format!("repo_activity:{}:*", repository_id),
            format!("code_quality:{}", repository_id),
            format!("team_performance:{}", repository_id),
        ];

        for pattern in patterns {
            self.cache.delete_pattern(&pattern).await?;
        }

        Ok(())
    }

    /// Invalidates cache for a user
    ///
    /// # Arguments
    ///
    /// * `user_id` - ID of the user
    pub async fn invalidate_user_cache(&self, user_id: i32) -> Result<(), sqlx::Error> {
        let key = format!("user_contributions:{}", user_id);
        self.cache.delete(&key).await?;
        Ok(())
    }

    /// Invalidates cache for an organization
    ///
    /// # Arguments
    ///
    /// * `org_id` - ID of the organization
    pub async fn invalidate_organization_cache(&self, org_id: i32) -> Result<(), sqlx::Error> {
        let key = format!("org_stats:{}", org_id);
        self.cache.delete(&key).await?;
        Ok(())
    }
} 