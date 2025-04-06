use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Repo {
    pub id: i32,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub private: bool,
    pub fork: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub pushed_at: Option<DateTime<Utc>>,
    pub homepage: Option<String>,
    pub size: i32,
    pub stargazers_count: i32,
    pub watchers_count: i32,
    pub language: Option<String>,
    pub forks_count: i32,
    pub archived: bool,
    pub disabled: bool,
    pub open_issues_count: i32,
    pub default_branch: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Organization {
    pub id: i32,
    pub login: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Issue {
    pub id: i32,
    pub repository_id: i32,
    pub number: i32,
    pub title: String,
    pub state: String,
    pub body: Option<String>,
    pub user_id: i32,
    pub labels: Vec<String>,
    pub assignees: Vec<i32>,
    pub milestone_id: Option<i32>,
    pub locked: bool,
    pub comments_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequest {
    pub id: i32,
    pub repository_id: i32,
    pub number: i32,
    pub title: String,
    pub state: String,
    pub body: Option<String>,
    pub merged: bool,
    pub merged_at: Option<DateTime<Utc>>,
    pub merge_commit_sha: Option<String>,
    pub requested_reviewers: serde_json::Value,
    pub requested_teams: serde_json::Value,
    pub labels: serde_json::Value,
    pub comments_count: i32,
    pub review_comments_count: i32,
    pub commits_count: i32,
    pub additions: i32,
    pub deletions: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub id: i32,
    pub repository_id: i32,
    pub sha: String,
    pub message: String,
    pub author: serde_json::Value,
    pub committer: serde_json::Value,
    pub stats: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    pub id: i32,
    pub repository_id: i32,
    pub name: String,
    pub commit_sha: String,
    pub protected: bool,
    pub protection: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Release {
    pub id: i32,
    pub repository_id: i32,
    pub tag_name: String,
    pub name: Option<String>,
    pub body: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    pub assets: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Milestone {
    pub id: i32,
    pub repository_id: i32,
    pub number: i32,
    pub title: String,
    pub description: Option<String>,
    pub state: String,
    pub due_on: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workflow {
    pub id: i32,
    pub repository_id: i32,
    pub name: String,
    pub path: String,
    pub state: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TestRun {
    pub id: i32,
    pub repository_id: i32,
    pub commit_sha: String,
    pub total_tests: i32,
    pub passed_tests: i32,
    pub failed_tests: i32,
    pub skipped_tests: i32,
    pub coverage_percentage: Option<f64>,
    pub duration_seconds: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Dependency {
    pub id: i32,
    pub repository_id: i32,
    pub name: String,
    pub version: String,
    pub type_: String,
    pub is_outdated: bool,
    pub latest_version: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct SecurityVulnerability {
    pub id: i32,
    pub repository_id: i32,
    pub severity: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct HealthSnapshot {
    pub id: i32,
    pub repository_id: i32,
    pub score: f64,
    pub metrics: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct RiskIndicator {
    pub id: i32,
    pub repository_id: i32,
    pub indicator_type: String,
    pub value: f64,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Bottleneck {
    pub id: i32,
    pub repository_id: i32,
    pub type_: String,
    pub severity: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
} 