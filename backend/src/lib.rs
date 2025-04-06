pub mod models;
pub mod db;
pub mod cache;
pub mod routes;
pub mod analytics;
pub mod collaboration;
pub mod health;
pub mod project_health;

use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPool;
use dotenv::dotenv;
use std::env;

pub struct AppState {
    db: Arc<db::Database>,
    cache: Arc<cache::Cache>,
    analytics: Arc<analytics::Analytics>,
    collaboration: Arc<collaboration::Collaboration>,
    health: Arc<health::Health>,
    project_health: Arc<project_health::ProjectHealth>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    
    let db = Arc::new(db::Database::new(&database_url).await.expect("Failed to connect to database"));
    let cache = Arc::new(cache::Cache::new(&redis_url).expect("Failed to initialize cache"));
    
    let analytics = Arc::new(analytics::Analytics::new((*db).clone(), (*cache).clone()));
    let collaboration = Arc::new(collaboration::Collaboration::new((*db).clone(), (*cache).clone()));
    let health = Arc::new(health::Health::new((*db).clone(), (*cache).clone()));
    let project_health = Arc::new(project_health::ProjectHealth::new(
        (*db).clone(),
        (*cache).clone(),
        (*analytics).clone(),
        (*collaboration).clone(),
        (*health).clone(),
    ));
    
    let app_state = web::Data::new(AppState {
        db,
        cache,
        analytics,
        collaboration,
        health,
        project_health,
    });
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(|cfg| routes::configure_routes(cfg, &app_state))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
} 