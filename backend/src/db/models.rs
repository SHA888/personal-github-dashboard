use octocrab::models::Author as GithubUser;
use octocrab::models::{orgs::Organization as GithubOrganization, Repository as GithubRepository};
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, FromRow};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub github_id: i64,
    pub login: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub access_token: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Organization {
    pub id: Uuid,
    pub github_id: i64,
    pub login: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Repository {
    pub id: Uuid,
    pub github_id: i64,
    pub owner_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_private: bool,
    pub is_fork: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct OrganizationMember {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct RepositoryCollaborator {
    pub id: Uuid,
    pub repository_id: Uuid,
    pub user_id: Uuid,
    pub permission: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
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
    pub recorded_at: OffsetDateTime,
}

impl From<GithubOrganization> for Organization {
    fn from(org: GithubOrganization) -> Self {
        Self {
            id: Uuid::new_v4(),
            github_id: org.id.0 as i64,
            login: org.login,
            name: org.name,
            description: org.description,
            avatar_url: Some(org.avatar_url.to_string()),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}

impl From<GithubRepository> for Repository {
    fn from(repo: GithubRepository) -> Self {
        Self {
            id: Uuid::new_v4(),
            github_id: repo.id.0 as i64,
            owner_id: Uuid::new_v4(), // This will be set separately when saving to DB
            name: repo.name,
            description: repo.description,
            is_private: repo.private.unwrap_or(false),
            is_fork: repo.fork.unwrap_or(false),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}

impl From<GithubUser> for User {
    fn from(user: GithubUser) -> Self {
        Self {
            id: Uuid::new_v4(),
            github_id: user.id.0 as i64,
            login: user.login,
            name: None,  // Author doesn't have name field
            email: None, // Author doesn't have email field
            avatar_url: Some(user.avatar_url.to_string()),
            access_token: None, // This will be set separately
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}
