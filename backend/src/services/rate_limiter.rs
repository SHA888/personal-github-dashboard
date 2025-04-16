use redis::{Client, RedisError};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[allow(dead_code)]
const GITHUB_CORE_RATE_LIMIT: u32 = 5000; // GitHub's default rate limit per hour

#[allow(dead_code)]
const BUCKET_REFILL_RATE: f64 = GITHUB_CORE_RATE_LIMIT as f64 / 3600.0; // Tokens per second

#[allow(dead_code)]
pub struct RateLimiter {
    redis: Client,
    bucket_key: String,
    last_update_key: String,
}

#[allow(dead_code)]
impl RateLimiter {
    #[allow(dependency_on_unit_never_type_fallback)]
    pub fn new(redis_url: &str, bucket_name: &str) -> Result<Self, RedisError> {
        let redis = Client::open(redis_url)?;
        let bucket_key = format!("rate_limit:{}:tokens", bucket_name);
        let last_update_key = format!("rate_limit:{}:last_update", bucket_name);

        Ok(Self {
            redis,
            bucket_key,
            last_update_key,
        })
    }

    #[allow(dependency_on_unit_never_type_fallback)]
    pub async fn acquire_token(&self) -> Result<bool, RedisError> {
        let mut conn = self.redis.get_connection()?;

        // Get current tokens and last update time
        let (tokens, last_update): (Option<f64>, Option<u64>) = redis::pipe()
            .get(&self.bucket_key)
            .get(&self.last_update_key)
            .query(&mut conn)?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let tokens = tokens.unwrap_or(GITHUB_CORE_RATE_LIMIT as f64);
        let last_update = last_update.unwrap_or(now);

        // Calculate token refill
        let elapsed = (now - last_update) as f64;
        let refill = elapsed * BUCKET_REFILL_RATE;
        let new_tokens = (tokens + refill).min(GITHUB_CORE_RATE_LIMIT as f64);

        if new_tokens >= 1.0 {
            // Consume one token
            let remaining_tokens = new_tokens - 1.0;
            redis::pipe()
                .set(&self.bucket_key, remaining_tokens)
                .set(&self.last_update_key, now)
                .query(&mut conn)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[allow(dependency_on_unit_never_type_fallback)]
    pub async fn get_remaining_tokens(&self) -> Result<f64, RedisError> {
        let mut conn = self.redis.get_connection()?;
        let (tokens, last_update): (Option<f64>, Option<u64>) = redis::pipe()
            .get(&self.bucket_key)
            .get(&self.last_update_key)
            .query(&mut conn)?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let tokens = tokens.unwrap_or(GITHUB_CORE_RATE_LIMIT as f64);
        let last_update = last_update.unwrap_or(now);

        let elapsed = (now - last_update) as f64;
        let refill = elapsed * BUCKET_REFILL_RATE;
        Ok((tokens + refill).min(GITHUB_CORE_RATE_LIMIT as f64))
    }

    #[allow(dependency_on_unit_never_type_fallback)]
    pub async fn wait_for_token(&self) -> Result<(), RedisError> {
        while !self.acquire_token().await? {
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        Ok(())
    }
}
