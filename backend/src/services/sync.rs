use crate::db::{Organization, Repository};
use crate::error::AppError;
use crate::services::github_api::GitHubService;
use octocrab::models::orgs::Organization as GithubOrg;
use octocrab::models::Repository as GithubRepo;
use sqlx::PgPool;
use time::OffsetDateTime;

#[derive(Clone)]
pub struct GitHubSyncService {
    pool: PgPool,
    github: GitHubService,
}

impl GitHubSyncService {
    pub fn new(pool: PgPool, github: GitHubService) -> Self {
        Self { pool, github }
    }

    pub async fn sync_organization(&self, org: GithubOrg) -> Result<Organization, AppError> {
        let org_model = Organization::from(org);

        let organization = sqlx::query_as!(
            Organization,
            r#"
            INSERT INTO organizations (
                id, github_id, login, name, description, avatar_url,
                created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (github_id) DO UPDATE SET
                login = EXCLUDED.login,
                name = EXCLUDED.name,
                description = EXCLUDED.description,
                avatar_url = EXCLUDED.avatar_url,
                updated_at = EXCLUDED.updated_at
            RETURNING id, github_id, login as "login!: String", name, description, avatar_url, created_at as "created_at!: OffsetDateTime", updated_at as "updated_at!: OffsetDateTime"
            "#,
            org_model.id,
            org_model.github_id,
            org_model.login,
            org_model.name,
            org_model.description,
            org_model.avatar_url,
            org_model.created_at,
            org_model.updated_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(organization)
    }

    pub async fn sync_repository(&self, repo: GithubRepo) -> Result<Repository, AppError> {
        let repo_model = Repository::from(repo);

        let repository = sqlx::query_as!(
            Repository,
            r#"
            INSERT INTO repositories (
                id, github_id, owner_id, name, description, is_private, is_fork,
                created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (github_id) DO UPDATE SET
                name = EXCLUDED.name,
                description = EXCLUDED.description,
                is_private = EXCLUDED.is_private,
                is_fork = EXCLUDED.is_fork,
                updated_at = EXCLUDED.updated_at
            RETURNING id, github_id, owner_id, name as "name!: String", description, is_private as "is_private!: bool", is_fork as "is_fork!: bool", created_at as "created_at!: OffsetDateTime", updated_at as "updated_at!: OffsetDateTime"
            "#,
            repo_model.id,
            repo_model.github_id,
            repo_model.owner_id,
            repo_model.name,
            repo_model.description,
            repo_model.is_private,
            repo_model.is_fork,
            repo_model.created_at,
            repo_model.updated_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(repository)
    }

    pub async fn sync_user_organizations(&self) -> Result<(), AppError> {
        let orgs = self.github.list_my_organizations().await?;

        for org in orgs {
            self.sync_organization(org).await?;
        }

        Ok(())
    }

    pub async fn sync_user_repositories(&self) -> Result<(), AppError> {
        let repos = self.github.list_my_repositories().await?;

        for repo in repos {
            self.sync_repository(repo).await?;
        }

        Ok(())
    }
}
