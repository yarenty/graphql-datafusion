//! Error handling for GraphQL DataFusion
//!
//! This module provides comprehensive error handling for the GraphQL DataFusion server.
//! It includes:
//!
//! - A unified error type that can represent various error sources
//! - Error conversion utilities
//! - Error context tracking
//! - Error categorization
//! - Error handling traits
//!
//! # Examples
//!
//! ```rust
//! use graphql_datafusion::error::{Error, ErrorKind, error};
//!
//! // Create an error from a string
//! let error = error("Something went wrong").build();
//!
//! // Add context to an error
//! let error = error("Database error")
//!     .with_context("While connecting to database")
//!     .build();
//!
//! // Check error kind
//! if let ErrorKind::Database = error.kind() {
//!     // Handle database error
//! }
//! ```

use thiserror::Error;
use async_graphql::Error as GraphQLError;
use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::num::ParseIntError;
use std::string::FromUtf8Error;
use std::sync::PoisonError;
use std::time::SystemTimeError;

use anyhow::Error as AnyhowError;
use async_graphql::Error as GraphQLError;
use config::ConfigError;
use thiserror::Error;
use tokio::sync::broadcast::error::{RecvError, SendError};
use tokio::task::JoinError;
use url::ParseError;

use actix_web::error::{BlockingError, PayloadError};
use actix_web::http::Error as HttpError;
use actix_web::Error as ActixError;
use actix_web::web::FormError;
use actix_web::web::PathError;
use actix_web::web::QueryPayloadError;
use actix_web::web::JsonPayloadError;
use actix_web::web::JsonError;
use actix_web::web::ParseError as ActixParseError;
use actix_web::web::PayloadError as ActixPayloadError;
use serde_json::Error as JsonError;

