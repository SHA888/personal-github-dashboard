use crate::db::{
    create_activity_with_cache, delete_activity_with_cache, get_activity_by_id_with_cache,
};
use crate::utils::redis::RedisClient;
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateActivityRequest {
    pub user_id: Uuid,
    pub repo_id: Option<Uuid>,
    pub org_id: Option<Uuid>,
    pub activity_type: String,
    pub details: Option<String>,
}

pub async fn get_activity_by_id(
    pool: web::Data<PgPool>,
    redis: web::Data<RedisClient>,
    activity_id: web::Path<Uuid>,
) -> HttpResponse {
    match get_activity_by_id_with_cache(&pool, &redis, &activity_id).await {
        Ok(Some(activity)) => HttpResponse::Ok().json(activity),
        Ok(None) => HttpResponse::NotFound().body("Activity not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB/Cache error: {}", e)),
    }
}

pub async fn create_activity(
    pool: web::Data<PgPool>,
    redis: web::Data<RedisClient>,
    req: web::Json<CreateActivityRequest>,
) -> HttpResponse {
    match create_activity_with_cache(
        &pool,
        &redis,
        &req.user_id,
        req.repo_id.as_ref(),
        req.org_id.as_ref(),
        &req.activity_type,
        req.details.as_deref(),
    )
    .await
    {
        Ok(activity) => HttpResponse::Created().json(activity),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB/Cache error: {}", e)),
    }
}

pub async fn delete_activity(
    pool: web::Data<PgPool>,
    redis: web::Data<RedisClient>,
    activity_id: web::Path<Uuid>,
) -> HttpResponse {
    match delete_activity_with_cache(&pool, &redis, &activity_id).await {
        Ok(affected) if affected > 0 => HttpResponse::NoContent().finish(),
        Ok(_) => HttpResponse::NotFound().body("Activity not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB/Cache error: {}", e)),
    }
}
