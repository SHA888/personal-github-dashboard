use crate::github::GitHubService;
use crate::services::analytics::Analytics;
use sqlx::PgPool;
use std::sync::Arc;

pub struct AppState {
    pub analytics: Arc<Analytics>,
    pub pool: PgPool,
    pub github: Arc<GitHubService>,
}
