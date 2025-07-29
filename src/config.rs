//! Configuration for GraphQL DataFusion
//!
//! Simplified configuration management for the GraphQL DataFusion server.

use serde::{Deserialize, Serialize};
use std::env;

/// Configuration for the GraphQL DataFusion server
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// HTTP server port
    pub http_port: u16,
    
    /// Data file path (CSV or Parquet)
    pub data_path: String,
    
    /// Table name for DataFusion
    pub table_name: String,
    
    /// Ollama API URL
    pub ollama_url: String,
    
    /// Ollama model name
    pub ollama_model: String,
    
    /// Enable metrics collection
    pub enable_metrics: bool,
    
    /// Log level
    pub log_level: String,
    
    /// Maximum query timeout in seconds
    pub query_timeout: u64,
    
    /// Enable query caching
    pub enable_caching: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            http_port: 8080,
            data_path: "/opt/data/tpch".to_string(),
            table_name: "customer".to_string(),
            ollama_url: "http://localhost:11434".to_string(),
            ollama_model: "llama2".to_string(),
            enable_metrics: true,
            log_level: "info".to_string(),
            query_timeout: 30,
            enable_caching: true,
        }
    }
}

impl Config {
    /// Create configuration from environment variables
    pub fn from_env() -> Self {
        let mut config = Self::default();
        
        if let Ok(port) = env::var("HTTP_PORT") {
            if let Ok(port_num) = port.parse() {
                config.http_port = port_num;
            }
        }
        
        if let Ok(path) = env::var("DATA_PATH") {
            config.data_path = path;
        }
        
        if let Ok(table) = env::var("TABLE_NAME") {
            config.table_name = table;
        }
        
        if let Ok(url) = env::var("OLLAMA_URL") {
            config.ollama_url = url;
        }
        
        if let Ok(model) = env::var("OLLAMA_MODEL") {
            config.ollama_model = model;
        }
        
        if let Ok(level) = env::var("LOG_LEVEL") {
            config.log_level = level;
        }
        
        if let Ok(timeout) = env::var("QUERY_TIMEOUT") {
            if let Ok(timeout_num) = timeout.parse() {
                config.query_timeout = timeout_num;
            }
        }
        
        config
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.http_port == 0 || self.http_port > 65535 {
            return Err("Invalid HTTP port number".to_string());
        }

        if self.data_path.is_empty() {
            return Err("Data path cannot be empty".to_string());
        }

        if self.table_name.is_empty() {
            return Err("Table name cannot be empty".to_string());
        }

        if self.ollama_url.is_empty() {
            return Err("Ollama URL cannot be empty".to_string());
        }

        if self.ollama_model.is_empty() {
            return Err("Ollama model cannot be empty".to_string());
        }

        if self.query_timeout == 0 {
            return Err("Query timeout must be greater than 0".to_string());
        }

        Ok(())
    }
}
