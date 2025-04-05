use redis::{Client, AsyncCommands};
use serde::{Serialize, de::DeserializeOwned};
use std::time::Duration;
use std::env;

pub struct Cache {
    client: Client,
}

impl Cache {
    pub fn new() -> Result<Self, redis::RedisError> {
        let redis_url = env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
        
        let client = Client::open(redis_url)?;
        Ok(Self { client })
    }

    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, redis::RedisError> {
        let mut conn = self.client.get_async_connection().await?;
        let data: Option<String> = conn.get(key).await?;
        
        match data {
            Some(json) => Ok(serde_json::from_str(&json).ok()),
            None => Ok(None),
        }
    }

    pub async fn set<T: Serialize>(&self, key: &str, value: &T, ttl: Option<Duration>) -> Result<(), redis::RedisError> {
        let mut conn = self.client.get_async_connection().await?;
        let json = serde_json::to_string(value)?;
        
        if let Some(ttl) = ttl {
            conn.set_ex(key, json, ttl.as_secs() as usize).await?;
        } else {
            conn.set(key, json).await?;
        }
        
        Ok(())
    }

    pub async fn delete(&self, key: &str) -> Result<(), redis::RedisError> {
        let mut conn = self.client.get_async_connection().await?;
        conn.del(key).await?;
        Ok(())
    }

    pub async fn exists(&self, key: &str) -> Result<bool, redis::RedisError> {
        let mut conn = self.client.get_async_connection().await?;
        Ok(conn.exists(key).await?)
    }
} 