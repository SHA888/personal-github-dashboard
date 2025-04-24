use crate::models::activity::Activity;
use crate::models::organization::Organization;
use crate::models::repository::Repository;
use crate::models::user::User;
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::Error;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use uuid::Uuid;

/// Create a PostgreSQL connection pool with custom configuration.
pub async fn create_pg_pool(database_url: &str, max_connections: u32) -> PgPool {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await
        .expect("Failed to create Postgres connection pool")
}

// --- User CRUD ---
pub async fn get_user_by_id(pool: &PgPool, user_id: &Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(pool)
        .await
}

pub async fn create_user(
    pool: &PgPool,
    username: &str,
    email: &str,
    avatar_url: Option<&str>,
) -> Result<User, Error> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email, avatar_url, created_at) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(username)
    .bind(email)
    .bind(avatar_url)
    .bind(Utc::now())
    .fetch_one(pool)
    .await
}

pub async fn update_user_avatar(
    pool: &PgPool,
    user_id: &Uuid,
    avatar_url: Option<&str>,
) -> Result<User, Error> {
    sqlx::query_as::<_, User>("UPDATE users SET avatar_url = $1 WHERE id = $2 RETURNING *")
        .bind(avatar_url)
        .bind(user_id)
        .fetch_one(pool)
        .await
}

pub async fn delete_user(pool: &PgPool, user_id: &Uuid) -> Result<u64, Error> {
    let res = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

// --- Organization CRUD ---
pub async fn get_organization_by_id(
    pool: &PgPool,
    org_id: &Uuid,
) -> Result<Option<Organization>, sqlx::Error> {
    sqlx::query_as::<_, Organization>("SELECT * FROM organizations WHERE id = $1")
        .bind(org_id)
        .fetch_optional(pool)
        .await
}

pub async fn create_organization(
    pool: &PgPool,
    name: &str,
    description: Option<&str>,
) -> Result<Organization, Error> {
    sqlx::query_as::<_, Organization>(
        "INSERT INTO organizations (name, description, created_at) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(name)
    .bind(description)
    .bind(Utc::now())
    .fetch_one(pool)
    .await
}

pub async fn update_organization_description(
    pool: &PgPool,
    org_id: &Uuid,
    description: Option<&str>,
) -> Result<Organization, Error> {
    sqlx::query_as::<_, Organization>(
        "UPDATE organizations SET description = $1 WHERE id = $2 RETURNING *",
    )
    .bind(description)
    .bind(org_id)
    .fetch_one(pool)
    .await
}

pub async fn delete_organization(pool: &PgPool, org_id: &Uuid) -> Result<u64, Error> {
    let res = sqlx::query("DELETE FROM organizations WHERE id = $1")
        .bind(org_id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

// --- Repository CRUD ---
pub async fn get_repository_by_id(
    pool: &PgPool,
    repo_id: &Uuid,
) -> Result<Option<Repository>, sqlx::Error> {
    sqlx::query_as::<_, Repository>("SELECT * FROM repositories WHERE id = $1")
        .bind(repo_id)
        .fetch_optional(pool)
        .await
}

pub async fn create_repository(
    pool: &PgPool,
    org_id: Option<&Uuid>,
    owner_id: &Uuid,
    name: &str,
    description: Option<&str>,
    is_private: bool,
) -> Result<Repository, Error> {
    sqlx::query_as::<_, Repository>(
        "INSERT INTO repositories (org_id, owner_id, name, description, is_private, created_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(org_id)
    .bind(owner_id)
    .bind(name)
    .bind(description)
    .bind(is_private)
    .bind(Utc::now())
    .fetch_one(pool)
    .await
}

pub async fn update_repository_description(
    pool: &PgPool,
    repo_id: &Uuid,
    description: Option<&str>,
) -> Result<Repository, Error> {
    sqlx::query_as::<_, Repository>(
        "UPDATE repositories SET description = $1 WHERE id = $2 RETURNING *",
    )
    .bind(description)
    .bind(repo_id)
    .fetch_one(pool)
    .await
}

pub async fn delete_repository(pool: &PgPool, repo_id: &Uuid) -> Result<u64, Error> {
    let res = sqlx::query("DELETE FROM repositories WHERE id = $1")
        .bind(repo_id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn get_repositories_by_user_id(
    pool: &PgPool,
    user_id: &Uuid,
) -> Result<Vec<Repository>, Error> {
    sqlx::query_as::<_, Repository>("SELECT * FROM repositories WHERE owner_id = $1")
        .bind(user_id)
        .fetch_all(pool)
        .await
}

pub async fn get_repositories_by_org_id(
    pool: &PgPool,
    org_id: &Uuid,
) -> Result<Vec<Repository>, Error> {
    sqlx::query_as::<_, Repository>("SELECT * FROM repositories WHERE org_id = $1")
        .bind(org_id)
        .fetch_all(pool)
        .await
}

// --- Activity CRUD ---
pub async fn get_activity_by_id(
    pool: &PgPool,
    activity_id: &Uuid,
) -> Result<Option<Activity>, Error> {
    sqlx::query_as::<_, Activity>("SELECT * FROM activities WHERE id = $1")
        .bind(activity_id)
        .fetch_optional(pool)
        .await
}

pub async fn create_activity(
    pool: &PgPool,
    user_id: &Uuid,
    repo_id: Option<&Uuid>,
    r#type: &str,
    timestamp: DateTime<Utc>,
    data: Value,
) -> Result<Activity, Error> {
    sqlx::query_as::<_, Activity>(
        "INSERT INTO activities (user_id, repo_id, type, timestamp, data, created_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(user_id)
    .bind(repo_id)
    .bind(r#type)
    .bind(timestamp)
    .bind(data)
    .bind(Utc::now())
    .fetch_one(pool)
    .await
}

pub async fn delete_activity(pool: &PgPool, activity_id: &Uuid) -> Result<u64, Error> {
    let res = sqlx::query("DELETE FROM activities WHERE id = $1")
        .bind(activity_id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
