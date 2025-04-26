use actix_web::{web, HttpResponse};
use crate::utils::redis::RedisClient;
use crate::error::AppError;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_activity_by_id_handler(
    pool: web::Data<PgPool>,
    redis: web::Data<RedisClient>,
    activity_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let cache_key = format!("activity:{}", activity_id);
    let activity = redis
        .cache_get_or_set(&cache_key, 300, || {
            let pool = pool.clone();
            let activity_id = *activity_id;
            async move {
                crate::db::get_activity_by_id(&pool, &activity_id).await.ok().flatten()
            }
        })
        .await
        .map_err(|e| AppError::InternalError(format!("Redis error: {}", e)))?;
    if let Some(a) = activity {
        Ok(HttpResponse::Ok().json(a))
    } else {
        Err(AppError::NotFound("Activity not found".into()))
    }
}
