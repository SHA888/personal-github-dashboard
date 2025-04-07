use crate::github::GitHubService;
use sqlx::PgPool;
use std::sync::Arc;

pub struct AppState {
    pub pool: PgPool,
    pub github: Arc<GitHubService>,
}
