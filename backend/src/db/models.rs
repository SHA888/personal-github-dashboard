use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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
    pub owner_id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
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
