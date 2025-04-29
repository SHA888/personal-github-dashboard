use redis::{AsyncCommands, Client, RedisResult};
use std::sync::Arc;

#[derive(Clone)]
pub struct RedisClient {
    pub client: Arc<Client>,
}

impl RedisClient {
    pub async fn new(redis_url: &str) -> RedisResult<Self> {
        let client = Client::open(redis_url)?;
        Ok(RedisClient {
            client: Arc::new(client),
        })
    }

    pub async fn get<T: redis::FromRedisValue + Send + 'static>(
        &self,
        key: &str,
    ) -> RedisResult<Option<T>> {
        let mut conn = self.client.get_async_connection().await?;
        conn.get(key).await
    }

    /// Sets a value in Redis with a specified time-to-live (TTL) in seconds.
    ///
    /// Stores the given value under the provided key and sets its expiration time atomically.
    ///
    /// # Examples
    ///
    /// ```
    /// # use your_crate::RedisClient;
    /// # async fn example() -> redis::RedisResult<()> {
    /// let client = RedisClient::new("redis://127.0.0.1/").await?;
    /// client.set("my_key", "my_value", 60).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set<T: redis::ToRedisArgs + Send + Sync + 'static>(
        &self,
        key: &str,
        value: T,
        ttl_seconds: usize,
    ) -> RedisResult<()> {
        let mut conn = self.client.get_async_connection().await?;
        redis::pipe()
            .set(key, value)
            .expire(key, ttl_seconds)
            .query_async::<_, ()>(&mut conn)
            .await?;
        Ok(())
    }

    /// Deletes a key from Redis asynchronously.
    ///
    /// Removes the specified key from the Redis database if it exists.
    ///
    /// # Examples
    ///
    /// ```
    /// # use backend::utils::redis::RedisClient;
    /// # async fn example() -> redis::RedisResult<()> {
    /// let client = RedisClient::new("redis://127.0.0.1/").await?;
    /// client.del("my_key").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn del(&self, key: &str) -> RedisResult<()> {
        let mut conn = self.client.get_async_connection().await?;
        redis::cmd("DEL").arg(key).query_async(&mut conn).await
    }
}
