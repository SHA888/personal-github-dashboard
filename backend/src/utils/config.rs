use std::env;

pub struct Config {
    pub database_url: String,
    pub github_token: String,
    pub redis_url: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Config {
            database_url: env::var("DATABASE_URL")?,
            github_token: env::var("GITHUB_TOKEN")?,
            redis_url: env::var("REDIS_URL")?,
            port: env::var("PORT")?.parse().unwrap_or(8080),
        })
    }
}
