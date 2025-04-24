use actix_web::App;
use chrono::{Duration, Utc};
use dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use personal_github_dashboard::utils::redis::RedisClient;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[actix_web::test]
async fn test_user_cache_flow() {
    dotenv::dotenv().ok();
    // Setup test pool/redis (point to test DB/Redis)
    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    let redis = RedisClient::new(&std::env::var("REDIS_URL").unwrap())
        .await
        .unwrap();
    let mut app = actix_web::test::init_service({
        use personal_github_dashboard::routes::init_routes;
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .app_data(actix_web::web::Data::new(redis.clone()))
            .configure(init_routes)
    })
    .await;

    // Clean up users table before test
    sqlx::query("DELETE FROM users WHERE username = $1 OR email = $2")
        .bind("cachetest")
        .bind("cache@test.com")
        .execute(&pool)
        .await
        .unwrap();

    // Generate JWT for Authorization header
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let claims = Claims {
        sub: "cachetest".to_string(),
        exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .unwrap();

    // Create user (auth required for registration)
    let req = actix_web::test::TestRequest::post()
        .uri("/api/user")
        .set_json(&serde_json::json!({"username":"cachetest","email":"cache@test.com"}))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = actix_web::test::call_service(&mut app, req).await;
    let status = resp.status();
    println!("POST /api/user status: {}", status);
    let body = actix_web::test::read_body(resp).await;
    println!("POST /api/user raw body: {:?}", body);
    assert_eq!(status, 201);
    let resp_json: serde_json::Value =
        serde_json::from_slice(&body).expect("Response body is not valid JSON");
    let user_id = resp_json["id"].as_str().unwrap();

    // First GET: should hit DB, populate cache
    let req = actix_web::test::TestRequest::get()
        .uri(&format!("/api/user/{}", user_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = actix_web::test::call_service(&mut app, req).await;
    let status = resp.status();
    let body = actix_web::test::read_body(resp).await;
    println!("GET /api/user/{} status: {}", user_id, status);
    println!("GET /api/user/{} raw body: {:?}", user_id, body);
    assert_eq!(status, 200);
    let resp1: serde_json::Value =
        serde_json::from_slice(&body).expect("GET response is not valid JSON");
    assert_eq!(resp1["username"], "cachetest");

    // Second GET: should hit cache
    let req = actix_web::test::TestRequest::get()
        .uri(&format!("/api/user/{}", user_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = actix_web::test::call_service(&mut app, req).await;
    let status = resp.status();
    let body = actix_web::test::read_body(resp).await;
    println!("GET (cache) /api/user/{} status: {}", user_id, status);
    println!("GET (cache) /api/user/{} raw body: {:?}", user_id, body);
    assert_eq!(status, 200);
    let resp2: serde_json::Value =
        serde_json::from_slice(&body).expect("GET (cache) response is not valid JSON");
    assert_eq!(resp2["username"], "cachetest");

    // Update user avatar
    let req = actix_web::test::TestRequest::put()
        .uri(&format!("/api/user/{}/avatar", user_id))
        .set_json(&serde_json::json!({"avatar_url":"https://example.com/avatar.png"}))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = actix_web::test::call_service(&mut app, req).await;
    let status = resp.status();
    let body = actix_web::test::read_body(resp).await;
    println!("PUT /api/user/{}/avatar status: {}", user_id, status);
    println!("PUT /api/user/{}/avatar raw body: {:?}", user_id, body);
    assert_eq!(status, 200);
    let resp: serde_json::Value =
        serde_json::from_slice(&body).expect("PUT response is not valid JSON");
    assert_eq!(resp["avatar_url"], "https://example.com/avatar.png");

    // Delete user
    let req = actix_web::test::TestRequest::delete()
        .uri(&format!("/api/user/{}", user_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = actix_web::test::call_service(&mut app, req).await;
    let status = resp.status();
    println!("DELETE /api/user/{} status: {}", user_id, status);
    assert_eq!(status, 204);

    // GET after delete: should return 404
    let req = actix_web::test::TestRequest::get()
        .uri(&format!("/api/user/{}", user_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = actix_web::test::call_service(&mut app, req).await;
    let status = resp.status();
    println!(
        "GET (after delete) /api/user/{} status: {}",
        user_id, status
    );
    assert_eq!(status, 404);
}

// Similar tests can be found in: repository_cache_integration.rs, organization_cache_integration.rs, activity_cache_integration.rs
