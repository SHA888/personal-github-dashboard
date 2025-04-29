use actix_web::{test, App};
use dotenv::dotenv;
use env_logger;
use log;
use once_cell::sync::Lazy;
use personal_github_dashboard::routes::init_routes_test::init_routes_no_auth;
use personal_github_dashboard::utils::redis::RedisClient;
use serde_json;
use sqlx::PgPool;
use std::sync::{Arc, Once};
use uuid::Uuid;

// Logger initialization for integration tests
static INIT: Once = Once::new();

/// ```
fn init_logger() {
    INIT.call_once(|| {
        env_logger::Builder::from_default_env()
            .is_test(true)
            .filter_level(log::LevelFilter::Debug)
            .init();
    });
}

// --- Shared test state ---
static TEST_POOL: Lazy<Arc<PgPool>> = Lazy::new(|| {
    let database_url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let rt = tokio::runtime::Runtime::new().unwrap();
    let pool = rt
        .block_on(PgPool::connect(&database_url))
        .expect("DB connect");
    Arc::new(pool)
});

static TEST_REDIS: Lazy<Arc<RedisClient>> = Lazy::new(|| {
    let redis_url = std::env::var("TEST_REDIS_URL").expect("TEST_REDIS_URL must be set");
    let rt = tokio::runtime::Runtime::new().unwrap();
    let redis = rt
        .block_on(RedisClient::new(&redis_url))
        .expect("Redis connect");
    Arc::new(redis)
});

/// Returns a clone of the shared PostgreSQL connection pool used for integration tests.
///
/// # Examples
///
/// ```
/// let pool = get_test_pool();
/// // Use `pool` to execute test queries.
/// ```
fn get_test_pool() -> Arc<PgPool> {
    TEST_POOL.clone()
}

/// Returns a clone of the shared Redis client used for integration tests.
///
/// # Examples
///
/// ```
/// let redis = get_test_redis();
/// // Use `redis` to interact with the test Redis instance.
/// ```
fn get_test_redis() -> Arc<RedisClient> {
    TEST_REDIS.clone()
}

