use crate::github::GitHubService;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

pub struct SyncService {
    github: Arc<GitHubService>,
    pool: PgPool,
}

impl SyncService {
    pub fn new(github: Arc<GitHubService>, pool: PgPool) -> Self {
        Self { github, pool }
    }

    pub async fn start(&self) {
        loop {
            // Get all repositories from database
            let repositories = sqlx::query!("SELECT owner, name FROM repositories")
                .fetch_all(&self.pool)
                .await
                .expect("Failed to fetch repositories");

            // Sync each repository
            for repo in repositories {
                if let Err(e) = self.github.sync_repository(&repo.owner, &repo.name).await {
                    eprintln!(
                        "Failed to sync repository {}/{}: {}",
                        repo.owner, repo.name, e
                    );
                }
            }

            // Wait for 1 hour before next sync
            sleep(Duration::from_secs(3600)).await;
        }
    }
}
