use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;

use github_dashboard::{AppState, db, analytics, routes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    env_logger::init();

    // Get configuration from environment
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    // Initialize database pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Initialize services
    let db = Arc::new(db::Database::new().await.expect("Failed to connect to database"));
    let analytics = Arc::new(analytics::Analytics::new(pool));
    
    let app_state = web::Data::new(AppState {
        db,
        analytics,
    });

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(app_state.clone())
            .configure(|cfg| routes::configure_routes(cfg, &app_state))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}