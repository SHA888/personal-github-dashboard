use crate::db::DbPool;
use crate::error::AppError;
use crate::models::{Notification, NotificationFrequency, NotificationSettings, NotificationType};
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

// --- Notifications ---

pub async fn create_notification(
    pool: &PgPool,
    user_id: Uuid,
    type_: &str,
    title: &str,
    message: Option<&str>,
) -> Result<Notification, sqlx::Error> {
    sqlx::query_as!(
        Notification,
        r#"
        INSERT INTO notifications (user_id, type, title, message)
        VALUES ($1, $2, $3, $4)
        RETURNING id, user_id, type as "type_: String", title, message, read, created_at, updated_at
        "#,
        user_id,
        type_,
        title,
        message,
    )
    .fetch_one(pool)
    .await
}

pub async fn get_notifications_by_user(
    pool: &PgPool,
    user_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<Notification>, sqlx::Error> {
    sqlx::query_as!(
        Notification,
        r#"
        SELECT id, user_id, type as "type_: String", title, message, read, created_at, updated_at
        FROM notifications
        WHERE user_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
        user_id,
        limit,
        offset
    )
    .fetch_all(pool)
    .await
}

pub async fn mark_notification_read(
    pool: &PgPool,
    notification_id: Uuid,
    user_id: Uuid,
) -> Result<(), sqlx::Error> {
    let result = sqlx::query!(
        r#"
        UPDATE notifications
        SET read = true, updated_at = NOW()
        WHERE id = $1 AND user_id = $2
        "#,
        notification_id,
        user_id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(())
}

// --- Notification Settings ---

pub async fn get_notification_settings(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<NotificationSettings>, sqlx::Error> {
    sqlx::query_as!(
        NotificationSettings,
        r#"
        SELECT id, user_id, type as "type_: String", enabled, frequency, created_at, updated_at
        FROM notification_settings
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
}

pub async fn update_notification_setting(
    pool: &PgPool,
    user_id: Uuid,
    type_: &str,
    enabled: bool,
    frequency: &str,
) -> Result<NotificationSettings, sqlx::Error> {
    sqlx::query_as!(
        NotificationSettings,
        r#"
        INSERT INTO notification_settings (user_id, type, enabled, frequency)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (user_id, type)
        DO UPDATE SET
            enabled = EXCLUDED.enabled,
            frequency = EXCLUDED.frequency,
            updated_at = NOW()
        RETURNING id, user_id, type as "type_: String", enabled, frequency, created_at, updated_at
        "#,
        user_id,
        type_,
        enabled,
        frequency
    )
    .fetch_one(pool)
    .await
}

pub async fn get_notification_by_id(
    pool: &PgPool,
    user_id: Uuid,
    notification_id: Uuid,
) -> Result<Notification, sqlx::Error> {
    sqlx::query_as!(
        Notification,
        r#"
        SELECT id, user_id, type as "type_: String", title, message, read, created_at, updated_at
        FROM notifications
        WHERE id = $1 AND user_id = $2
        "#,
        notification_id,
        user_id
    )
    .fetch_one(pool)
    .await
}
