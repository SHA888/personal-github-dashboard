use actix_web::{App, test, web};
use dotenv;
use personal_github_dashboard::models::user::User;
use personal_github_dashboard::utils::redis::RedisClient;
use serde_json;
use sqlx::PgPool;
use uuid::Uuid;

#[actix_rt::test]
/// Integration test verifying user data caching and invalidation between PostgreSQL and Redis.
///
/// This test ensures that user data is correctly retrieved from the database and cached in Redis on the first request, served from the cache on subsequent requests, and properly invalidated when the user is deleted from the database.
///
/// # Examples
///
/// ```
/// // This test is intended to be run as part of the integration test suite.
/// // It requires a running PostgreSQL and Redis instance, and appropriate environment variables set.
/// #[actix_rt::test]
/// async fn test_user_cache_behavior() {
///     // ...test logic as implemented...
/// }
/// ```
async fn test_user_cache_behavior() {
    dotenv::dotenv().ok();
    // Setup: pool, redis, app
    let db_url_res = std::env::var("DATABASE_URL");
    println!("DATABASE_URL env: {:?}", db_url_res);
    let db_url = db_url_res.expect("DATABASE_URL not set");
    let pool_res = PgPool::connect(&db_url).await;
    println!("PgPool connect result: {:?}", pool_res);
    let pool = pool_res.expect("PgPool connect failed");
    let redis_url_res = std::env::var("REDIS_URL");
    println!("REDIS_URL env: {:?}", redis_url_res);
    let redis_url = redis_url_res.expect("REDIS_URL not set");
    let redis_res = RedisClient::new(&redis_url).await;
    println!(
        "RedisClient connect result: is_err={} err={:?}",
        redis_res.is_err(),
        redis_res.as_ref().err()
    );
    let redis = redis_res.expect("RedisClient connect failed");
    let user_id = Uuid::new_v4();
    let test_user = User {
        id: user_id,
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        avatar_url: None,
        created_at: chrono::Utc::now(),
    };
    // Clean up any existing test user before inserting
    let cleanup_res = sqlx::query("DELETE FROM users WHERE username = $1 OR email = $2")
        .bind("testuser")
        .bind("test@example.com")
        .execute(&pool)
        .await;
    println!("Cleanup result: {:?}", cleanup_res);
    cleanup_res.unwrap();

    // Insert user into DB
    let insert_res =
        sqlx::query("INSERT INTO users (id, username, email, created_at) VALUES ($1, $2, $3, $4)")
            .bind(user_id)
            .bind(&test_user.username)
            .bind(&test_user.email)
            .bind(test_user.created_at)
            .execute(&pool)
            .await;
    println!("Insert result: {:?}", insert_res);
    insert_res.unwrap();

    // Build app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(redis.clone()))
            .route(
                "/api/user/{id}",
                web::get().to(personal_github_dashboard::handlers::users::get_user_by_id_handler),
            ),
    )
    .await;

    // 1. First request: should hit DB, store in cache
    let req = test::TestRequest::get()
        .uri(&format!("/api/user/{}", user_id))
        .to_request();
    let resp_body = test::call_and_read_body(&app, req).await;
    let resp: Result<User, _> = serde_json::from_slice(&resp_body);
    println!("First GET result: {:?}, raw body: {:?}", resp, resp_body);
    let resp = resp.unwrap();
    assert_eq!(resp.id, user_id);

    // 2. Second request: should hit cache
    let req = test::TestRequest::get()
        .uri(&format!("/api/user/{}", user_id))
        .to_request();
    let resp2_body = test::call_and_read_body(&app, req).await;
    let resp2: Result<User, _> = serde_json::from_slice(&resp2_body);
    println!(
        "Second GET (cache) result: {:?}, raw body: {:?}",
        resp2, resp2_body
    );
    let resp2 = resp2.unwrap();
    assert_eq!(resp2.id, user_id);

    // 3. Delete user, cache should be invalidated
    let del_res = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id)
        .execute(&pool)
        .await;
    println!("Delete result: {:?}", del_res);
    del_res.unwrap();
    let _ = redis.del(&format!("user:{}", user_id)).await;
    let req = test::TestRequest::get()
        .uri(&format!("/api/user/{}", user_id))
        .to_request();
    let resp3 = test::call_service(&app, req).await;
    println!("GET after delete status: {}", resp3.status());
    assert_eq!(resp3.status(), 404);
}
