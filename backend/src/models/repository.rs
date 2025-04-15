use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Repository {
    pub id: Uuid,
    pub github_id: i64,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub private: bool,
    pub fork: bool,
    pub html_url: Option<String>,
    pub clone_url: Option<String>,
    pub default_branch: Option<String>,
    pub language: Option<String>,
    pub stargazers_count: i32,
    pub watchers_count: i32,
    pub forks_count: i32,
    pub open_issues_count: i32,
    pub size: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub pushed_at: Option<DateTime<Utc>>,
    pub last_synced_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RepositoryMetrics {
    pub id: Uuid,
    pub repository_id: Uuid,
    pub stargazers_count: i32,
    pub watchers_count: i32,
    pub forks_count: i32,
    pub open_issues_count: i32,
    pub open_pull_requests_count: i32,
    pub commit_count: i32,
    pub contributor_count: i32,
    pub recorded_at: DateTime<Utc>,
}

// Helper struct for aggregating metrics over time
#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryMetricsAggregation {
    pub repository_id: Uuid,
    pub period: String, // 'daily', 'weekly', 'monthly'
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub avg_stargazers: f64,
    pub avg_watchers: f64,
    pub avg_forks: f64,
    pub avg_open_issues: f64,
    pub avg_open_prs: f64,
    pub total_commits: i32,
    pub unique_contributors: i32,
}
