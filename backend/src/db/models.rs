use chrono::{DateTime, Utc};
use octocrab::models::{orgs::Organization as GitHubOrg, Repository as GitHubRepo};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub github_id: i64,
    pub login: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub html_url: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub last_synced_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Organization {
    pub id: Uuid,
    pub github_id: i64,
    pub login: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub html_url: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub last_synced_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Repository {
    pub id: Uuid,
    pub github_id: i64,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub private: Option<bool>,
    pub fork: Option<bool>,
    pub html_url: Option<String>,
    pub clone_url: Option<String>,
    pub default_branch: Option<String>,
    pub language: Option<String>,
    pub stargazers_count: Option<i32>,
    pub watchers_count: Option<i32>,
    pub forks_count: Option<i32>,
    pub open_issues_count: Option<i32>,
    pub size: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub pushed_at: Option<DateTime<Utc>>,
    pub last_synced_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrganizationMember {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RepositoryCollaborator {
    pub id: Uuid,
    pub repository_id: Uuid,
    pub user_id: Uuid,
    pub permission: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
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

impl From<GitHubOrg> for Organization {
    fn from(org: GitHubOrg) -> Self {
        Self {
            id: Uuid::new_v4(),
            github_id: org.id.0 as i64,
            login: org.login,
            name: org.name,
            description: org.description,
            avatar_url: Some(org.avatar_url.to_string()),
            html_url: org.html_url.map(|url| url.to_string()),
            created_at: org.created_at,
            updated_at: org.created_at,
            last_synced_at: Some(Utc::now()),
        }
    }
}

impl From<GitHubRepo> for Repository {
    fn from(repo: GitHubRepo) -> Self {
        Self {
            id: Uuid::new_v4(),
            github_id: repo.id.0 as i64,
            name: repo.name.clone(),
            full_name: repo.full_name.unwrap_or_else(|| repo.name.clone()),
            description: repo.description,
            private: repo.private,
            fork: repo.fork,
            html_url: repo.html_url.map(|url| url.to_string()),
            clone_url: repo.clone_url.map(|url| url.to_string()),
            default_branch: repo.default_branch,
            language: repo.language.and_then(|v| match v {
                Value::String(s) => Some(s),
                _ => None,
            }),
            stargazers_count: repo.stargazers_count.map(|c| c as i32),
            watchers_count: repo.watchers_count.map(|c| c as i32),
            forks_count: repo.forks_count.map(|c| c as i32),
            open_issues_count: repo.open_issues_count.map(|c| c as i32),
            size: repo.size.map(|s| s as i32),
            created_at: repo.created_at,
            updated_at: repo.updated_at,
            pushed_at: repo.pushed_at,
            last_synced_at: Some(Utc::now()),
        }
    }
}
