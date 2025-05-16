use actix_web::{App, http::StatusCode, test, web};
use dotenv::dotenv;
use personal_github_dashboard::routes::init_routes_test::init_routes_no_auth;
use personal_github_dashboard::utils::redis::RedisClient;
use rand::Rng;
use serial_test::serial;
use sqlx::PgPool;
use std::sync::Once;
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

/// Returns a new PostgreSQL connection pool for integration tests.
///
/// # Examples
///
/// ```
/// let pool = get_test_pool().await;
/// // Use `pool` to execute test queries.
/// ```
async fn get_test_pool() -> PgPool {
    let database_url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    PgPool::connect(&database_url).await.expect("DB connect")
}

/// Returns a new Redis client for integration tests.
///
/// # Examples
///
/// ```
/// let redis = get_test_redis().await;
/// // Use `redis` to interact with the test Redis instance.
/// ```
async fn get_test_redis() -> RedisClient {
    let redis_url = std::env::var("TEST_REDIS_URL").expect("TEST_REDIS_URL must be set");
    RedisClient::new(&redis_url).await.expect("Redis connect")
}

/// Removes all data from the `activities`, `repositories`, `organizations`, `users`, and `oauth_tokens` tables, resetting identity sequences and cascading deletions.
///
/// This ensures a clean database state before each test by truncating the relevant tables.
///
/// # Examples
///
/// ```
/// let pool = get_test_pool().await;
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

async fn insert_user(pool: &PgPool, github_id: i64, username: &str) -> Uuid {
    let rec = sqlx::query!(
        "INSERT INTO users (github_id, username, email) VALUES ($1, $2, $3) RETURNING id",
        github_id as i64,
        username,
        &format!("{}@example.com", username)
    )
    .fetch_one(pool)
    .await
    .expect("insert user");
    rec.id
}

async fn insert_organization(pool: &PgPool, name: &str) -> Uuid {
    let rec = sqlx::query!(
        "INSERT INTO organizations (name) VALUES ($1) RETURNING id",
        name
    )
    .fetch_one(pool)
    .await
    .expect("insert organization");
    rec.id
}

async fn insert_repository(
    pool: &PgPool,
    org_id: Option<Uuid>,
    owner_id: Uuid,
    name: &str,
) -> Uuid {
    let rec = sqlx::query!(
        r#"
        INSERT INTO repositories (org_id, owner_id, name, is_private)
        VALUES ($1, $2, $3, false)
        RETURNING id
        "#,
        org_id,
        owner_id,
        name
    )
    .fetch_one(pool)
    .await
    .expect("insert repository");
    rec.id
}

/// Inserts a test activity record into the database with the specified IDs and sample data.
///
/// The activity is created with a fixed type ("testtype"), current timestamp, and a sample JSON payload.
/// Used to set up test data for integration tests involving activities.
///
/// # Examples
///
/// ```
/// let pool = get_test_pool().await;
/// let user_id = Uuid::new_v4();
/// let repo_id = Some(Uuid::new_v4());
/// insert_activity(&pool, user_id, repo_id).await;
/// ```
async fn insert_activity(pool: &PgPool, user_id: Uuid, repo_id: Option<Uuid>) -> Uuid {
    let rec = sqlx::query!(
        r#"
        INSERT INTO activities (user_id, repo_id, type, data)
        VALUES ($1, $2, 'testtype', '{}')
        RETURNING id
        "#,
        user_id,
        repo_id,
    )
    .fetch_one(pool)
    .await
    .expect("insert activity");
    rec.id
}

#[actix_rt::test]
#[serial]
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
    // SAFETY: Setting environment variables in tests is required to isolate test state and avoid conflicts between tests running in parallel.
    // This is safe here because tests are run serially (see #[serial] attribute) and only test configuration is affected.
    unsafe {
        std::env::set_var("USER_CACHE_TTL", "2");
    }
    let pool = get_test_pool().await;
    let redis = get_test_redis().await;
    truncate_tables(&pool).await;

    // Generate a random github_id
    let mut rng = rand::thread_rng();
    let github_id = rng.gen_range(1000000..9999999);
    let username = format!("testuser-{}", github_id);

    // Insert user and get the database ID
    let user_id = insert_user(&pool, github_id, &username).await;

    // Get the user from the database to ensure it exists
    let user = sqlx::query!("SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(&pool)
        .await
        .expect("User should exist in database");

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
    assert!(status1.is_success(), "First request should succeed");

    // Check cache directly - use user_id as the cache key
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
    assert!(status2.is_success(), "Second request should succeed");

    // Cache should still exist
    let cached: Option<String> = redis.get(&cache_key).await.unwrap();
    assert!(
        cached.is_some(),
        "Cache should still exist after second fetch"
    );
}

