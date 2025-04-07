use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use personal_github_dashboard::{
    api::{configure_analytics_routes, configure_sync_routes},
    github::GitHubService,
    services::{analytics::Analytics, sync::SyncService},
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

    // Get environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid port number");

    // Create database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Run migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    // Initialize services
    let analytics = Arc::new(Analytics::new(pool.clone()));
    let github_service = Arc::new(GitHubService::new(github_token, pool.clone()));
    let sync_service = SyncService::new(github_service.clone());

    // Start the sync service in the background
    let _sync_handle = tokio::spawn(async move {
        sync_service.start().await;
    });

    // Create app state
    let app_state = web::Data::new(AppState {
        analytics,
        pool,
        github: github_service.clone(),
    });

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(web::resource("/health").to(health_check))
            .configure(|cfg| configure_analytics_routes(cfg, &app_state))
            .configure(|cfg| configure_sync_routes(cfg, &app_state))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}