/// Removes all data from the `activities`, `repositories`, `organizations`, and `users` tables, resetting identity sequences and cascading deletions.
///
/// This ensures a clean database state before each test by truncating the relevant tables.
///
/// # Examples
///
/// ```
/// let pool = get_test_pool();
/// truncate_tables(&pool).await;
/// ```
async fn truncate_tables(pool: &PgPool) {
    sqlx::query!(
        "TRUNCATE TABLE activities, repositories, organizations, users RESTART IDENTITY CASCADE;"
    )
    .execute(pool)
    .await
    .expect("truncate tables");
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

/// Inserts a test activity record into the database with the specified IDs and sample data.
///
/// The activity is created with a fixed type ("testtype"), current timestamp, and a sample JSON payload.
/// Used to set up test data for integration tests involving activities.
///
/// # Examples
///
/// ```
/// let pool = get_test_pool();
/// let activity_id = Uuid::new_v4();
/// let user_id = Uuid::new_v4();
/// let repo_id = Some(Uuid::new_v4());
/// insert_activity(&pool, activity_id, user_id, repo_id).await;
/// ```
async fn insert_activity(pool: &PgPool, activity_id: Uuid, user_id: Uuid, repo_id: Option<Uuid>) {
async fn insert_activity(pool: &PgPool, activity_id: Uuid, user_id: Uuid, repo_id: Option<Uuid>) {
    sqlx::query!("INSERT INTO activities (id, user_id, repo_id, type, timestamp, data, created_at) VALUES ($1, $2, $3, $4, NOW(), $5, NOW())",
        activity_id, user_id, repo_id, "testtype", serde_json::json!({"k":"v"}))
        .execute(pool).await.expect("insert activity");
}

#[actix_rt::test]
/// Tests that user data is cached in Redis after the first API fetch and that subsequent requests retrieve the data from the cache.
///
/// This test inserts a user, fetches the user via the API to populate the cache, verifies the cache is set, and ensures a second fetch results in a cache hit.
///
/// # Examples
///
/// ```
/// // Runs as part of the integration test suite; not intended for direct invocation.
/// tokio_test::block_on(test_user_caching());
/// ```
async fn test_user_caching() {
    init_logger();
    dotenv().ok();
    std::env::set_var("USER_CACHE_TTL", "2");
    let pool = get_test_pool();
    let redis = get_test_redis();
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
    // First GET - should populate cache
    let req = test::TestRequest::get()
        .uri(&format!("/api/user/{}", user_id))
        .to_request();
    let resp1 = test::call_service(&app, req).await;
    let status1 = resp1.status();
    let body_bytes = test::read_body(resp1).await;
    println!("[DEBUG] Status: {}", status1);
    println!("[DEBUG] Body: {}", String::from_utf8_lossy(&body_bytes));
    assert!(status1.is_success());
    // Check cache directly
    let cache_key = format!("user:{}", user_id);
    let cached: Option<String> = redis.get(&cache_key).await.unwrap();
    assert!(cached.is_some(), "Cache should be set after first fetch");
    // Second GET - should hit cache
    let req = test::TestRequest::get()
        .uri(&format!("/api/user/{}", user_id))
        .to_request();
    let resp2 = test::call_service(&app, req).await;
    let status2 = resp2.status();
    let body_bytes2 = test::read_body(resp2).await;
    println!("[DEBUG] Status: {}", status2);
    println!("[DEBUG] Body: {}", String::from_utf8_lossy(&body_bytes2));
    assert!(status2.is_success());
    // Cache should still exist
    let cached: Option<String> = redis.get(&cache_key).await.unwrap();
    assert!(
        cached.is_some(),
        "Cache should still exist after second fetch"
    );
}

#[actix_rt::test]
/// Tests that organization data is cached in Redis after the first API fetch and that subsequent fetches use the cache.
///
/// This test inserts an organization, fetches it via the API to populate the cache, verifies the cache is set,
/// then fetches it again to ensure the cache is still present.
///
/// # Examples
///
/// ```
/// // Runs as part of the integration test suite; not intended for direct invocation.
/// // Verifies organization caching behavior.
/// ```
async fn test_organization_caching() {
    init_logger();
    dotenv().ok();
    std::env::set_var("USER_CACHE_TTL", "2");
    let pool = get_test_pool();
    let redis = get_test_redis();
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
    // First GET - should populate cache
    let req = test::TestRequest::get()
        .uri(&format!("/api/organization/{}", org_id))
        .to_request();
    let resp1 = test::call_service(&app, req).await;
    let status1 = resp1.status();
    let body_bytes = test::read_body(resp1).await;
    println!("[DEBUG] Status: {}", status1);
    println!("[DEBUG] Body: {}", String::from_utf8_lossy(&body_bytes));
    assert!(status1.is_success());
    // Check cache directly
    let cache_key = format!("org:{}", org_id);
    let cached: Option<String> = redis.get(&cache_key).await.unwrap();
    assert!(cached.is_some(), "Cache should be set after first fetch");
    // Second GET - should hit cache
    let req = test::TestRequest::get()
        .uri(&format!("/api/organization/{}", org_id))
        .to_request();
    let resp2 = test::call_service(&app, req).await;
    let status2 = resp2.status();
    let body_bytes2 = test::read_body(resp2).await;
    println!("[DEBUG] Status: {}", status2);
    println!("[DEBUG] Body: {}", String::from_utf8_lossy(&body_bytes2));
    assert!(status2.is_success());
    // Cache should still exist
    let cached: Option<String> = redis.get(&cache_key).await.unwrap();
    assert!(
        cached.is_some(),
        "Cache should still exist after second fetch"
    );
}

#[actix_rt::test]
/// Tests that repository data is cached in Redis after the first fetch and remains available on subsequent requests.
///
/// This integration test inserts a user, organization, and repository into the database, then fetches the repository via the API.
/// It verifies that the repository data is cached in Redis after the first request and that the cache persists after a second request.
///
/// # Examples
///
/// ```
/// // Runs as part of the integration test suite; not intended for direct invocation.
/// tokio_test::block_on(test_repository_caching());
/// ```
async fn test_repository_caching() {
    init_logger();
    dotenv().ok();
    std::env::set_var("USER_CACHE_TTL", "2");
    let pool = get_test_pool();
    let redis = get_test_redis();
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
    // First GET - should populate cache
    let req = test::TestRequest::get()
        .uri(&format!("/api/repository/{}", repo_id))
        .to_request();
    let resp1 = test::call_service(&app, req).await;
    let status1 = resp1.status();
    let body_bytes = test::read_body(resp1).await;
    println!("[DEBUG] Status: {}", status1);
    println!("[DEBUG] Body: {}", String::from_utf8_lossy(&body_bytes));
    assert!(status1.is_success());
    // Check cache directly
    let cache_key = format!("repo:{}", repo_id);
    let cached: Option<String> = redis.get(&cache_key).await.unwrap();
    assert!(cached.is_some(), "Cache should be set after first fetch");
    // Second GET - should hit cache
    let req = test::TestRequest::get()
        .uri(&format!("/api/repository/{}", repo_id))
        .to_request();
    let resp2 = test::call_service(&app, req).await;
    let status2 = resp2.status();
    let body_bytes2 = test::read_body(resp2).await;
    println!("[DEBUG] Status: {}", status2);
    println!("[DEBUG] Body: {}", String::from_utf8_lossy(&body_bytes2));
    assert!(status2.is_success());
    // Cache should still exist
    let cached: Option<String> = redis.get(&cache_key).await.unwrap();
    assert!(
        cached.is_some(),
        "Cache should still exist after second fetch"
    );
}

#[actix_rt::test]
/// Tests that activity data is cached in Redis after the first API fetch and remains available on subsequent requests.
///
/// This test inserts a user, repository, and activity into the database, fetches the activity via the API, and verifies that the activity is cached in Redis. It then fetches the activity again to confirm the cache is still present.
///
/// # Examples
///
/// ```
/// // Runs as part of the integration test suite; not intended for direct invocation.
/// tokio_test::block_on(test_activity_caching());
/// ```
async fn test_activity_caching() {
    init_logger();
    dotenv().ok();
    std::env::set_var("USER_CACHE_TTL", "2");
    let pool = get_test_pool();
    let redis = get_test_redis();
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
/// Tests that requesting a non-existent user returns a 404 response and does not populate the Redis cache.
///
/// This test sends a GET request for a randomly generated user ID that does not exist in the database,
/// verifies that the response status is 404 Not Found, and asserts that the corresponding Redis cache key
/// is not set.
///
/// # Examples
///
/// ```
/// // This test is intended to be run as part of the integration test suite.
/// // It does not require any setup, as it generates a random user ID.
/// tokio_test::block_on(async {
///     test_user_cache_miss_and_invalid_id().await;
/// });
/// ```
async fn test_user_cache_miss_and_invalid_id() {
    init_logger();
    dotenv().ok();
    std::env::set_var("USER_CACHE_TTL", "2");
    let pool = get_test_pool();
    let redis = get_test_redis();
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
/// Tests that the user cache entry expires after the configured TTL and is not repopulated after the user is deleted.
///
/// This test verifies that fetching a user populates the cache, the cache entry expires after the TTL, and subsequent requests for a deleted user do not repopulate the cache.
///
/// # Examples
///
/// ```
/// // This is an integration test and is run as part of the test suite.
/// // It does not return a value.
/// ```
async fn test_user_cache_ttl_and_invalidation() {
    init_logger();
    use std::time::Duration;
    use tokio::time::sleep;
    dotenv().ok();
    std::env::set_var("USER_CACHE_TTL", "2");
    let pool = get_test_pool();
    let redis = get_test_redis();
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
    assert!(cached.is_some(), "Cache should be set after first fetch");
    // Wait for TTL to expire (assuming 2s TTL in handler)
    sleep(Duration::from_secs(3)).await;
    let cached: Option<String> = redis.get(&cache_key).await.unwrap();
    assert!(cached.is_none(), "Cache should expire after TTL");
    // Invalidate cache by deleting user
    sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
        .execute(&*pool)
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
