pub mod analytics;
pub mod models;
pub mod routes;
pub mod services;
pub mod middleware;

use sqlx::PgPool;
use std::sync::Arc;
use crate::analytics::Analytics;

pub struct AppState {
    pub analytics: analytics::Analytics,
    pub pool: PgPool,
} 