mod models;
mod routes;
mod handlers;
mod middleware;
mod utils;
mod error;
mod db;
mod api;
mod github;

use actix_web::{App, HttpServer};
use actix_cors::Cors;
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    env_logger::init();

    // Initialize database connection
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Get port from environment or use default
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("127.0.0.1:{}", port);

    println!("Starting server on {}", bind_addr);

    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .wrap(middleware::logging::RequestLogger::default())
            .wrap(middleware::error_handler::ErrorHandler::default())
            .configure(routes::configure)
            .app_data(db_pool.clone())
    })
    .bind(&bind_addr)?
    .run()
    .await
}
