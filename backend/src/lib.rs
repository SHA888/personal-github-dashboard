use std::sync::Arc;

pub mod models;
pub mod db;
pub mod routes;
pub mod analytics;

pub struct AppState {
    pub db: Arc<db::Database>,
    pub analytics: Arc<analytics::Analytics>,
} 