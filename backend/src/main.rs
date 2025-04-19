use actix_cors::Cors;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{App, HttpServer};
use cookie::Key;
use dotenv::dotenv;
use sqlx::PgPool;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    env_logger::init();

    // Database pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    // Redis session store
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let redis_store = RedisSessionStore::new(redis_url)
        .await
        .expect("Failed to connect to Redis");

    // Server binding
    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".into());

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(SessionMiddleware::new(redis_store.clone(), Key::generate()))
            // Pass database pool to routes
            .app_data(actix_web::web::Data::new(pool.clone()))
            // Configure routes
            .configure(routes::init_routes)
    })
    .bind(bind_addr)?
    .run()
    .await
}
