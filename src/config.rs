use serde::Deserialize;
use std::env;

/// Application configuration loaded from environment variables
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub api: ApiConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub address: String,
    pub port: u16,
    pub workers: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiConfig {
    pub api_key: String,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, config::ConfigError> {
        // Default workers: try to get from env, otherwise default to 4
        // In production, set SERVER__WORKERS to match your CPU count
        let default_workers = env::var("SERVER__WORKERS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(4);

        let config = config::Config::builder()
            .add_source(config::Environment::default())
            .set_default("server.address", "0.0.0.0")?
            .set_default("server.port", 8080)?
            .set_default("server.workers", default_workers)?
            .set_default("api.api_key", "dev-api-key-change-in-production")?
            .build()?;

        config.try_deserialize()
    }
}
