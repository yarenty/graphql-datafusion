//! Error handling for GraphQL DataFusion

use thiserror::Error;
use async_graphql::Error as GraphQLError;
use sqlx::Error as DatabaseError;
use redis::RedisError;
use std::io::Error as IoError;
use crate::rate_limit::RateLimitError;
use crate::security::SecurityError;
use crate::validation::ValidationError;
use crate::agents::AgentError;

/// Error type for the GraphQL DataFusion server
#[derive(Debug, Error)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),
    
    #[error("Database error: {0}")]
    Database(#[from] DatabaseError),
    
    #[error("Cache error: {0}")]
    Cache(#[from] RedisError),
    
    #[error("GraphQL error: {0}")]
    GraphQL(#[from] GraphQLError),
    
    #[error("Rate limit error: {0}")]
    RateLimit(#[from] RateLimitError),
    
    #[error("Security error: {0}")]
    Security(#[from] SecurityError),
    
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),
    
    #[error("Agent error: {0}")]
    Agent(#[from] AgentError),
    
    #[error("IO error: {0}")]
    Io(#[from] IoError),
    
    #[error("Other error: {0}")]
    Other(String),
}

/// Result type for the GraphQL DataFusion server
pub type Result<T> = std::result::Result<T, Error>;

/// Converts any error into our custom Error type
impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::Other(err.to_string())
    }
}

/// Converts String into our custom Error type
impl From<String> for Error {
    fn from(msg: String) -> Self {
        Error::Other(msg)
    }
}

/// Converts &str into our custom Error type
impl<'a> From<&'a str> for Error {
    fn from(msg: &'a str) -> Self {
        Error::Other(msg.to_string())
    }
}

/// Error extensions for better error handling
pub trait ErrorExt {
    /// Converts error to our custom Error type
    fn into_custom(self) -> Error;
    
    /// Adds context to the error
    fn with_context(self, context: &str) -> Error;
}

impl<T, E: Into<Error>> ErrorExt for std::result::Result<T, E> {
    fn into_custom(self) -> Error {
        self.map_err(|e| e.into())
            .unwrap_or_else(|e| Error::Other("Unknown error".to_string()))
    }
    
    fn with_context(self, context: &str) -> Error {
        self.map_err(|e| Error::Other(format!("{}: {}", context, e)))
            .unwrap_or_else(|e| Error::Other(format!("{}: Unknown error", context)))
    }
}
