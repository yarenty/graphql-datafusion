//! Configuration for GraphQL DataFusion
//!
//! This module provides configuration management for the GraphQL DataFusion server.
//! It includes:
//!
//! - Configuration struct with all server settings
//! - Environment variable loading
//! - Configuration validation
//! - Default values
//! - Error handling
//!

use crate::error::Error as AppError;
use config::{Config as ConfigLib, ConfigError as ConfigLibError};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;

/// Configuration for the GraphQL DataFusion server
///
/// This struct holds all configuration values for the server.
/// It can be loaded from environment variables or a configuration file.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    /// HTTP server port
    #[serde(default = "default_http_port")]
    pub http_port: u16,

    /// WebSocket server port
    #[serde(default = "default_ws_port")]
    pub ws_port: u16,

    /// JWT secret
    #[serde(default = "default_jwt_secret")]
    pub jwt_secret: String,

    /// JWT expiration in seconds
    #[serde(default = "default_jwt_expiration_secs")]
    pub jwt_expiration_secs: u64,

    /// Maximum request size in bytes
    #[serde(default = "default_max_request_size")]
    pub max_request_size: usize,

    /// Request timeout in seconds
    #[serde(default = "default_request_timeout")]
    pub request_timeout: u64,

    /// Maximum concurrent requests
    #[serde(default = "default_max_concurrent_requests")]
    pub max_concurrent_requests: usize,

    /// Maximum WebSocket connections
    #[serde(default = "default_max_ws_connections")]
    pub max_ws_connections: usize,

    /// Query validation

    /// Path to TLS certificate (for HTTPS)
    #[serde(default = "default_tls_cert")]
    pub tls_cert: Option<PathBuf>,

    /// Path to TLS private key (for HTTPS)
    #[serde(default = "default_tls_key")]
    pub tls_key: Option<PathBuf>,

    /// Enable query caching
    #[serde(default = "default_enable_caching")]
    pub enable_caching: bool,

    /// Cache expiration time in seconds
    #[serde(default = "default_cache_expiration")]
    pub cache_expiration: u64,

    /// Maximum cache size in bytes
    #[serde(default = "default_max_cache_size")]
    pub max_cache_size: u64,

    /// Enable query validation
    #[serde(default = "default_enable_validation")]
    pub enable_validation: bool,

    /// Maximum query depth
    #[serde(default = "default_max_query_depth")]
    pub max_query_depth: usize,

    /// Maximum query complexity
    #[serde(default = "default_max_query_complexity")]
    pub max_query_complexity: usize,

    /// Enable query batching
    #[serde(default = "default_enable_batching")]
    pub enable_batching: bool,

    /// Maximum batch size
    #[serde(default = "default_max_batch_size")]
    pub max_batch_size: usize,

    /// Enable query tracing
    #[serde(default = "default_enable_tracing")]
    pub enable_tracing: bool,

    /// Enable request logging
    #[serde(default = "default_enable_request_logging")]
    pub enable_request_logging: bool,

    /// Enable metrics collection
    #[serde(default = "default_enable_metrics")]
    pub enable_metrics: bool,

    /// Metrics collection interval in seconds
    #[serde(default = "default_metrics_interval")]
    pub metrics_interval: u64,

    /// Enable query optimization
    #[serde(default = "default_enable_optimization")]
    pub enable_optimization: bool,

    /// Query optimization level (0-3)
    #[serde(default = "default_optimization_level")]
    pub optimization_level: u8,
}

impl Default for Config {
    /// Creates a new configuration with default values
    fn default() -> Self {
        Self {
            http_port: default_http_port(),
            ws_port: default_ws_port(),
            jwt_secret: default_jwt_secret(),
            jwt_expiration_secs: default_jwt_expiration_secs(),
            max_request_size: default_max_request_size(),
            request_timeout: default_request_timeout(),
            max_concurrent_requests: default_max_concurrent_requests(),
            max_ws_connections: default_max_ws_connections(),
            max_query_depth: default_max_query_depth(),
            max_query_complexity: default_max_query_complexity(),
            max_batch_size: default_max_batch_size(),
            optimization_level: default_optimization_level(),
            tls_cert: default_tls_cert(),
            tls_key: default_tls_key(),
            enable_caching: default_enable_caching(),
            cache_expiration: default_cache_expiration(),
            max_cache_size: default_max_cache_size(),
            enable_validation: default_enable_validation(),
            enable_batching: default_enable_batching(),
            enable_tracing: default_enable_tracing(),
            enable_request_logging: default_enable_request_logging(),
            enable_metrics: default_enable_metrics(),
            metrics_interval: default_metrics_interval(),
            enable_optimization: default_enable_optimization(),
        }
    }
}