/// Error type for the GraphQL DataFusion server
///
/// This enum represents all possible error states that can occur in the server.
/// Each variant corresponds to a specific error source or category.
#[derive(Debug, Error)]
pub enum Error {
    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),
    
    /// Environment variable error
    #[error("Environment variable error: {0}")]
    Env(#[from] std::env::VarError),
    
    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialize(#[from] serde_json::Error),
    
    /// URL parse error
    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    /// GraphQL error
    #[error("GraphQL error: {0}")]
    GraphQL(#[from] GraphQLError),
    
    /// Rate limit error
    #[error("Rate limit error: {0}")]
    RateLimit(#[from] RateLimitError),
    
    /// Security error
    #[error("Security error: {0}")]
    Security(#[from] SecurityError),
    
    /// Validation error
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),
    
    /// Agent error
    #[error("Agent error: {0}")]
    Agent(#[from] AgentError),
    
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] IoError),
    

    
    /// UTF-8 error
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] Utf8Error),
    
    /// From UTF-8 error
    #[error("From UTF-8 error: {0}")]
    FromUtf8(#[from] FromUtf8Error),
    
    /// System time error
    #[error("System time error: {0}")]
    SystemTime(#[from] SystemTimeError),
    
    /// Poison error
    #[error("Poison error: {0}")]
    Poison(#[from] PoisonError<()>),
    
    /// Parse int error
    #[error("Parse int error: {0}")]
    ParseInt(#[from] ParseIntError),
    
    /// Actix error
    #[error("Actix error: {0}")]
    Actix(#[from] ActixError),
    
    /// HTTP error
    #[error("HTTP error: {0}")]
    Http(#[from] HttpError),
    
    /// Blocking error
    #[error("Blocking error: {0}")]
    Blocking(#[from] BlockingError),
    
    /// Payload error
    #[error("Payload error: {0}")]
    Payload(#[from] PayloadError),
    
    /// Actix parse error
    #[error("Actix parse error: {0}")]
    ActixParse(#[from] ActixParseError),
    
    /// Query payload error
    #[error("Query payload error: {0}")]
    QueryPayload(#[from] QueryPayloadError),
    
    /// JSON payload error
    #[error("JSON payload error: {0}")]
    JsonPayload(#[from] JsonPayloadError),
    
    /// Path error
    #[error("Path error: {0}")]
    Path(#[from] PathError),
    
    /// Form error
    #[error("Form error: {0}")]
    Form(#[from] FormError),
    
    /// Response error
    #[error("Response error: {0}")]
    Response(Box<dyn std::error::Error + Send + Sync>),
    
    /// Actix payload error
    #[error("Actix payload error: {0}")]
    ActixPayload(#[from] ActixPayloadError),
    
    /// Send error
    #[error("Send error: {0}")]
    Send(#[from] SendError<()>),
    
    /// Join error
    #[error("Join error: {0}")]
    Join(#[from] JoinError),
    
    /// Other error
    #[error("Other error: {0}")]
    Other(String),
}

/// Result type for the GraphQL DataFusion server
///
/// This type is used throughout the codebase to represent operations that
/// might fail with an `Error`.
pub type Result<T> = std::result::Result<T, Error>;

/// Converts any error into our custom Error type
///
/// This implementation allows converting any error type that implements
/// `Display` into our custom `Error` type.
impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::Other(err.to_string())
    }
}

/// Converts String into our custom Error type
///
/// This implementation allows creating errors from string literals.
impl From<String> for Error {
    fn from(msg: String) -> Self {
        Error::Other(msg)
    }
}

/// Converts &str into our custom Error type
///
/// This implementation allows creating errors from string slices.
impl<'a> From<&'a str> for Error {
    fn from(msg: &'a str) -> Self {
        Error::Other(msg.to_string())
    }
}

/// Error extensions for better error handling
///
/// This trait provides additional methods for handling errors.
pub trait ErrorExt {
    /// Converts error to our custom Error type
    ///
    /// This method converts any error type into our custom `Error` type.
    fn into_custom(self) -> Error;
    
    /// Adds context to the error
    ///
    /// This method adds contextual information to an error.
    fn with_context(self, context: &str) -> Error;
    
    /// Converts error to a GraphQL error
    ///
    /// This method converts an error into a GraphQL error.
    fn into_graphql(self) -> GraphQLError;
    
    /// Converts error to an HTTP error
    ///
    /// This method converts an error into an HTTP error.
    fn into_http(self) -> HttpError;
    
    /// Gets the error kind
    ///
    /// This method returns the kind of error.
    fn kind(&self) -> ErrorKind;
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
    
    fn into_graphql(self) -> GraphQLError {
        self.map_err(|e| GraphQLError::new(format!("GraphQL error: {}", e)))
            .unwrap_or_else(|e| GraphQLError::new("Unknown GraphQL error"))
    }
    
    fn into_http(self) -> HttpError {
        self.map_err(|e| HttpError::from(Error::from(e)))
            .unwrap_or_else(|e| HttpError::from(Error::Other("Unknown HTTP error".to_string())))
    }
    
    fn kind(&self) -> ErrorKind {
        match self.as_ref() {
            Ok(_) => ErrorKind::Ok,
            Err(e) => match e.into_custom() {
                Error::Config(_) => ErrorKind::Config,
                Error::Database(_) => ErrorKind::Database,
                Error::Cache(_) => ErrorKind::Cache,
                Error::GraphQL(_) => ErrorKind::GraphQL,
                Error::RateLimit(_) => ErrorKind::RateLimit,
                Error::Security(_) => ErrorKind::Security,
                Error::Validation(_) => ErrorKind::Validation,
                Error::Agent(_) => ErrorKind::Agent,
                Error::Io(_) => ErrorKind::Io,
                Error::Parse(_) => ErrorKind::Parse,
                Error::Json(_) => ErrorKind::Json,
                Error::Utf8(_) => ErrorKind::Utf8,
                Error::FromUtf8(_) => ErrorKind::FromUtf8,
                Error::SystemTime(_) => ErrorKind::SystemTime,
                Error::Poison(_) => ErrorKind::Poison,
                Error::ParseInt(_) => ErrorKind::ParseInt,
                Error::Actix(_) => ErrorKind::Actix,
                Error::Http(_) => ErrorKind::Http,
                Error::Blocking(_) => ErrorKind::Blocking,
                Error::Payload(_) => ErrorKind::Payload,
                Error::ActixParse(_) => ErrorKind::ActixParse,
                Error::UrlParse(_) => ErrorKind::UrlParse,
                Error::QueryPayload(_) => ErrorKind::QueryPayload,
                Error::JsonPayload(_) => ErrorKind::JsonPayload,
                Error::Path(_) => ErrorKind::Path,
                Error::Form(_) => ErrorKind::Form,
                Error::Response(_) => ErrorKind::Response,
                Error::ActixPayload(_) => ErrorKind::ActixPayload,
                Error::Send(_) => ErrorKind::Send,
                Error::Join(_) => ErrorKind::Join,
                Error::Other(_) => ErrorKind::Other,
            }
        }
    }
}

/// Error kind for categorizing errors
///
/// This enum represents different categories of errors that can occur in the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// No error
    Ok,
    
    /// Configuration error
    Config,
    
    /// Database error
    Database,
    
    /// Cache error
    Cache,
    
    /// GraphQL error
    GraphQL,
    
    /// Rate limit error
    RateLimit,
    
    /// Security error
    Security,
    
    /// Validation error
    Validation,
    
    /// Agent error
    Agent,
    
    /// IO error
    Io,
    
    /// Parse error
    Parse,
    
    /// JSON error
    Json,
    
    /// UTF-8 error
    Utf8,
    
    /// From UTF-8 error
    FromUtf8,
    
    /// System time error
    SystemTime,
    
    /// Poison error
    Poison,
    
    /// Parse int error
    ParseInt,
    
    /// Actix error
    Actix,
    
    /// HTTP error
    Http,
    
    /// Blocking error
    Blocking,
    
    /// Payload error
    Payload,
    
    /// Actix parse error
    ActixParse,
    
    /// URL parse error
    UrlParse,
    
    /// Query payload error
    QueryPayload,
    
    /// JSON payload error
    JsonPayload,
    
    /// Path error
    Path,
    
    /// Form error
    Form,
    
    /// Response error
    Response,
    
    /// Actix payload error
    ActixPayload,
    
    /// Send error
    Send,
    
    /// Join error
    Join,
    
    /// Other error
    Other,
}

/// Error builder for creating errors with context
///
/// This struct provides a fluent API for building errors with context.
pub struct ErrorBuilder {
    inner: Error,
    context: Vec<String>,
}

impl ErrorBuilder {
    /// Creates a new error builder
    ///
    /// This method creates a new error builder with the given error.
    pub fn new(inner: Error) -> Self {
        Self {
            inner,
            context: Vec::new(),
        }
    }
    
    /// Adds context to the error
    ///
    /// This method adds contextual information to the error.
    pub fn with_context(mut self, context: &str) -> Self {
        self.context.push(context.to_string());
        self
    }
    
    /// Builds the final error
    ///
    /// This method builds the final error with all accumulated context.
    pub fn build(self) -> Error {
        if self.context.is_empty() {
            self.inner
        } else {
            let context = self.context.join(" -> ");
            Error::Other(format!("{}: {}", context, self.inner))
        }
    }
}

/// Creates a new error builder
///
/// This function creates a new error builder from an existing error.
pub fn error(inner: Error) -> ErrorBuilder {
    ErrorBuilder::new(inner)
}

/// Creates a new error builder from a string
///
/// This function creates a new error builder from a string message.
pub fn error_msg(msg: &str) -> ErrorBuilder {
    ErrorBuilder::new(Error::Other(msg.to_string()))
}
