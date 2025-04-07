use crate::github::GitHubService;
use log::{error, info};
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
        info!("Starting sync service");
        loop {
            info!("Starting new sync cycle");
            // Get all repositories from database
            let repositories = match sqlx::query!("SELECT owner, name FROM repositories")
                .fetch_all(&self.pool)
                .await
            {
                Ok(repos) => repos,
                Err(e) => {
                    error!("Failed to fetch repositories: {}", e);
                    sleep(Duration::from_secs(60)).await;
                    continue;
                }
            };

            info!("Found {} repositories to sync", repositories.len());

            // Sync each repository
            for repo in repositories {
                info!("Syncing repository {}/{}", repo.owner, repo.name);
                if let Err(e) = self.github.sync_repository(&repo.owner, &repo.name).await {
                    error!(
                        "Failed to sync repository {}/{}: {}",
                        repo.owner, repo.name, e
                    );
                } else {
                    info!(
                        "Successfully synced repository {}/{}",
                        repo.owner, repo.name
                    );
                }
            }

            info!("Sync cycle completed. Waiting for next cycle in 5 minutes.");
            // Wait for 5 minutes before next sync (for testing)
            sleep(Duration::from_secs(300)).await;
        }
    }
}
