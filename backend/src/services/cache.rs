use redis::{Client, RedisError};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

/// Cache service for storing and retrieving data using Redis
#[allow(dead_code)]
pub struct CacheService {
    redis: Client,
    prefix: String,
}

#[allow(dead_code)]
impl CacheService {
    /// Create a new CacheService instance
    pub fn new(redis_url: &str, prefix: &str) -> Result<Self, RedisError> {
        let redis = Client::open(redis_url)?;
        Ok(Self {
            redis,
            prefix: prefix.to_string(),
        })
    }

    /// Generate a prefixed key for Redis
    fn get_prefixed_key(&self, key: &str) -> String {
        format!("{}:{}", self.prefix, key)
    }

    /// Set a value in the cache with optional TTL
    #[allow(dependency_on_unit_never_type_fallback)]
    pub async fn set<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        ttl: Option<Duration>,
    ) -> Result<(), RedisError> {
        let mut conn = self.redis.get_async_connection().await?;
        let key = self.get_prefixed_key(key);

        // Serialize the value to JSON
        let serialized = serde_json::to_string(value).map_err(|e| {
            RedisError::from((
                redis::ErrorKind::IoError,
                "Serialization error",
                e.to_string(),
            ))
        })?;

        let mut pipe = redis::pipe();
        pipe.set(&key, serialized);

        if let Some(ttl) = ttl {
            pipe.expire(&key, ttl.as_secs() as usize);
        }

        pipe.query_async(&mut conn).await?;
        Ok(())
    }

    /// Get a value from the cache
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, RedisError> {
        let mut conn = self.redis.get_async_connection().await?;
        let key = self.get_prefixed_key(key);

        let value: Option<String> = redis::cmd("GET").arg(&key).query_async(&mut conn).await?;

        match value {
            Some(serialized) => serde_json::from_str(&serialized)
                .map_err(|e| {
                    RedisError::from((
                        redis::ErrorKind::IoError,
                        "Deserialization error",
                        e.to_string(),
                    ))
                })
                .map(Some),
            None => Ok(None),
        }
    }

    /// Delete a value from the cache
    pub async fn delete(&self, key: &str) -> Result<bool, RedisError> {
        let mut conn = self.redis.get_async_connection().await?;
        let key = self.get_prefixed_key(key);
        let deleted: i32 = redis::cmd("DEL").arg(&key).query_async(&mut conn).await?;
        Ok(deleted > 0)
    }

    /// Check if a key exists in the cache
    pub async fn exists(&self, key: &str) -> Result<bool, RedisError> {
        let mut conn = self.redis.get_async_connection().await?;
        let key = self.get_prefixed_key(key);
        let exists: i32 = redis::cmd("EXISTS")
            .arg(&key)
            .query_async(&mut conn)
            .await?;
        Ok(exists > 0)
    }

    /// Get the remaining TTL for a key in seconds
    pub async fn ttl(&self, key: &str) -> Result<i64, RedisError> {
        let mut conn = self.redis.get_async_connection().await?;
        let key = self.get_prefixed_key(key);
        redis::cmd("TTL").arg(&key).query_async(&mut conn).await
    }

    /// Set multiple values in the cache with the same TTL
    #[allow(dependency_on_unit_never_type_fallback)]
    pub async fn set_many<T: Serialize>(
        &self,
        items: &[(&str, &T)],
        ttl: Option<Duration>,
    ) -> Result<(), RedisError> {
        let mut conn = self.redis.get_async_connection().await?;
        let mut pipe = redis::pipe();

        for (key, value) in items {
            let key = self.get_prefixed_key(key);
            let serialized = serde_json::to_string(value).map_err(|e| {
                RedisError::from((
                    redis::ErrorKind::IoError,
                    "Serialization error",
                    e.to_string(),
                ))
            })?;

            pipe.set(&key, serialized);
            if let Some(ttl) = ttl {
                pipe.expire(&key, ttl.as_secs() as usize);
            }
        }

        pipe.query_async(&mut conn).await?;
        Ok(())
    }

    /// Delete multiple keys from the cache
    pub async fn delete_many(&self, keys: &[&str]) -> Result<i32, RedisError> {
        if keys.is_empty() {
            return Ok(0);
        }

        let mut conn = self.redis.get_async_connection().await?;
        let prefixed_keys: Vec<String> = keys.iter().map(|&k| self.get_prefixed_key(k)).collect();

        redis::cmd("DEL")
            .arg(&prefixed_keys)
            .query_async(&mut conn)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use tokio;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestStruct {
        field1: String,
        field2: i32,
    }

    #[tokio::test]
    async fn test_basic_operations() -> Result<(), RedisError> {
        let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
        let cache = CacheService::new(&redis_url, "test")?;

        // Test data
        let test_data = TestStruct {
            field1: "test".to_string(),
            field2: 42,
        };

        // Test set and get
        cache.set("test_key", &test_data, None).await?;
        let retrieved: TestStruct = cache.get("test_key").await?.unwrap();
        assert_eq!(retrieved, test_data);

        // Test exists
        assert!(cache.exists("test_key").await?);

        // Test delete
        assert!(cache.delete("test_key").await?);
        assert!(!cache.exists("test_key").await?);

        Ok(())
    }

    #[tokio::test]
    async fn test_ttl() -> Result<(), RedisError> {
        let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
        let cache = CacheService::new(&redis_url, "test")?;

        let test_data = "test_value";
        cache
            .set("ttl_key", &test_data, Some(Duration::from_secs(10)))
            .await?;

        let ttl = cache.ttl("ttl_key").await?;
        assert!(ttl > 0 && ttl <= 10);

        Ok(())
    }
}
