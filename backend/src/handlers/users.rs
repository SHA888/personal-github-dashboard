// TODO: Implement users handler module for user management endpoints.

use crate::db::{
    create_user_with_cache, delete_user_with_cache, get_user_by_id_with_cache,
    update_user_avatar_with_cache,
};
use crate::utils::redis::RedisClient;
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub avatar_url: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateAvatarRequest {
    pub avatar_url: Option<String>,
}

pub async fn create_user(
    pool: web::Data<PgPool>,
    redis: web::Data<RedisClient>,
    req: web::Json<CreateUserRequest>,
) -> HttpResponse {
    match create_user_with_cache(
        &pool,
        &redis,
        &req.username,
        &req.email,
        req.avatar_url.as_deref(),
    )
    .await
    {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB/Cache error: {}", e)),
    }
}

// Handler to fetch user by ID (for GET /user/{id})
pub async fn get_user_by_id_handler(
    pool: web::Data<PgPool>,
    redis: web::Data<RedisClient>,
    user_id: web::Path<Uuid>,
) -> impl Responder {
    match get_user_by_id_with_cache(&pool, &redis, &user_id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB/Cache error: {}", e)),
    }
}

pub async fn update_user_avatar(
    pool: web::Data<PgPool>,
    redis: web::Data<RedisClient>,
    user_id: web::Path<Uuid>,
    req: web::Json<UpdateAvatarRequest>,
) -> HttpResponse {
    match update_user_avatar_with_cache(&pool, &redis, &user_id, req.avatar_url.as_deref()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB/Cache error: {}", e)),
    }
}

pub async fn delete_user(
    pool: web::Data<PgPool>,
    redis: web::Data<RedisClient>,
    user_id: web::Path<Uuid>,
) -> HttpResponse {
    match delete_user_with_cache(&pool, &redis, &user_id).await {
        Ok(affected) if affected > 0 => HttpResponse::NoContent().finish(),
        Ok(_) => HttpResponse::NotFound().body("User not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB/Cache error: {}", e)),
    }
}
