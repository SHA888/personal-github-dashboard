use crate::models::activity::Activity;
use crate::models::organization::Organization;
use crate::models::repository::Repository;
use crate::models::user::User;
use crate::utils::cache::{
    TTL_ACTIVITY, TTL_REPO, activity_cache_key, org_cache_key, repo_cache_key, ttl_user,
    user_cache_key,
};
use crate::utils::redis::RedisClient;
use chrono::{DateTime, Utc};
use log::{debug, warn};
use metrics::{histogram, increment_counter};
use serde_json::Value;
use sqlx::Error;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;
use uuid::Uuid;

/// Creates a PostgreSQL connection pool with a maximum number of connections and a 5-second acquire timeout.
/// # Examples
/// ```
/// let pool = create_pg_pool("postgres://user:pass@localhost/db", 10).await;
/// assert!(pool.is_closed() == false);
/// ```
pub async fn create_pg_pool(database_url: &str, max_connections: u32) -> PgPool {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await
        .expect("Failed to create Postgres connection pool")
}

/// Creates a PostgreSQL connection pool optimized for memory efficiency.
/// The pool maintains a minimum of one idle connection, limits the maximum number of connections,
/// sets a maximum connection lifetime of 10 minutes, and applies a 60-second idle timeout. Each new
/// connection is configured with a 5-second statement timeout.
/// # Examples
/// ```
/// let pool = create_pg_pool_memory_efficient("postgres://user:pass@localhost/db", 10).await;
/// assert!(pool.acquire().await.is_ok());
/// ```
pub async fn create_pg_pool_memory_efficient(database_url: &str, max_connections: u32) -> PgPool {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .min_connections(1) // keep only 1 idle connection
        .max_lifetime(Some(Duration::from_secs(600))) // close idle after 10m
        .idle_timeout(Some(Duration::from_secs(60))) // idle conn timeout 60s
        .after_connect(|conn, _meta| {
            Box::pin(async move {
                sqlx::query("SET statement_timeout = 5000;")
                    .execute(conn)
                    .await
                    .ok();
                Ok(())
            })
        })
        .connect(database_url)
        .await
        .expect("Failed to create PgPool")
}

/// Retrieves a user by their unique identifier from the database.
/// Returns `Ok(Some(User))` if a user with the given ID exists, `Ok(None)` if not found, or an error if the query fails.
/// # Examples
/// ```
/// let user = get_user_by_id(&pool, &user_id).await?;
/// if let Some(user) = user {
///     println!("Found user: {}", user.username);
/// }
/// ```
pub async fn get_user_by_id(pool: &PgPool, user_id: &Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(pool)
        .await
}

pub async fn get_user_by_id_with_cache(
    pool: &PgPool,
    redis: &RedisClient,
    user_id: &Uuid,
) -> Result<Option<User>, sqlx::Error> {
    let cache_key = user_cache_key(user_id);
    let timer = std::time::Instant::now();
    debug!(
        "[CACHE] Attempting to GET user from Redis with key: {}",
        cache_key
    );
    let cache_result: redis::RedisResult<Option<String>> = redis.get::<String>(&cache_key).await;
    match &cache_result {
        Ok(Some(_)) => debug!("[CACHE] HIT for user_id: {}", user_id),
        Ok(None) => debug!("[CACHE] MISS for user_id: {}", user_id),
        Err(e) => warn!("[CACHE] ERROR on GET for user_id: {}: {}", user_id, e),
    }
    if let Ok(Some(cached)) = cache_result {
        match serde_json::from_str::<User>(&cached) {
            Ok(user) => {
                debug!("[CACHE] Successfully deserialized cached user: {}", user_id);
                return Ok(Some(user));
            }
            Err(e) => {
                warn!(
                    "[CACHE] Deserialization failed for user_id: {}: {}",
                    user_id, e
                );
            }
        }
    }
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(pool)
        .await?;
    histogram!("db_user_query_duration", timer.elapsed().as_secs_f64());
    if let Some(ref user) = user {
        if let Ok(json) = serde_json::to_string(user) {
            debug!("[CACHE] Setting user in Redis with key: {}", cache_key);
            let set_result = redis.set(&cache_key, json, ttl_user()).await;
            if let Err(e) = set_result {
                warn!("[CACHE] ERROR on SET for user_id: {}: {}", user_id, e);
            } else {
                debug!("[CACHE] SET successful for user_id: {}", user_id);
            }
        }
    }
    Ok(user)
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

pub async fn create_user_with_cache(
    pool: &PgPool,
    redis: &RedisClient,
    username: &str,
    email: &str,
    avatar_url: Option<&str>,
) -> Result<User, Error> {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email, avatar_url, created_at) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(username)
    .bind(email)
    .bind(avatar_url)
    .bind(Utc::now())
    .fetch_one(pool)
    .await?;
    cache_user(redis, &user).await;
    Ok(user)
}

