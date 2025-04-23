use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

/// Create a PostgreSQL connection pool with custom configuration.
pub async fn create_pg_pool(database_url: &str, max_connections: u32) -> PgPool {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await
        .expect("Failed to create Postgres connection pool")
}
