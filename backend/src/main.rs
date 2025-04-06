use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use github_dashboard::services::{github::GitHubService, sync::SyncService};
use github_dashboard::{
    analytics::Analytics,
    routes::{configure_routes, configure_sync_routes},
    AppState,
};
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    env_logger::init();

    // Load environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    // Create database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    // Initialize services
    let analytics = Analytics::new(pool.clone());
    let github = Arc::new(GitHubService::new(github_token, pool.clone()));
    let sync_service = SyncService::new(github.clone(), pool.clone());

    // Start sync service in background
    tokio::spawn(async move {
        sync_service.start().await;
    });

    let app_state = web::Data::new(AppState { analytics, pool });

    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .configure(|cfg| configure_routes(cfg, &app_state))
            .configure(|cfg| configure_sync_routes(cfg, github.clone()))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
