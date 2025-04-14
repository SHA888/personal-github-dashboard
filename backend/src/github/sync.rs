use crate::{
    db::{
        models::{Organization, Repository},
        DbPool,
    },
    error::AppError,
};
use octocrab::models::{orgs::Organization as GitHubOrg, Repository as GitHubRepo};

#[derive(Clone)]
pub struct GitHubSyncService {
    pool: DbPool,
}

impl GitHubSyncService {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn sync_organization(&self, org: GitHubOrg) -> Result<Organization, AppError> {
        let org_model = Organization::from(org);

        sqlx::query_as!(
            Organization,
            r#"
            INSERT INTO organizations (
                github_id, login, name, description,
                avatar_url, html_url, created_at, updated_at, id, last_synced_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (github_id) DO UPDATE SET
                login = EXCLUDED.login,
                name = EXCLUDED.name,
                description = EXCLUDED.description,
                avatar_url = EXCLUDED.avatar_url,
                html_url = EXCLUDED.html_url,
                updated_at = EXCLUDED.updated_at,
                last_synced_at = EXCLUDED.last_synced_at
            RETURNING *
            "#,
            org_model.github_id,
            org_model.login,
            org_model.name,
            org_model.description,
            org_model.avatar_url,
            org_model.html_url,
            org_model.created_at,
            org_model.updated_at,
            org_model.id,
            org_model.last_synced_at
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    #[allow(dead_code)]
    pub async fn sync_repository(&self, repo: GitHubRepo) -> Result<Repository, AppError> {
        let repo_model = Repository::from(repo);

        sqlx::query_as!(
            Repository,
            r#"
            INSERT INTO repositories (
                github_id, name, full_name, description, private, fork,
                html_url, clone_url, default_branch, language,
                stargazers_count, watchers_count, forks_count,
                open_issues_count, size, created_at, updated_at,
                pushed_at, last_synced_at, id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)
            ON CONFLICT (github_id) DO UPDATE SET
                name = EXCLUDED.name,
                full_name = EXCLUDED.full_name,
                description = EXCLUDED.description,
                private = EXCLUDED.private,
                fork = EXCLUDED.fork,
                html_url = EXCLUDED.html_url,
                clone_url = EXCLUDED.clone_url,
                default_branch = EXCLUDED.default_branch,
                language = EXCLUDED.language,
                stargazers_count = EXCLUDED.stargazers_count,
                watchers_count = EXCLUDED.watchers_count,
                forks_count = EXCLUDED.forks_count,
                open_issues_count = EXCLUDED.open_issues_count,
                size = EXCLUDED.size,
                updated_at = EXCLUDED.updated_at,
                pushed_at = EXCLUDED.pushed_at,
                last_synced_at = EXCLUDED.last_synced_at
            RETURNING
                id, github_id, name, full_name, description, private, fork,
                html_url, clone_url, default_branch, language, stargazers_count,
                watchers_count, forks_count, open_issues_count, size, created_at,
                updated_at, pushed_at, last_synced_at
            "#,
            repo_model.github_id,
            repo_model.name,
            repo_model.full_name,
            repo_model.description,
            repo_model.private,
            repo_model.fork,
            repo_model.html_url,
            repo_model.clone_url,
            repo_model.default_branch,
            repo_model.language,
            repo_model.stargazers_count,
            repo_model.watchers_count,
            repo_model.forks_count,
            repo_model.open_issues_count,
            repo_model.size,
            repo_model.created_at,
            repo_model.updated_at,
            repo_model.pushed_at,
            repo_model.last_synced_at,
            repo_model.id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
    }
}
