use octocrab::Octocrab;
use sqlx::PgPool;
use chrono::{DateTime, Utc};
use serde_json::Value;

pub struct GitHubService {
    client: Octocrab,
    pool: PgPool,
}

impl GitHubService {
    pub fn new(token: String, pool: PgPool) -> Self {
        let client = Octocrab::builder()
            .personal_token(token)
            .build()
            .expect("Failed to create GitHub client");
        
        Self { client, pool }
    }

    pub async fn sync_repository(&self, owner: &str, repo: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Get repository info
        let repository = self.client.repos(owner, repo).get().await?;
        
        // Insert or update repository
        let repository_id = sqlx::query!(
            r#"
            INSERT INTO repositories (id, owner, name)
            VALUES ($1, $2, $3)
            ON CONFLICT (owner, name) DO UPDATE
            SET updated_at = CURRENT_TIMESTAMP
            RETURNING id
            "#,
            repository.id.0 as i32,
            owner,
            repo
        )
        .fetch_one(&self.pool)
        .await?
        .id;

        // Get commits
        let commits = self.client.repos(owner, repo)
            .list_commits()
            .send()
            .await?;

        // Store commits in database
        for commit in commits.items {
            sqlx::query!(
                r#"
                INSERT INTO commits (sha, repository_id, author_name, author_email, message, created_at)
                VALUES ($1, $2, $3, $4, $5, $6)
                ON CONFLICT (repository_id, sha) DO UPDATE
                SET author_name = EXCLUDED.author_name,
                    author_email = EXCLUDED.author_email,
                    message = EXCLUDED.message,
                    created_at = EXCLUDED.created_at
                "#,
                commit.sha,
                repository_id,  // Use the returned repository_id
                commit.commit.author.as_ref().and_then(|a| Some(a.user.name.clone())),
                commit.commit.author.as_ref().and_then(|a| Some(a.user.email.clone())),
                commit.commit.message,
                commit.commit.author.as_ref().and_then(|a| a.date)
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }
} 