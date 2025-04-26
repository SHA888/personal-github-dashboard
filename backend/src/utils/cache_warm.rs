use crate::db::{
    get_activity_by_id_with_cache, get_organization_by_id_with_cache,
    get_repository_by_id_with_cache, get_user_by_id_with_cache,
};
use crate::models::{
    activity::Activity, organization::Organization, repository::Repository, user::User,
};
use crate::utils::redis::RedisClient;
use log::{info, warn};
use sqlx::PgPool;
use uuid::Uuid;

/// Fetches up to N most recent entities of each type and warms the cache by calling their *_by_id_with_cache functions.
pub async fn warm_cache(pool: &PgPool, redis: &RedisClient) {
    const LIMIT: i64 = 10;

    // Users
    match sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC LIMIT $1")
        .bind(LIMIT)
        .fetch_all(pool)
        .await
    {
        Ok(users) => {
            for user in users {
                let _ = get_user_by_id_with_cache(pool, redis, &user.id).await;
            }
            info!("Warmed cache for {} users", LIMIT);
        }
        Err(e) => warn!("Failed to fetch users for cache warming: {}", e),
    }

    // Repositories
    match sqlx::query_as::<_, Repository>(
        "SELECT * FROM repositories ORDER BY created_at DESC LIMIT $1",
    )
    .bind(LIMIT)
    .fetch_all(pool)
    .await
    {
        Ok(repos) => {
            for repo in repos {
                let _ = get_repository_by_id_with_cache(pool, redis, &repo.id).await;
            }
            info!("Warmed cache for {} repositories", LIMIT);
        }
        Err(e) => warn!("Failed to fetch repositories for cache warming: {}", e),
    }

    // Organizations
    match sqlx::query_as::<_, Organization>(
        "SELECT * FROM organizations ORDER BY created_at DESC LIMIT $1",
    )
    .bind(LIMIT)
    .fetch_all(pool)
    .await
    {
        Ok(orgs) => {
            for org in orgs {
                let _ = get_organization_by_id_with_cache(pool, redis, &org.id).await;
            }
            info!("Warmed cache for {} organizations", LIMIT);
        }
        Err(e) => warn!("Failed to fetch organizations for cache warming: {}", e),
    }

    // Activities
    match sqlx::query_as::<_, Activity>(
        "SELECT * FROM activities ORDER BY created_at DESC LIMIT $1",
    )
    .bind(LIMIT)
    .fetch_all(pool)
    .await
    {
        Ok(activities) => {
            for activity in activities {
                let _ = get_activity_by_id_with_cache(pool, redis, &activity.id).await;
            }
            info!("Warmed cache for {} activities", LIMIT);
        }
        Err(e) => warn!("Failed to fetch activities for cache warming: {}", e),
    }
}