#[actix_rt::test]
#[serial]
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

    let pool = get_test_pool().await;
    truncate_tables(&pool).await;

    // Generate a random name for organization
    let mut rng = rand::thread_rng();
    let org_num = rng.gen_range(100000..999999);
    let name = format!("testorg-{}", org_num);

    // Insert organization and get the database ID
    let org_id = insert_organization(&pool, &name).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(get_test_redis().await))
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
    let redis = get_test_redis().await;
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

    // Verify the response is the same as the first request
    let org1: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let org2: serde_json::Value = serde_json::from_slice(&body_bytes2).unwrap();
    assert_eq!(org1, org2, "Cached response should match first response");

    // Verify the cache contains the expected data
    let cached_org: serde_json::Value = serde_json::from_str(&cached.unwrap()).unwrap();
    assert_eq!(cached_org["id"].as_str().unwrap(), org_id.to_string());
    assert_eq!(cached_org["name"], name);
}

#[actix_rt::test]
#[serial]
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
    // SAFETY: Setting environment variables in tests is required to isolate test state and avoid conflicts between tests running in parallel.
    // This is safe here because tests are run serially (see #[serial] attribute) and only test configuration is affected.
    unsafe {
        std::env::set_var("USER_CACHE_TTL", "2");
    }
    let pool = get_test_pool().await;
    let redis = get_test_redis().await;
    truncate_tables(&pool).await;

    // Generate random IDs
    let mut rng = rand::thread_rng();
    let user_github_id = rng.gen_range(1000000..9999999);
    let org_github_id = rng.gen_range(1000000..9999999);
    let repo_github_id = rng.gen_range(1000000..9999999);

    // Insert user and get database ID
    let username = format!("testuser-{}", user_github_id);
    insert_user(&pool, user_github_id, &username).await;
    let user_rec = sqlx::query!("SELECT id FROM users WHERE github_id = $1", user_github_id)
        .fetch_one(&pool)
        .await
        .expect("get user id");

    // Insert organization and get database ID
    let org_name = format!("testorg-{}", org_github_id);
    let org_id = insert_organization(&pool, &org_name).await;

    // Insert repository
    let repo_name = format!("testrepo-{}", repo_github_id);
    let repo_id = insert_repository(&pool, Some(org_id), user_rec.id, &repo_name).await;

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
#[serial]
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

    let pool = get_test_pool().await;
    truncate_tables(&pool).await;

    // Generate random IDs
    let mut rng = rand::thread_rng();
    let user_github_id = rng.gen_range(100000..999999) as i64;
    let repo_github_id = rng.gen_range(100000..999999) as i64;

    // Insert user and get database ID
    let username = format!("testuser-{}", user_github_id);
    let user_id = insert_user(&pool, user_github_id, &username).await;

    // Insert repository
    let repo_name = format!("testrepo-{}", repo_github_id);
    let repo_id = insert_repository(
        &pool, None, // No org for this test
        user_id, &repo_name,
    )
    .await;

    // Insert activity
    let activity_id = insert_activity(&pool, user_id, Some(repo_id)).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(get_test_redis().await))
            .configure(init_routes_no_auth),
    )
    .await;

    // First request - should hit the database
    let req = test::TestRequest::get()
        .uri(&format!("/api/activity/{}", activity_id))
        .to_request();
    let resp1 = test::call_service(&app, req).await;
    let status1 = resp1.status();
    let body_bytes = test::read_body(resp1).await;
    println!("[DEBUG] Status: {}", status1);
    println!("[DEBUG] Body: {}", String::from_utf8_lossy(&body_bytes));
    assert!(status1.is_success());

    // Check cache directly
    let redis = get_test_redis().await;
    let cache_key = format!("activity:{}", activity_id);
    let cached: Option<String> = redis.get(&cache_key).await.unwrap();
    assert!(
        cached.is_some(),
        "Activity should be cached after first fetch"
    );

    // Second request - should hit the cache
    let req = test::TestRequest::get()
        .uri(&format!("/api/activity/{}", activity_id))
        .to_request();
    let resp2 = test::call_service(&app, req).await;
    let status2 = resp2.status();
    let body_bytes2 = test::read_body(resp2).await;
    println!("[DEBUG] Status: {}", status2);
    println!("[DEBUG] Body: {}", String::from_utf8_lossy(&body_bytes2));
    assert!(status2.is_success());

    // Verify the response is the same as the first request
    let activity1: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let activity2: serde_json::Value = serde_json::from_slice(&body_bytes2).unwrap();
    assert_eq!(
        activity1, activity2,
        "Cached response should match first response"
    );

    // Verify the cache contains the expected data
    let cached_activity: serde_json::Value = serde_json::from_str(&cached.unwrap()).unwrap();
    assert_eq!(
        cached_activity["id"].as_str().unwrap(),
        activity_id.to_string()
    );
    assert_eq!(
        cached_activity["user_id"].as_str().unwrap(),
        user_id.to_string()
    );
    assert_eq!(
        cached_activity["repo_id"].as_str().unwrap(),
        repo_id.to_string()
    );
}

