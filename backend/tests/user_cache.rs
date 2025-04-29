use actix_web::{App, test, web};
use personal_github_dashboard::models::user::User;
use personal_github_dashboard::utils::redis::RedisClient;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

#[actix_rt::test]
async fn test_user_cache_behavior() {
    // Setup: pool, redis, app
    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    let redis = RedisClient::new(&std::env::var("REDIS_URL").unwrap())
        .await
        .unwrap();
    let user_id = Uuid::new_v4();
    let test_user = User {
        id: user_id,
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        avatar_url: None,
        created_at: chrono::Utc::now(),
    };
    // Insert user into DB
    sqlx::query("INSERT INTO users (id, username, email, created_at) VALUES ($1, $2, $3, $4)")
        .bind(user_id)
        .bind(&test_user.username)
        .bind(&test_user.email)
        .bind(test_user.created_at)
        .execute(&pool)
        .await
        .unwrap();

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
    let resp: User = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp.id, user_id);

    // 2. Second request: should hit cache
    let req = test::TestRequest::get()
        .uri(&format!("/api/user/{}", user_id))
        .to_request();
    let resp2: User = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp2.id, user_id);

    // 3. Delete user, cache should be invalidated
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id)
        .execute(&pool)
        .await
        .unwrap();
    let _ = redis.set(&format!("user:{}", user_id), "", 1).await;
    let req = test::TestRequest::get()
        .uri(&format!("/api/user/{}", user_id))
        .to_request();
    let resp3 = test::call_service(&app, req).await;
    assert_eq!(resp3.status(), 404);
}
