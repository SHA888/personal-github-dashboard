use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod db;
mod error;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();

    // Get database URL from environment
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create database connection pool
    let pool = db::create_pool(&database_url)
        .await
        .expect("Failed to create database pool");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(api::configure_routes)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
