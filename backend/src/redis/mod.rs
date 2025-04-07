use redis::{Client, Connection};
use std::env;

pub struct RedisClient {
    client: Client,
}

impl Default for RedisClient {
    fn default() -> Self {
        Self::new()
    }
}

impl RedisClient {
    pub fn new() -> Self {
        let redis_url =
            env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
        let client = Client::open(redis_url).expect("Failed to create Redis client");
        Self { client }
    }

    pub fn get_connection(&self) -> redis::RedisResult<Connection> {
        self.client.get_connection()
    }
}
