use super::config::Config;
use super::config::ConfigError;
use super::config::ConfigResult;
use std::path::PathBuf;
use tracing::Level;
use url::Url;
use crate::rate_limit::RateLimitConfig;
use crate::security::SecurityConfig;

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::str::FromStr;
    use std::time::Duration;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        
        assert_eq!(config.http_port, 8000);
        assert_eq!(config.ws_port, 8001);
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.jwt_expiration, 3600);
        assert_eq!(config.log_level, Level::INFO);
        assert_eq!(config.tracing_level, Level::INFO);
        assert_eq!(config.max_request_size, 10485760);
        assert_eq!(config.request_timeout, 30);
        assert_eq!(config.max_concurrent_requests, 100);
        assert_eq!(config.max_ws_connections, 50);
        assert_eq!(config.enable_caching, true);
        assert_eq!(config.cache_expiration, 3600);
        assert_eq!(config.max_cache_size, 1073741824);
        assert_eq!(config.enable_validation, true);
        assert_eq!(config.max_query_depth, 10);
        assert_eq!(config.max_query_complexity, 100);
        assert_eq!(config.enable_batching, true);
        assert_eq!(config.max_batch_size, 10);
        assert_eq!(config.enable_tracing, true);
        assert_eq!(config.enable_request_logging, true);
        assert_eq!(config.enable_metrics, true);
        assert_eq!(config.metrics_interval, 60);
        assert_eq!(config.enable_optimization, true);
        assert_eq!(config.optimization_level, 2);
        
        // Test rate limit config
        assert_eq!(config.rate_limit.window, Duration::from_secs(60));
        assert_eq!(config.rate_limit.limit, 100);
        assert_eq!(config.rate_limit.burst, 5);
        assert_eq!(config.rate_limit.ip_tracking, true);
        assert_eq!(config.rate_limit.headers, true);
        
        // Test security config
        assert_eq!(config.security.enabled, true);
        assert_eq!(config.security.headers, true);
        assert_eq!(config.security.cors, true);
        assert_eq!(config.security.xss_protection, true);
        assert_eq!(config.security.hsts, true);
        assert_eq!(config.security.content_security_policy, true);
    }

    #[test]
    fn test_env_config() {
        // Set environment variables
        env::set_var("HTTP_PORT", "8080");
        env::set_var("WS_PORT", "8081");
        env::set_var("HOST", "localhost");
        env::set_var("JWT_SECRET", "test-secret");
        env::set_var("JWT_EXPIRATION", "7200");
        env::set_var("DATABASE_URL", "postgresql://test:5432/test");
        env::set_var("CACHE_URL", "redis://test:6379");
        env::set_var("LOG_LEVEL", "debug");
        env::set_var("TRACING_LEVEL", "trace");
        env::set_var("MAX_REQUEST_SIZE", "20971520");
        env::set_var("REQUEST_TIMEOUT", "60");
        env::set_var("MAX_CONCURRENT_REQUESTS", "200");
        env::set_var("MAX_WS_CONNECTIONS", "100");
        env::set_var("ENABLE_CACHING", "false");
        env::set_var("CACHE_EXPIRATION", "7200");
        env::set_var("MAX_CACHE_SIZE", "2147483648");
        env::set_var("ENABLE_VALIDATION", "false");
        env::set_var("MAX_QUERY_DEPTH", "15");
        env::set_var("MAX_QUERY_COMPLEXITY", "200");
        env::set_var("ENABLE_BATCHING", "false");
        env::set_var("MAX_BATCH_SIZE", "20");
        env::set_var("ENABLE_TRACING", "false");
        env::set_var("ENABLE_REQUEST_LOGGING", "false");
        env::set_var("ENABLE_METRICS", "false");
        env::set_var("METRICS_INTERVAL", "120");
        env::set_var("ENABLE_OPTIMIZATION", "false");
        env::set_var("OPTIMIZATION_LEVEL", "3");
        
        let config = Config::from_env().unwrap();
        
        assert_eq!(config.http_port, 8080);
        assert_eq!(config.ws_port, 8081);
        assert_eq!(config.host, "localhost");
        assert_eq!(config.jwt_expiration, 7200);
        assert_eq!(config.log_level, Level::DEBUG);
        assert_eq!(config.tracing_level, Level::TRACE);
        assert_eq!(config.max_request_size, 20971520);
        assert_eq!(config.request_timeout, 60);
        assert_eq!(config.max_concurrent_requests, 200);
        assert_eq!(config.max_ws_connections, 100);
        assert_eq!(config.enable_caching, false);
        assert_eq!(config.cache_expiration, 7200);
        assert_eq!(config.max_cache_size, 2147483648);
        assert_eq!(config.enable_validation, false);
        assert_eq!(config.max_query_depth, 15);
        assert_eq!(config.max_query_complexity, 200);
        assert_eq!(config.enable_batching, false);
        assert_eq!(config.max_batch_size, 20);
        assert_eq!(config.enable_tracing, false);
        assert_eq!(config.enable_request_logging, false);
        assert_eq!(config.enable_metrics, false);
        assert_eq!(config.metrics_interval, 120);
        assert_eq!(config.enable_optimization, false);
        assert_eq!(config.optimization_level, 3);
    }

    #[test]
    fn test_config_validation() {
        // Test valid config
        let config = Config {
            http_port: 8000,
            ws_port: 8001,
            host: "localhost".to_string(),
            jwt_expiration: 3600,
            ..Default::default()
        };
        assert!(config.validate().is_ok());
        
        // Test invalid HTTP port
        let config = Config {
            http_port: 0,
            ..Default::default()
        };
        assert!(config.validate().is_err());
        
        // Test invalid WebSocket port
        let config = Config {
            ws_port: 0,
            ..Default::default()
        };
        assert!(config.validate().is_err());
        
        // Test invalid JWT expiration
        let config = Config {
            jwt_expiration: 0,
            ..Default::default()
        };
        assert!(config.validate().is_err());
        
        // Test invalid request size
        let config = Config {
            max_request_size: 0,
            ..Default::default()
        };
        assert!(config.validate().is_err());
        
        // Test invalid request timeout
        let config = Config {
            request_timeout: 0,
            ..Default::default()
        };
        assert!(config.validate().is_err());
        
        // Test invalid concurrent requests
        let config = Config {
            max_concurrent_requests: 0,
            ..Default::default()
        };
        assert!(config.validate().is_err());
        
        // Test invalid WebSocket connections
        let config = Config {
            max_ws_connections: 0,
            ..Default::default()
        };
        assert!(config.validate().is_err());
        
        // Test invalid cache expiration
        let config = Config {
            cache_expiration: 0,
            ..Default::default()
        };
        assert!(config.validate().is_err());
        
        // Test invalid cache size
        let config = Config {
            max_cache_size: 0,
            ..Default::default()
        };
        assert!(config.validate().is_err());
        
        // Test invalid query depth
        let config = Config {
            max_query_depth: 0,
            ..Default::default()
        };
        assert!(config.validate().is_err());
        
        // Test invalid query complexity
        let config = Config {
            max_query_complexity: 0,
            ..Default::default()
        };
        assert!(config.validate().is_err());
        
        // Test invalid batch size
        let config = Config {
            max_batch_size: 0,
            ..Default::default()
        };
        assert!(config.validate().is_err());
        
        // Test invalid optimization level
        let config = Config {
            optimization_level: 4,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_url_validation() {
        // Test valid database URL
        let config = Config {
            database_url: "postgresql://localhost:5432/test".to_string(),
            ..Default::default()
        };
        assert!(config.verify_database_url().is_ok());
        
        // Test invalid database URL
        let config = Config {
            database_url: "invalid://url".to_string(),
            ..Default::default()
        };
        assert!(config.verify_database_url().is_err());
        
        // Test valid cache URL
        let config = Config {
            cache_url: "redis://localhost:6379".to_string(),
            ..Default::default()
        };
        assert!(config.verify_cache_url().is_ok());
        
        // Test invalid cache URL
        let config = Config {
            cache_url: "invalid://url".to_string(),
            ..Default::default()
        };
        assert!(config.verify_cache_url().is_err());
    }

    #[test]
    fn test_tls_config_validation() {
        // Test no TLS config
        let config = Config {
            tls_cert: None,
            tls_key: None,
            ..Default::default()
        };
        assert!(config.verify_tls_config().is_ok());
        
        // Test both TLS config
        let config = Config {
            tls_cert: Some(PathBuf::from("cert.pem")),
            tls_key: Some(PathBuf::from("key.pem")),
            ..Default::default()
        };
        assert!(config.verify_tls_config().is_ok());
        
        // Test missing cert
        let config = Config {
            tls_cert: None,
            tls_key: Some(PathBuf::from("key.pem")),
            ..Default::default()
        };
        assert!(config.verify_tls_config().is_err());
        
        // Test missing key
        let config = Config {
            tls_cert: Some(PathBuf::from("cert.pem")),
            tls_key: None,
            ..Default::default()
        };
        assert!(config.verify_tls_config().is_err());
    }
}
