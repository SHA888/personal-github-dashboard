use crate::github::GitHubService;
use log;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;

pub struct SyncService {
    github_service: Arc<GitHubService>,
}

impl SyncService {
    pub fn new(github_service: Arc<GitHubService>) -> Self {
        Self { github_service }
    }

    pub async fn start(&self) {
        log::info!("Starting sync service");

        loop {
            match self.github_service.sync_user_repositories().await {
                Ok(_) => log::info!("Successfully synced all repositories"),
                Err(e) => log::error!("Failed to sync repositories: {}", e),
            }

            // Wait for 5 minutes before next sync
            time::sleep(Duration::from_secs(300)).await;
        }
    }
}
