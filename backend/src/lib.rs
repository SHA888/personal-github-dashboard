pub mod api;          // Previously routes
pub mod db;          // Previously models
pub mod github;      // Previously services/github
pub mod middleware;
pub mod services;    // Includes analytics and sync
pub mod websocket;
pub mod config;
pub mod utils;

use sqlx::PgPool;
use crate::services::analytics::Analytics;

pub struct AppState {
    pub analytics: Analytics,
    pub pool: PgPool,
}
