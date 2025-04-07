pub mod api; // Previously routes
pub mod config;
pub mod db; // Previously models
pub mod github; // Previously services/github
pub mod middleware;
pub mod services; // Includes analytics and sync
pub mod utils;
pub mod websocket;

use crate::services::analytics::Analytics;
use sqlx::PgPool;

pub struct AppState {
    pub analytics: Analytics,
    pub pool: PgPool,
}
