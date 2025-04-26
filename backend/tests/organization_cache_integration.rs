use actix_web::{test, App};
use jsonwebtoken::{encode, EncodingKey, Header};
use personal_github_dashboard::routes::init_routes;
use personal_github_dashboard::utils::config::Config;
use personal_github_dashboard::utils::redis::RedisClient;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[actix_web::test]
async fn test_organization_cache_flow() {
    // Setup
    let config = Config::from_env();
    let pool = PgPool::connect(&config.database_url).await.unwrap();
    let redis = RedisClient::new(&config.redis_url).await.unwrap();

    // Clean up test org if exists
    let test_org_name = "cachetestorg";
    sqlx::query("DELETE FROM organizations WHERE name = $1")
        .bind(test_org_name)
        .execute(&pool)
        .await
        .unwrap();

    // JWT setup
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let claims = Claims {
        sub: "testuser".to_string(),
        exp: 2000000000,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .unwrap();

    // Insert test user (owner) required for FK if needed by org endpoints
    let test_owner_id = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();
    sqlx::query("INSERT INTO users (id, username, email, created_at, updated_at) VALUES ($1, $2, $3, NOW(), NOW()) ON CONFLICT (id) DO NOTHING")
        .bind(test_owner_id)
        .bind("testuser")
        .bind("testuser@example.com")
        .execute(&pool)
        .await
        .unwrap();

    // Start app
    let mut app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .app_data(actix_web::web::Data::new(redis.clone()))
            .configure(init_routes),
    )
    .await;

    // Create organization (POST)
    let req = test::TestRequest::post()
        .uri("/api/organizations")
        .set_json(&serde_json::json!({
            "name": test_org_name,
            "description": "Test org for cache integration"
        }))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;
    println!("POST /api/organizations status: {}", status);
    println!("POST /api/organizations raw body: {:?}", body);
    assert_eq!(status, 201);
    let resp_json: serde_json::Value =
        serde_json::from_slice(&body).expect("POST response is not valid JSON");
    let org_id = resp_json["id"].as_str().unwrap();

    // First GET: should hit DB, populate cache
    let req = test::TestRequest::get()
        .uri(&format!("/api/organizations/{}", org_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;
    println!("GET /api/organizations/{} status: {}", org_id, status);
    println!("GET /api/organizations/{} raw body: {:?}", org_id, body);
    assert_eq!(status, 200);
    let resp1: serde_json::Value =
        serde_json::from_slice(&body).expect("GET response is not valid JSON");
    assert_eq!(resp1["name"], test_org_name);

    // Second GET: should hit cache
    let req = test::TestRequest::get()
        .uri(&format!("/api/organizations/{}", org_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;
    println!(
        "GET (cache) /api/organizations/{} status: {}",
        org_id, status
    );
    println!(
        "GET (cache) /api/organizations/{} raw body: {:?}",
        org_id, body
    );
    assert_eq!(status, 200);
    let resp2: serde_json::Value =
        serde_json::from_slice(&body).expect("GET (cache) response is not valid JSON");
    assert_eq!(resp2["name"], test_org_name);

    // Update organization description (PUT)
    let req = test::TestRequest::put()
        .uri(&format!("/api/organizations/{}/description", org_id))
        .set_json(&serde_json::json!({"description": "Updated description"}))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;
    println!(
        "PUT /api/organizations/{}/description status: {}",
        org_id, status
    );
    println!(
        "PUT /api/organizations/{}/description raw body: {:?}",
        org_id, body
    );
    assert_eq!(status, 200);
    let resp: serde_json::Value =
        serde_json::from_slice(&body).expect("PUT response is not valid JSON");
    assert_eq!(resp["description"], "Updated description");

    // Delete organization (DELETE)
    let req = test::TestRequest::delete()
        .uri(&format!("/api/organizations/{}", org_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    println!("DELETE /api/organizations/{} status: {}", org_id, status);
    assert_eq!(status, 204);

    // GET after delete: should return 404
    let req = test::TestRequest::get()
        .uri(&format!("/api/organizations/{}", org_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    println!(
        "GET (after delete) /api/organizations/{} status: {}",
        org_id, status
    );
    assert_eq!(status, 404);
}

// Similar tests: user_cache_integration.rs, repository_cache_integration.rs, activity_cache_integration.rs
