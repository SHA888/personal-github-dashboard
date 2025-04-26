use actix_web::{test, App};
use dotenv::dotenv;
use personal_github_dashboard::routes::init_routes_test::init_routes_no_auth;
use personal_github_dashboard::utils::redis::RedisClient;
use serde_json;
use sqlx::PgPool;
use uuid::Uuid;

// Helper: Setup test app and Redis
async fn setup_app_and_redis() -> (PgPool, RedisClient) {
    // These should use test env vars or config
    let database_url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let redis_url = std::env::var("TEST_REDIS_URL").expect("TEST_REDIS_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("DB connect");
    let redis = RedisClient::new(&redis_url).await.expect("Redis connect");
    (pool, redis)
}

// --- DRY Helper functions ---

async fn insert_user(pool: &PgPool, user_id: Uuid, username: &str) {
    sqlx::query!(
        "INSERT INTO users (id, username, email, created_at) VALUES ($1, $2, $3, NOW())",
        user_id,
        username,
        &format!("{}@example.com", username)
    )
    .execute(pool)
    .await
    .expect("insert user");
}

async fn insert_organization(pool: &PgPool, org_id: Uuid, name: &str) {
    sqlx::query!(
        "INSERT INTO organizations (id, name, description, created_at) VALUES ($1, $2, $3, NOW())",
        org_id,
        name,
        Some("desc")
    )
    .execute(pool)
    .await
    .expect("insert org");
}

async fn insert_repository(
    pool: &PgPool,
    repo_id: Uuid,
    org_id: Option<Uuid>,
    owner_id: Uuid,
    name: &str,
) {
    sqlx::query!("INSERT INTO repositories (id, org_id, owner_id, name, description, created_at) VALUES ($1, $2, $3, $4, $5, NOW())",
        repo_id, org_id, owner_id, name, Some("desc"))
        .execute(pool).await.expect("insert repo");
}

async fn insert_activity(pool: &PgPool, activity_id: Uuid, user_id: Uuid, repo_id: Option<Uuid>) {
    sqlx::query!("INSERT INTO activities (id, user_id, repo_id, type, timestamp, data, created_at) VALUES ($1, $2, $3, $4, NOW(), $5, NOW())",
        activity_id, user_id, repo_id, "testtype", serde_json::json!({"k":"v"}))
        .execute(pool).await.expect("insert activity");
}

// Truncate tables for a clean state before each test
async fn truncate_tables(pool: &PgPool) {
    sqlx::query!(
        "TRUNCATE TABLE activities, repositories, organizations, users RESTART IDENTITY CASCADE;"
    )
    .execute(pool)
    .await
    .expect("truncate tables");
}

#[actix_rt::test]
async fn test_user_caching() {
    dotenv().ok();
    let database_url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let redis_url = std::env::var("TEST_REDIS_URL").expect("TEST_REDIS_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("DB connect");
    let redis = RedisClient::new(&redis_url).await.expect("Redis connect");
    truncate_tables(&pool).await;
    let user_id = Uuid::new_v4();
    let username = format!("testuser-{}", user_id);
    insert_user(&pool, user_id, &username).await;
    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .app_data(actix_web::web::Data::new(redis.clone()))
            .configure(init_routes_no_auth),
    )
    .await;
    let req = test::TestRequest::get()
        .uri(&format!("/api/user/{}", user_id))
        .to_request();
    let resp1 = test::call_service(&app, req).await;
    let status1 = resp1.status();
    let body_bytes = test::read_body(resp1).await;
    println!("[DEBUG] Status: {}", status1);
    println!("[DEBUG] Body: {}", String::from_utf8_lossy(&body_bytes));
    assert!(status1.is_success());
    let req = test::TestRequest::get()
        .uri(&format!("/api/user/{}", user_id))
        .to_request();
    let resp2 = test::call_service(&app, req).await;
    let status2 = resp2.status();
    let body_bytes2 = test::read_body(resp2).await;
    println!("[DEBUG] Status: {}", status2);
    println!("[DEBUG] Body: {}", String::from_utf8_lossy(&body_bytes2));
    assert!(status2.is_success());
    let cache_key = format!("user:{}", user_id);
    let cached: Option<String> = redis.get(&cache_key).await.unwrap();
    assert!(cached.is_some());
}

#[actix_rt::test]
async fn test_organization_caching() {
    dotenv().ok();
    let database_url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let redis_url = std::env::var("TEST_REDIS_URL").expect("TEST_REDIS_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("DB connect");
    let redis = RedisClient::new(&redis_url).await.expect("Redis connect");
    truncate_tables(&pool).await;
    let org_id = Uuid::new_v4();
    let name = format!("testorg-{}", org_id);
    insert_organization(&pool, org_id, &name).await;
    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .app_data(actix_web::web::Data::new(redis.clone()))
            .configure(init_routes_no_auth),
    )
    .await;
    let req = test::TestRequest::get()
        .uri(&format!("/api/organization/{}", org_id))
        .to_request();
    let resp1 = test::call_service(&app, req).await;
    let status1 = resp1.status();
    let body_bytes = test::read_body(resp1).await;
    println!("[DEBUG] Status: {}", status1);
    println!("[DEBUG] Body: {}", String::from_utf8_lossy(&body_bytes));
    assert!(status1.is_success());
    let req = test::TestRequest::get()
        .uri(&format!("/api/organization/{}", org_id))
        .to_request();
    let resp2 = test::call_service(&app, req).await;
    let status2 = resp2.status();
    let body_bytes2 = test::read_body(resp2).await;
    println!("[DEBUG] Status: {}", status2);
    println!("[DEBUG] Body: {}", String::from_utf8_lossy(&body_bytes2));
    assert!(status2.is_success());
    let cache_key = format!("organization:{}", org_id);
    let cached: Option<String> = redis.get(&cache_key).await.unwrap();
    assert!(cached.is_some());
}

#[actix_rt::test]
async fn test_repository_caching() {
    dotenv().ok();
    let database_url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let redis_url = std::env::var("TEST_REDIS_URL").expect("TEST_REDIS_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("DB connect");
    let redis = RedisClient::new(&redis_url).await.expect("Redis connect");
    truncate_tables(&pool).await;
    let org_id = Uuid::new_v4();
    let repo_id = Uuid::new_v4();
    let owner_id = Uuid::new_v4();
    let name = format!("testrepo-{}", repo_id);
    insert_user(&pool, owner_id, &format!("testuser-{}", owner_id)).await;
    insert_organization(&pool, org_id, &format!("testorg-{}", org_id)).await;
    insert_repository(&pool, repo_id, Some(org_id), owner_id, &name).await;
    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .app_data(actix_web::web::Data::new(redis.clone()))
            .configure(init_routes_no_auth),
    )
    .await;
    let req = test::TestRequest::get()
        .uri(&format!("/api/repository/{}", repo_id))
        .to_request();
    let resp1 = test::call_service(&app, req).await;
    let status1 = resp1.status();
    let body_bytes = test::read_body(resp1).await;
    println!("[DEBUG] Status: {}", status1);
    println!("[DEBUG] Body: {}", String::from_utf8_lossy(&body_bytes));
    assert!(status1.is_success());
    let req = test::TestRequest::get()
        .uri(&format!("/api/repository/{}", repo_id))
        .to_request();
    let resp2 = test::call_service(&app, req).await;
    let status2 = resp2.status();
    let body_bytes2 = test::read_body(resp2).await;
    println!("[DEBUG] Status: {}", status2);
    println!("[DEBUG] Body: {}", String::from_utf8_lossy(&body_bytes2));
    assert!(status2.is_success());
    let cache_key = format!("repository:{}", repo_id);
    let cached: Option<String> = redis.get(&cache_key).await.unwrap();
    assert!(cached.is_some());
}

#[actix_rt::test]
async fn test_activity_caching() {
    dotenv().ok();
    let database_url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let redis_url = std::env::var("TEST_REDIS_URL").expect("TEST_REDIS_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("DB connect");
    let redis = RedisClient::new(&redis_url).await.expect("Redis connect");
    truncate_tables(&pool).await;
    let user_id = Uuid::new_v4();
    let repo_id = Uuid::new_v4();
    let activity_id = Uuid::new_v4();
    insert_user(&pool, user_id, &format!("testuser-{}", user_id)).await;
    insert_repository(
        &pool,
        repo_id,
        None,
        user_id,
        &format!("testrepo-{}", repo_id),
    )
    .await;
    insert_activity(&pool, activity_id, user_id, Some(repo_id)).await;
    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .app_data(actix_web::web::Data::new(redis.clone()))
            .configure(init_routes_no_auth),
    )
    .await;
    let req = test::TestRequest::get()
        .uri(&format!("/api/activity/{}", activity_id))
        .to_request();
    let resp1 = test::call_service(&app, req).await;
    let status1 = resp1.status();
    let body_bytes = test::read_body(resp1).await;
    println!("[DEBUG] Status: {}", status1);
    println!("[DEBUG] Body: {}", String::from_utf8_lossy(&body_bytes));
    assert!(status1.is_success());
    let req = test::TestRequest::get()
        .uri(&format!("/api/activity/{}", activity_id))
        .to_request();
    let resp2 = test::call_service(&app, req).await;
    let status2 = resp2.status();
    let body_bytes2 = test::read_body(resp2).await;
    println!("[DEBUG] Status: {}", status2);
    println!("[DEBUG] Body: {}", String::from_utf8_lossy(&body_bytes2));
    assert!(status2.is_success());
    let cache_key = format!("activity:{}", activity_id);
    let cached: Option<String> = redis.get(&cache_key).await.unwrap();
    assert!(cached.is_some());
}

#[actix_rt::test]
async fn test_user_cache_miss_and_invalid_id() {
    dotenv().ok();
    let database_url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let redis_url = std::env::var("TEST_REDIS_URL").expect("TEST_REDIS_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("DB connect");
    let redis = RedisClient::new(&redis_url).await.expect("Redis connect");
    truncate_tables(&pool).await;
    let random_id = Uuid::new_v4();
    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .app_data(actix_web::web::Data::new(redis.clone()))
            .configure(init_routes_no_auth),
    )
    .await;
    let req = test::TestRequest::get()
        .uri(&format!("/api/user/{}", random_id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    let body_bytes = test::read_body(resp).await;
    println!("[DEBUG] Status: {}", status);
    println!("[DEBUG] Body: {}", String::from_utf8_lossy(&body_bytes));
    assert_eq!(status.as_u16(), 404);
    let cache_key = format!("user:{}", random_id);
    let cached: Option<String> = redis.get(&cache_key).await.unwrap();
    assert!(cached.is_none());
}

#[actix_rt::test]
async fn test_user_cache_ttl_and_invalidation() {
    dotenv().ok();
    use std::time::Duration;
    use tokio::time::sleep;
    let database_url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let redis_url = std::env::var("TEST_REDIS_URL").expect("TEST_REDIS_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("DB connect");
    let redis = RedisClient::new(&redis_url).await.expect("Redis connect");
    truncate_tables(&pool).await;
    let user_id = Uuid::new_v4();
    let username = format!("testuser-{}", user_id);
    insert_user(&pool, user_id, &username).await;
    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .app_data(actix_web::web::Data::new(redis.clone()))
            .configure(init_routes_no_auth),
    )
    .await;
    // GET to populate cache
    let req = test::TestRequest::get()
        .uri(&format!("/api/user/{}", user_id))
        .to_request();
    let resp1 = test::call_service(&app, req).await;
    let status1 = resp1.status();
    let body_bytes = test::read_body(resp1).await;
    println!("[DEBUG] Status: {}", status1);
    println!("[DEBUG] Body: {}", String::from_utf8_lossy(&body_bytes));
    assert!(status1.is_success());
    let cache_key = format!("user:{}", user_id);
    let cached: Option<String> = redis.get(&cache_key).await.unwrap();
    assert!(cached.is_some());
    // Wait for TTL to expire (assuming 2s TTL in handler)
    sleep(Duration::from_secs(3)).await;
    let cached: Option<String> = redis.get(&cache_key).await.unwrap();
    assert!(cached.is_none(), "Cache should expire after TTL");
    // Invalidate cache by deleting user
    sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
        .execute(&pool)
        .await
        .expect("delete user");
    // GET again, should return 404 and not repopulate cache
    let req = test::TestRequest::get()
        .uri(&format!("/api/user/{}", user_id))
        .to_request();
    let resp2 = test::call_service(&app, req).await;
    let status2 = resp2.status();
    let body_bytes2 = test::read_body(resp2).await;
    println!("[DEBUG] Status: {}", status2);
    println!("[DEBUG] Body: {}", String::from_utf8_lossy(&body_bytes2));
    assert_eq!(status2.as_u16(), 404);
    let cached: Option<String> = redis.get(&cache_key).await.unwrap();
    assert!(cached.is_none());
}
