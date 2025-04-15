use crate::db::DbPool;
use crate::error::AppError;
use crate::models::{Notification, NotificationFrequency, NotificationSettings, NotificationType};
use uuid::Uuid;

// --- Notifications ---

pub async fn create_notification(
    pool: &DbPool,
    user_id: Uuid,
    type_: NotificationType,
    title: &str,
    message: Option<&str>,
) -> Result<Notification, AppError> {
    let notification = sqlx::query_as!(
        Notification,
        r#"
        INSERT INTO notifications (user_id, type, title, message)
        VALUES ($1, $2, $3, $4)
        RETURNING id, user_id, type as "type_", title, message, read, created_at, updated_at
        "#,
        user_id,
        type_.as_str(),
        title,
        message
    )
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(notification)
}

pub async fn get_notifications_by_user(
    pool: &DbPool,
    user_id: Uuid,
    read: Option<bool>,
    limit: i64,
    offset: i64,
) -> Result<Vec<Notification>, AppError> {
    let mut query_builder = sqlx::QueryBuilder::new(
        r#"SELECT id, user_id, type as "type_", title, message, read, created_at, updated_at
           FROM notifications WHERE user_id = "#,
    );
    query_builder.push_bind(user_id);

    if let Some(read_status) = read {
        query_builder.push(" AND read = ");
        query_builder.push_bind(read_status);
    }

    query_builder.push(" ORDER BY created_at DESC LIMIT ");
    query_builder.push_bind(limit);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset);

    let query = query_builder.build_query_as::<Notification>();

    let notifications = query
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(notifications)
}

pub async fn mark_notification_read(
    pool: &DbPool,
    notification_id: Uuid,
) -> Result<bool, AppError> {
    let rows_affected = sqlx::query!(
        r#"
        UPDATE notifications SET read = TRUE, updated_at = NOW()
        WHERE id = $1 AND read = FALSE
        "#,
        notification_id
    )
    .execute(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(rows_affected.rows_affected() > 0)
}

// --- Notification Settings ---

pub async fn get_notification_settings(
    pool: &DbPool,
    user_id: Uuid,
) -> Result<Vec<NotificationSettings>, AppError> {
    let settings = sqlx::query_as!(
        NotificationSettings,
        r#"
        SELECT id, user_id, type as "type_", enabled, frequency, created_at, updated_at
        FROM notification_settings
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(settings)
}

pub async fn update_notification_setting(
    pool: &DbPool,
    user_id: Uuid,
    type_: NotificationType,
    enabled: bool,
    frequency: NotificationFrequency,
) -> Result<NotificationSettings, AppError> {
    let setting = sqlx::query_as!(
        NotificationSettings,
        r#"
        INSERT INTO notification_settings (user_id, type, enabled, frequency)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (user_id, type) DO UPDATE SET
            enabled = $3,
            frequency = $4,
            updated_at = NOW()
        RETURNING id, user_id, type as "type_", enabled, frequency, created_at, updated_at
        "#,
        user_id,
        type_.as_str(),
        enabled,
        frequency.as_str()
    )
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(setting)
}
