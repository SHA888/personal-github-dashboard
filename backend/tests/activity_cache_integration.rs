use actix_web::{App, test};
use jsonwebtoken::{EncodingKey, Header, encode};
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
async fn test_activity_cache_flow() {
    // Setup
    let config = Config::from_env();
    let pool = PgPool::connect(&config.database_url).await.unwrap();
    let redis = RedisClient::new(&config.redis_url).await.unwrap();

    // Clean up test activity if exists
    let test_activity_type = "cachetest";
    let test_user_id = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();
    let test_repo_id = Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap();
    sqlx::query("DELETE FROM activities WHERE type = $1")
        .bind(test_activity_type)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM repositories WHERE id = $1")
        .bind(test_repo_id)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(test_user_id)
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

    // Insert test user & repo required for FKs
    sqlx::query("INSERT INTO users (id, username, email, created_at, updated_at) VALUES ($1, $2, $3, NOW(), NOW()) ON CONFLICT (id) DO NOTHING")
        .bind(test_user_id)
        .bind("testuser")
        .bind("testuser@example.com")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO repositories (id, name, owner_id, created_at, updated_at, is_private) VALUES ($1, $2, $3, NOW(), NOW(), false) ON CONFLICT (id) DO NOTHING")
        .bind(test_repo_id)
        .bind("cachetestrepo")
        .bind(test_user_id)
        .execute(&pool)
        .await
        .unwrap();

    // Insert test org required for org-level activity
    let test_org_id = Uuid::parse_str("33333333-3333-3333-3333-333333333333").unwrap();
    sqlx::query("INSERT INTO organizations (id, name, created_at, updated_at) VALUES ($1, $2, NOW(), NOW()) ON CONFLICT (id) DO NOTHING")
        .bind(test_org_id)
        .bind("cachetestorg")
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

    // --- USER-ONLY ACTIVITY ---
    let req = test::TestRequest::post()
        .uri("/api/activities")
        .set_json(&serde_json::json!({
            "user_id": test_user_id,
            "activity_type": test_activity_type,
            "details": "User-only activity"
        }))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;
    println!("POST /api/activities (user-only) status: {}", status);
    println!("POST /api/activities (user-only) raw body: {:?}", body);
    assert_eq!(status, 201);
    let resp_json: serde_json::Value =
        serde_json::from_slice(&body).expect("POST response is not valid JSON");
    let activity_id_user = resp_json["id"].as_str().unwrap();

    // --- USER+REPO ACTIVITY ---
    let req = test::TestRequest::post()
        .uri("/api/activities")
        .set_json(&serde_json::json!({
            "user_id": test_user_id,
            "repo_id": test_repo_id,
            "activity_type": test_activity_type,
            "details": "User+repo activity"
        }))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;
    println!("POST /api/activities (user+repo) status: {}", status);
    println!("POST /api/activities (user+repo) raw body: {:?}", body);
    assert_eq!(status, 201);
    let resp_json: serde_json::Value =
        serde_json::from_slice(&body).expect("POST response is not valid JSON");
    let activity_id_repo = resp_json["id"].as_str().unwrap();

    // --- USER+ORG ACTIVITY ---
    let req = test::TestRequest::post()
        .uri("/api/activities")
        .set_json(&serde_json::json!({
            "user_id": test_user_id,
            "org_id": test_org_id,
            "activity_type": test_activity_type,
            "details": "User+org activity"
        }))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;
    println!("POST /api/activities (user+org) status: {}", status);
    println!("POST /api/activities (user+org) raw body: {:?}", body);
    assert_eq!(status, 201);
    let resp_json: serde_json::Value =
        serde_json::from_slice(&body).expect("POST response is not valid JSON");
    let activity_id_org = resp_json["id"].as_str().unwrap();

    // GET and DELETE tests for each variant...
    for (label, activity_id, expected_details) in [
        ("user-only", activity_id_user, "User-only activity"),
        ("user+repo", activity_id_repo, "User+repo activity"),
        ("user+org", activity_id_org, "User+org activity"),
    ] {
        // GET (DB)
        let req = test::TestRequest::get()
            .uri(&format!("/api/activities/{}", activity_id))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        let status = resp.status();
        let body = test::read_body(resp).await;
        println!(
            "GET /api/activities/{} ({} DB) status: {}",
            activity_id, label, status
        );
        assert_eq!(status, 200);
        let resp1: serde_json::Value =
            serde_json::from_slice(&body).expect("GET response is not valid JSON");
        assert_eq!(resp1["details"], expected_details);
        // GET (cache)
        let req = test::TestRequest::get()
            .uri(&format!("/api/activities/{}", activity_id))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        let status = resp.status();
        let body = test::read_body(resp).await;
        println!(
            "GET /api/activities/{} ({} cache) status: {}",
            activity_id, label, status
        );
        assert_eq!(status, 200);
        let resp2: serde_json::Value =
            serde_json::from_slice(&body).expect("GET (cache) response is not valid JSON");
        assert_eq!(resp2["details"], expected_details);
        // DELETE
        let req = test::TestRequest::delete()
            .uri(&format!("/api/activities/{}", activity_id))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        let status = resp.status();
        println!(
            "DELETE /api/activities/{} ({}) status: {}",
            activity_id, label, status
        );
        assert_eq!(status, 204);
        // GET after delete
        let req = test::TestRequest::get()
            .uri(&format!("/api/activities/{}", activity_id))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        let status = resp.status();
        println!(
            "GET (after delete) /api/activities/{} ({}) status: {}",
            activity_id, label, status
        );
        assert_eq!(status, 404);
    }
    // Similar tests: user_cache_integration.rs, repository_cache_integration.rs, organization_cache_integration.rs
}
