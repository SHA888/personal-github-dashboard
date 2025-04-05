use actix_web::{web, App, HttpServer};
use actix_ratelimit::{RateLimiter, MemoryStore, MemoryStoreActor};
use actix_cors::Cors;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::time::Duration;

mod analytics;
mod collaboration;
mod health;
mod project_health;
mod routes;
mod cache;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    env_logger::init();

    // Get configuration from environment
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    // Initialize database pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Initialize rate limiter
    let store = MemoryStore::new();
    let limiter = RateLimiter::new(
        MemoryStoreActor::from(store).start(),
        Duration::from_secs(60),
        60,
    );

    // Initialize services
    let analytics = web::Data::new(analytics::Analytics::new(pool.clone()));
    let collaboration = web::Data::new(collaboration::Collaboration::new(pool.clone()));
    let health = web::Data::new(health::Health::new(pool.clone()));
    let project_health = web::Data::new(project_health::ProjectHealth::new(pool.clone()));

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(limiter.clone())
            .app_data(analytics.clone())
            .app_data(collaboration.clone())
            .app_data(health.clone())
            .app_data(project_health.clone())
            .configure(routes::configure_routes)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}