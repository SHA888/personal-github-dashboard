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

    pub async fn del(&self, key: &str) -> RedisResult<()> {
        let mut conn = self.client.get_async_connection().await?;
        redis::cmd("DEL").arg(key).query_async(&mut conn).await
    }
}