/// Configuration operations
impl Config {
    /// Create a new configuration from environment variables
    pub fn from_env() -> Result<Self, AppError> {
        let mut config = ConfigLib::builder().add_source(config::Environment::default());

        config
            .build()
            .map_err(|err| AppError::Config(err))?
            .try_deserialize()
            .map_err(|err| AppError::Config(err))
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), AppError> {
        if self.http_port == 0 || self.http_port > 65535 {
            return Err(AppError::Config(ConfigLibError::Message(
                "Invalid HTTP port number".to_string(),
            )));
        }

        if self.ws_port == 0 || self.ws_port > 65535 {
            return Err(AppError::Config(ConfigLibError::Message(
                "Invalid WebSocket port number".to_string(),
            )));
        }

        if self.jwt_expiration_secs < 60 {
            return Err(AppError::Config(ConfigLibError::Message(
                "JWT expiration must be at least 60 seconds".to_string(),
            )));
        }

        if self.max_request_size < 1024 {
            return Err(AppError::Config(ConfigLibError::Message(
                "Maximum request size must be at least 1KB".to_string(),
            )));
        }

        if self.request_timeout < 1 {
            return Err(AppError::Config(ConfigLibError::Message(
                "Request timeout must be at least 1 second".to_string(),
            )));
        }

        if self.max_concurrent_requests < 1 {
            return Err(AppError::Config(ConfigLibError::Message(
                "Maximum concurrent requests must be at least 1".to_string(),
            )));
        }

        if self.max_ws_connections < 1 {
            return Err(AppError::Config(ConfigLibError::Message(
                "Maximum WebSocket connections must be at least 1".to_string(),
            )));
        }

        if self.max_query_depth < 1 {
            return Err(AppError::Config(ConfigLibError::Message(
                "Maximum query depth must be at least 1".to_string(),
            )));
        }

        if self.max_query_complexity < 1 {
            return Err(AppError::Config(ConfigLibError::Message(
                "Maximum query complexity must be at least 1".to_string(),
            )));
        }

        if self.max_batch_size < 1 {
            return Err(AppError::Config(ConfigLibError::Message(
                "Maximum batch size must be at least 1".to_string(),
            )));
        }

        if self.optimization_level > 3 {
            return Err(AppError::Config(ConfigLibError::Message(
                "Optimization level must be between 0 and 3".to_string(),
            )));
        }

        Ok(())
    }
}

/// Default configuration values

/// Default HTTP port
const fn default_http_port() -> u16 {
    8080
}

/// Default WebSocket port
const fn default_ws_port() -> u16 {
    8081
}

/// Default JWT secret
const fn default_jwt_secret() -> String {
    "your-secret-key".to_string()
}

/// Default JWT expiration
const fn default_jwt_expiration_secs() -> u64 {
    3600
}

/// Default maximum request size
const fn default_max_request_size() -> usize {
    10485760 // 10MB
}

/// Default request timeout
const fn default_request_timeout() -> u64 {
    30
}

/// Default maximum concurrent requests
const fn default_max_concurrent_requests() -> usize {
    100
}

/// Default maximum WebSocket connections
const fn default_max_ws_connections() -> usize {
    100
}

/// Default rate limiting window
const fn default_rate_limit_window() -> u64 {
    60
}

/// Default rate limiting max requests
const fn default_rate_limit_max_requests() -> usize {
    100
}

/// Default optimization threshold
const fn default_optimization_threshold() -> f64 {
    0.5
}

/// Default metrics interval
const fn default_metrics_interval_secs() -> u64 {
    60
}

/// Default batch size
const fn default_batch_size() -> usize {
    100
}

/// Default batch timeout
const fn default_batch_timeout_secs() -> u64 {
    1
}

/// Default optimization timeout
const fn default_optimization_timeout_secs() -> u64 {
    5
}

/// Default optimization batch size
const fn default_optimization_batch_size() -> usize {
    10
}

/// Default TLS certificate
fn default_tls_cert() -> Option<PathBuf> {
    None
}

/// Default TLS private key
fn default_tls_key() -> Option<PathBuf> {
    None
}

/// Default enable caching
fn default_enable_caching() -> bool {
    true
}

/// Default cache expiration
fn default_cache_expiration() -> u64 {
    3600 // 1 hour
}

/// Default maximum cache size
fn default_max_cache_size() -> u64 {
    1073741824 // 1GB
}

/// Default enable validation
fn default_enable_validation() -> bool {
    true
}

/// Default maximum query depth
fn default_max_query_depth() -> usize {
    10
}

/// Default maximum query complexity
fn default_max_query_complexity() -> usize {
    100
}

/// Default enable batching
fn default_enable_batching() -> bool {
    true
}

/// Default maximum batch size
fn default_max_batch_size() -> usize {
    10
}

/// Default enable tracing
fn default_enable_tracing() -> bool {
    true
}

/// Default enable request logging
fn default_enable_request_logging() -> bool {
    true
}

/// Default enable metrics
fn default_enable_metrics() -> bool {
    true
}

/// Default metrics interval
fn default_metrics_interval() -> u64 {
    60 // 1 minute
}

/// Default enable optimization
fn default_enable_optimization() -> bool {
    true
}

/// Default optimization level
fn default_optimization_level() -> u8 {
    2
}

/// Result type for configuration operations
///
/// This type is used for all configuration-related operations that might fail.
pub type ConfigResult<T> = std::result::Result<T, AppError>;
