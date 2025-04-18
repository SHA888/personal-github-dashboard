use crate::{db::DbPool, error::AppError, models::Notification};
use actix_web::{get, post, put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::notifications::{
    create_notification, get_notification_by_id, get_notification_settings,
    get_notifications_by_user, mark_notification_read, update_notification_setting,
};
use crate::models::notification::{
    CreateNotificationRequest, NotificationFrequency, NotificationSettings, NotificationType,
    UpdateNotificationSettingRequest,
};
use time::OffsetDateTime;

#[derive(Deserialize)]
pub struct QueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[get("/notifications")]
pub async fn list_notifications(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<Uuid>,
    query: web::Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    let limit = query.limit.unwrap_or(10);
    let offset = query.offset.unwrap_or(0);
    let notifications = get_notifications_by_user(&pool, *user_id, limit, offset).await?;
    Ok(HttpResponse::Ok().json(notifications))
}

#[get("/notifications/{id}")]
pub async fn get_notification(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<Uuid>,
    notification_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let notification = get_notification_by_id(&pool, *user_id, *notification_id).await?;
    Ok(HttpResponse::Ok().json(notification))
}

#[post("/notifications")]
pub async fn create_new_notification(
    pool: web::Data<PgPool>,
    user_id: web::ReqData<Uuid>,
    req: web::Json<CreateNotificationRequest>,
) -> Result<HttpResponse, AppError> {
    let notification = create_notification(
        &pool,
        *user_id,
        req.type_.as_str(),
        &req.title,
        req.message.as_deref(),
    )
    .await?;
    Ok(HttpResponse::Created().json(notification))
}

#[put("/notifications/{notification_id}/read")]
pub async fn mark_read(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<Uuid>,
    notification_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    mark_notification_read(&pool, *user_id, *notification_id).await?;
    Ok(HttpResponse::Ok().finish())
}

#[get("/notifications/settings")]
pub async fn get_settings(
    pool: web::Data<PgPool>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, AppError> {
    let settings = get_notification_settings(&pool, *user_id).await?;
    Ok(HttpResponse::Ok().json(settings))
}

#[put("/notifications/settings")]
pub async fn update_settings(
    pool: web::Data<PgPool>,
    user_id: web::ReqData<Uuid>,
    req: web::Json<UpdateNotificationSettingRequest>,
) -> Result<HttpResponse, AppError> {
    let settings = update_notification_setting(
        &pool,
        *user_id,
        req.type_.as_str(),
        req.enabled,
        req.frequency.as_str(),
    )
    .await?;
    Ok(HttpResponse::Ok().json(settings))
}