pub async fn cache_user(redis: &RedisClient, user: &User) {
    let cache_key = user_cache_key(&user.id);
    if let Ok(json) = serde_json::to_string(user) {
        let _ = redis
            .set(&cache_key, json, crate::utils::cache::ttl_user())
            .await;
    }
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

pub async fn update_user_avatar_with_cache(
    pool: &PgPool,
    redis: &RedisClient,
    user_id: &Uuid,
    avatar_url: Option<&str>,
) -> Result<User, Error> {
    let user =
        sqlx::query_as::<_, User>("UPDATE users SET avatar_url = $1 WHERE id = $2 RETURNING *")
            .bind(avatar_url)
            .bind(user_id)
            .fetch_one(pool)
            .await?;
    cache_user(redis, &user).await;
    Ok(user)
}

pub async fn delete_user(pool: &PgPool, user_id: &Uuid) -> Result<u64, Error> {
    let res = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn delete_user_with_cache(
    pool: &PgPool,
    redis: &RedisClient,
    user_id: &Uuid,
) -> Result<u64, Error> {
    let res = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id)
        .execute(pool)
        .await?;
    invalidate_user_cache(redis, user_id).await;
    Ok(res.rows_affected())
}

pub async fn invalidate_user_cache(redis: &RedisClient, user_id: &Uuid) {
    let cache_key = user_cache_key(user_id);
    if let Ok(mut conn) = redis.client.get_async_connection().await {
        let _: Result<(), _> = redis::cmd("DEL")
            .arg(&cache_key)
            .query_async(&mut conn)
            .await;
    }
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

pub async fn get_organization_by_id_with_cache(
    pool: &PgPool,
    redis: &RedisClient,
    org_id: &Uuid,
) -> Result<Option<Organization>, sqlx::Error> {
    let cache_key = org_cache_key(org_id);
    let timer = std::time::Instant::now();
    debug!(
        "[CACHE] Attempting to GET org from Redis with key: {}",
        cache_key
    );
    let cache_result: redis::RedisResult<Option<String>> = redis.get::<String>(&cache_key).await;
    match &cache_result {
        Ok(Some(_)) => debug!("[CACHE] HIT for org_id: {}", org_id),
        Ok(None) => debug!("[CACHE] MISS for org_id: {}", org_id),
        Err(e) => warn!("[CACHE] ERROR on GET for org_id: {}: {}", org_id, e),
    }
    if let Ok(Some(cached)) = cache_result {
        match serde_json::from_str::<Organization>(&cached) {
            Ok(org) => {
                debug!("[CACHE] Successfully deserialized cached org: {}", org_id);
                return Ok(Some(org));
            }
            Err(e) => {
                warn!(
                    "[CACHE] Deserialization failed for org_id: {}: {}",
                    org_id, e
                );
            }
        }
    }
    let org = sqlx::query_as::<_, Organization>("SELECT * FROM organizations WHERE id = $1")
        .bind(org_id)
        .fetch_optional(pool)
        .await?;
    histogram!("db_org_query_duration", timer.elapsed().as_secs_f64());
    if let Some(ref org) = org {
        if let Ok(json) = serde_json::to_string(org) {
            debug!("[CACHE] Setting org in Redis with key: {}", cache_key);
            let set_result = redis.set(&cache_key, json, TTL_REPO).await;
            if let Err(e) = set_result {
                warn!("[CACHE] ERROR on SET for org_id: {}: {}", org_id, e);
            } else {
                debug!("[CACHE] SET successful for org_id: {}", org_id);
            }
        }
    }
    Ok(org)
}

pub async fn cache_organization(redis: &RedisClient, org: &Organization) {
    let cache_key = org_cache_key(&org.id);
    if let Ok(json) = serde_json::to_string(org) {
        let _ = redis.set(&cache_key, json, TTL_REPO).await;
    }
}

pub async fn invalidate_organization_cache(redis: &RedisClient, org_id: &Uuid) {
    let cache_key = org_cache_key(org_id);
    if let Ok(mut conn) = redis.client.get_async_connection().await {
        let _: Result<(), _> = redis::cmd("DEL")
            .arg(&cache_key)
            .query_async(&mut conn)
            .await;
    }
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

pub async fn create_organization_with_cache(
    pool: &PgPool,
    redis: &RedisClient,
    name: &str,
    description: Option<&str>,
) -> Result<Organization, Error> {
    let org = sqlx::query_as::<_, Organization>(
        "INSERT INTO organizations (name, description, created_at) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(name)
    .bind(description)
    .bind(Utc::now())
    .fetch_one(pool)
    .await?;
    cache_organization(redis, &org).await;
    Ok(org)
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

pub async fn update_organization_description_with_cache(
    pool: &PgPool,
    redis: &RedisClient,
    org_id: &Uuid,
    description: Option<&str>,
) -> Result<Organization, Error> {
    let org = sqlx::query_as::<_, Organization>(
        "UPDATE organizations SET description = $1 WHERE id = $2 RETURNING *",
    )
    .bind(description)
    .bind(org_id)
    .fetch_one(pool)
    .await?;
    cache_organization(redis, &org).await;
    Ok(org)
}

pub async fn delete_organization(pool: &PgPool, org_id: &Uuid) -> Result<u64, Error> {
    let res = sqlx::query("DELETE FROM organizations WHERE id = $1")
        .bind(org_id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn delete_organization_with_cache(
    pool: &PgPool,
    redis: &RedisClient,
    org_id: &Uuid,
) -> Result<u64, Error> {
    let res = sqlx::query("DELETE FROM organizations WHERE id = $1")
        .bind(org_id)
        .execute(pool)
        .await?;
    invalidate_organization_cache(redis, org_id).await;
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

pub async fn get_repository_by_id_with_cache(
    pool: &PgPool,
    redis: &RedisClient,
    repo_id: &Uuid,
) -> Result<Option<Repository>, sqlx::Error> {
    let cache_key = repo_cache_key(repo_id);
    let timer = std::time::Instant::now();
    debug!(
        "[CACHE] Attempting to GET repo from Redis with key: {}",
        cache_key
    );
    let cache_result: redis::RedisResult<Option<String>> = redis.get::<String>(&cache_key).await;
    match &cache_result {
        Ok(Some(_)) => debug!("[CACHE] HIT for repo_id: {}", repo_id),
        Ok(None) => debug!("[CACHE] MISS for repo_id: {}", repo_id),
        Err(e) => warn!("[CACHE] ERROR on GET for repo_id: {}: {}", repo_id, e),
    }
    if let Ok(Some(cached)) = cache_result {
        match serde_json::from_str::<Repository>(&cached) {
            Ok(repo) => {
                debug!("[CACHE] Successfully deserialized cached repo: {}", repo_id);
                return Ok(Some(repo));
            }
            Err(e) => {
                warn!(
                    "[CACHE] Deserialization failed for repo_id: {}: {}",
                    repo_id, e
                );
            }
        }
    }
    let repo = sqlx::query_as::<_, Repository>("SELECT * FROM repositories WHERE id = $1")
        .bind(repo_id)
        .fetch_optional(pool)
        .await?;
    histogram!("db_repo_query_duration", timer.elapsed().as_secs_f64());
    if let Some(ref repo) = repo {
        if let Ok(json) = serde_json::to_string(repo) {
            debug!("[CACHE] Setting repo in Redis with key: {}", cache_key);
            let set_result = redis.set(&cache_key, json, TTL_REPO).await;
            if let Err(e) = set_result {
                warn!("[CACHE] ERROR on SET for repo_id: {}: {}", repo_id, e);
            } else {
                debug!("[CACHE] SET successful for repo_id: {}", repo_id);
            }
        }
    }
    Ok(repo)
}

pub async fn cache_repository(redis: &RedisClient, repo: &Repository) {
    let cache_key = repo_cache_key(&repo.id);
    if let Ok(json) = serde_json::to_string(repo) {
        let _ = redis.set(&cache_key, json, TTL_REPO).await;
    }
}

pub async fn invalidate_repository_cache(redis: &RedisClient, repo_id: &Uuid) {
    let cache_key = repo_cache_key(repo_id);
    if let Ok(mut conn) = redis.client.get_async_connection().await {
        let _: Result<(), _> = redis::cmd("DEL")
            .arg(&cache_key)
            .query_async(&mut conn)
            .await;
    }
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

pub async fn create_repository_with_cache(
    pool: &PgPool,
    redis: &RedisClient,
    org_id: Option<&Uuid>,
    owner_id: &Uuid,
    name: &str,
    description: Option<&str>,
    is_private: bool,
) -> Result<Repository, Error> {
    let repo = sqlx::query_as::<_, Repository>(
        "INSERT INTO repositories (org_id, owner_id, name, description, is_private, created_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(org_id)
    .bind(owner_id)
    .bind(name)
    .bind(description)
    .bind(is_private)
    .bind(Utc::now())
    .fetch_one(pool)
    .await?;
    cache_repository(redis, &repo).await;
    Ok(repo)
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

pub async fn update_repository_description_with_cache(
    pool: &PgPool,
    redis: &RedisClient,
    repo_id: &Uuid,
    description: Option<&str>,
) -> Result<Repository, Error> {
    let repo = sqlx::query_as::<_, Repository>(
        "UPDATE repositories SET description = $1 WHERE id = $2 RETURNING *",
    )
    .bind(description)
    .bind(repo_id)
    .fetch_one(pool)
    .await?;
    cache_repository(redis, &repo).await;
    Ok(repo)
}

pub async fn delete_repository(pool: &PgPool, repo_id: &Uuid) -> Result<u64, Error> {
    let res = sqlx::query("DELETE FROM repositories WHERE id = $1")
        .bind(repo_id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn delete_repository_with_cache(
    pool: &PgPool,
    redis: &RedisClient,
    repo_id: &Uuid,
) -> Result<u64, Error> {
    let res = sqlx::query("DELETE FROM repositories WHERE id = $1")
        .bind(repo_id)
        .execute(pool)
        .await?;
    invalidate_repository_cache(redis, repo_id).await;
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

pub async fn get_activity_by_id_with_cache(
    pool: &PgPool,
    redis: &RedisClient,
    activity_id: &Uuid,
) -> Result<Option<Activity>, sqlx::Error> {
    let cache_key = activity_cache_key(activity_id);
    let timer = std::time::Instant::now();
    let cache_result: redis::RedisResult<Option<String>> = redis.get::<String>(&cache_key).await;
    match &cache_result {
        Ok(Some(_)) => increment_counter!("cache_activity_hit"),
        Ok(None) => increment_counter!("cache_activity_miss"),
        Err(_) => increment_counter!("cache_activity_error"),
    }
    if let Ok(Some(cached)) = cache_result {
        if let Ok(activity) = serde_json::from_str::<Activity>(&cached) {
            return Ok(Some(activity));
        } else {
            warn!(
                "Cache deserialization failed for activity_id: {}",
                activity_id
            );
        }
    }
    let activity = sqlx::query_as::<_, Activity>("SELECT * FROM activities WHERE id = $1")
        .bind(activity_id)
        .fetch_optional(pool)
        .await?;
    histogram!("db_activity_query_duration", timer.elapsed().as_secs_f64());
    if let Some(ref activity) = activity {
        if let Ok(json) = serde_json::to_string(activity) {
            let _ = redis.set(&cache_key, json, TTL_ACTIVITY).await;
        }
    }
    Ok(activity)
}

pub async fn cache_activity(redis: &RedisClient, activity: &Activity) {
    let cache_key = activity_cache_key(&activity.id);
    if let Ok(json) = serde_json::to_string(activity) {
        let _ = redis.set(&cache_key, json, TTL_ACTIVITY).await;
    }
}

pub async fn invalidate_activity_cache(redis: &RedisClient, activity_id: &Uuid) {
    let cache_key = activity_cache_key(activity_id);
    if let Ok(mut conn) = redis.client.get_async_connection().await {
        let _: Result<(), _> = redis::cmd("DEL")
            .arg(&cache_key)
            .query_async(&mut conn)
            .await;
    }
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

pub async fn create_activity_with_cache(
    pool: &PgPool,
    redis: &RedisClient,
    user_id: &Uuid,
    repo_id: Option<&Uuid>,
    org_id: Option<&Uuid>,
    activity_type: &str,
    details: Option<&str>,
) -> Result<Activity, Error> {
    let activity = sqlx::query_as::<_, Activity>(
        "INSERT INTO activities (user_id, repo_id, org_id, type, details, created_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(user_id)
    .bind(repo_id)
    .bind(org_id)
    .bind(activity_type)
    .bind(details)
    .bind(Utc::now())
    .fetch_one(pool)
    .await?;
    cache_activity(redis, &activity).await;
    Ok(activity)
}

pub async fn delete_activity(pool: &PgPool, activity_id: &Uuid) -> Result<u64, Error> {
    let res = sqlx::query("DELETE FROM activities WHERE id = $1")
        .bind(activity_id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn delete_activity_with_cache(
    pool: &PgPool,
    redis: &RedisClient,
    activity_id: &Uuid,
) -> Result<u64, Error> {
    let res = sqlx::query("DELETE FROM activities WHERE id = $1")
        .bind(activity_id)
        .execute(pool)
        .await?;
    invalidate_activity_cache(redis, activity_id).await;
    Ok(res.rows_affected())
}