#[actix_rt::test]
#[serial]
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

    let pool = get_test_pool().await;
    let redis = get_test_redis().await;
    truncate_tables(&pool).await;

    // Generate a random UUID that doesn't exist in the database
    let non_existent_user_id = Uuid::new_v4();

    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(redis.clone()))
            .configure(init_routes_no_auth),
    )
    .await;

    // Make request for non-existent user
    let req = test::TestRequest::get()
        .uri(&format!("/api/user/{}", non_existent_user_id))
        .to_request();

    // Send request and verify 404 response
    let resp = test::call_service(&app, req).await;
    assert_eq!(
        resp.status(),
        StatusCode::NOT_FOUND,
        "Request for non-existent user should return 404"
    );

    // Verify no cache entry was created
    let cache_key = format!("user:{}", non_existent_user_id);
    let cached: Option<String> = redis.get(&cache_key).await.unwrap();
    assert!(
        cached.is_none(),
        "Cache should not be set for non-existent users"
    );
}

#[actix_rt::test]
#[serial]
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

    let pool = get_test_pool().await;
    let redis = get_test_redis().await;
    truncate_tables(&pool).await;

    // Generate a random github_id and create a user
    let mut rng = rand::thread_rng();
    let github_id = rng.gen_range(100000..999999) as i64;
    let username = format!("testuser-{}", github_id);

    // Insert user and get the user ID
    let user_id = insert_user(&pool, github_id, &username).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(redis.clone()))
            .configure(init_routes_no_auth),
    )
    .await;

    // First request to populate cache
    let req = test::TestRequest::get()
        .uri(&format!("/api/user/{}", user_id))
        .to_request();
    let resp1 = test::call_service(&app, req).await;
    let status1 = resp1.status();
    let body_bytes = test::read_body(resp1).await;
    println!("[DEBUG] Status: {}", status1);
    println!("[DEBUG] Body: {}", String::from_utf8_lossy(&body_bytes));
    assert!(status1.is_success(), "First request should succeed");

    // Verify cache is set
    let cache_key = format!("user:{}", user_id);
    let cached: Option<String> = redis.get(&cache_key).await.unwrap();
    assert!(cached.is_some(), "Cache should be set after first fetch");

    // Wait for TTL to expire (assuming 2s TTL in handler)
    println!("Waiting for cache TTL to expire...");
    sleep(Duration::from_secs(3)).await;

    // Cache should be expired
    let cached: Option<String> = redis.get(&cache_key).await.unwrap();
    assert!(cached.is_none(), "Cache should be expired after TTL");

    // Delete the user from the database
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
        .execute(&pool)
        .await;
    assert!(result.is_ok(), "Should be able to delete user");

    // Try to fetch the deleted user - should not repopulate cache
    let req = test::TestRequest::get()
        .uri(&format!("/api/user/{}", user_id))
        .to_request();
    let resp2 = test::call_service(&app, req).await;
    let status2 = resp2.status();
    assert_eq!(
        status2,
        StatusCode::NOT_FOUND,
        "Should return 404 for deleted user"
    );

    // Verify cache is still empty
    let cached: Option<String> = redis.get(&cache_key).await.unwrap();
    assert!(
        cached.is_none(),
        "Cache should not be repopulated for deleted user"
    );
}
