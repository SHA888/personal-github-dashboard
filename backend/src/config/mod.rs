use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub github_token: String,
    pub redis_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            github_token: env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set"),
            redis_url: env::var("REDIS_URL").expect("REDIS_URL must be set"),
        }
    }
} 