//! Configuration for GraphQL DataFusion

use serde::{Deserialize, Serialize};
use std::time::Duration;
use crate::rate_limit::RateLimitConfig;
use crate::security::SecurityConfig;

/// Configuration for the GraphQL DataFusion server
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    /// Port for the HTTP server
    #[serde(default = "default_http_port")]
    pub http_port: u16,
    
    /// Port for WebSocket server
    #[serde(default = "default_ws_port")]
    pub ws_port: u16,
    
    /// JWT secret for authentication
    pub jwt_secret: String,
    
    /// Rate limiting configuration
    #[serde(default = "default_rate_limit")]
    pub rate_limit: RateLimitConfig,
    
    /// Security headers configuration
    #[serde(default = "default_security")]
    pub security: SecurityConfig,
    
    /// Database connection URL
    pub database_url: String,
    
    /// Cache connection URL
    pub cache_url: String,
    
    /// Logging configuration
    #[serde(default = "default_log_level")]
    pub log_level: String,
    
    /// Tracing configuration
    #[serde(default = "default_tracing_level")]
    pub tracing_level: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            http_port: default_http_port(),
            ws_port: default_ws_port(),
            jwt_secret: "your-secret-key".to_string(),
            rate_limit: default_rate_limit(),
            security: default_security(),
            database_url: "postgresql://localhost/datafusion".to_string(),
            cache_url: "redis://localhost:6379".to_string(),
            log_level: default_log_level(),
            tracing_level: default_tracing_level(),
        }
    }
}

/// Load configuration from environment variables
impl Config {
    /// Creates a new configuration from environment variables
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize()
    }
}

/// Default HTTP port
fn default_http_port() -> u16 {
    8000
}

/// Default WebSocket port
fn default_ws_port() -> u16 {
    8001
}

/// Default log level
fn default_log_level() -> String {
    "info".to_string()
}

/// Default tracing level
fn default_tracing_level() -> String {
    "info".to_string()
}

/// Default rate limit configuration
fn default_rate_limit() -> RateLimitConfig {
    RateLimitConfig {
        window: Duration::from_secs(60),
        limit: 100,
        burst: 5,
        ip_tracking: true,
        headers: true,
    }
}

/// Default security configuration
fn default_security() -> SecurityConfig {
    SecurityConfig {
        enabled: true,
        headers: true,
        cors: true,
        xss_protection: true,
        hsts: true,
        content_security_policy: true,
    }
}

/// Error type for configuration
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),
    
    #[error("Environment variable error: {0}")]
    Env(#[from] std::env::VarError),
    
    #[error("Serialization error: {0}")]
    Serialize(#[from] serde_json::Error),
    
    #[error("Other error: {0}")]
    Other(String),
}

/// Result type for configuration operations
pub type ConfigResult<T> = std::result::Result<T, ConfigError>;
