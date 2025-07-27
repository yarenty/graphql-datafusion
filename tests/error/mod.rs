use super::error::{
    Error,
    ErrorBuilder,
    ErrorExt,
    ErrorKind,
    error,
    error_msg,
};

#[cfg(test)]
mod tests {
    use super::*;
    use async_graphql::Error as GraphQLError;
    use actix_web::Error as ActixError;
    use std::io::Error as IoError;
    use std::num::ParseIntError;
    use std::str::FromStr;
    use url::ParseError;
    use serde_json::Error as JsonError;
    use config::ConfigError;
    use tokio::task::JoinError;
    use futures::channel::mpsc::SendError;
    use sqlx::Error as DatabaseError;
    use redis::RedisError;
    use crate::rate_limit::RateLimitError;
    use crate::security::SecurityError;
    use crate::validation::ValidationError;
    use crate::agents::AgentError;
    
    #[test]
    fn test_error_conversions() {
        // Test From<String>
        let err: Error = "test error".into();
        assert_eq!(format!("{}", err), "Other error: test error");
        
        // Test From<&str>
        let err: Error = "test error".into();
        assert_eq!(format!("{}", err), "Other error: test error");
        
        // Test From<anyhow::Error>
        let err: Error = anyhow::anyhow!("test error").into();
        assert_eq!(format!("{}", err), "Other error: test error");
        
        // Test From<ParseIntError>
        let err: Error = "42".parse::<i32>().unwrap_err().into();
        assert_eq!(err.kind(), ErrorKind::ParseInt);
        
        // Test From<ParseError>
        let err: Error = "invalid://url".parse::<Url>().unwrap_err().into();
        assert_eq!(err.kind(), ErrorKind::UrlParse);
        
        // Test From<JsonError>
        let err: Error = serde_json::from_str::<i32>("invalid json").unwrap_err().into();
        assert_eq!(err.kind(), ErrorKind::Json);
        
        // Test From<ConfigError>
        let err: Error = ConfigError::from("invalid config").into();
        assert_eq!(err.kind(), ErrorKind::Config);
        
        // Test From<JoinError>
        let err: Error = JoinError::from_panic(Box::new("panic".into())).into();
        assert_eq!(err.kind(), ErrorKind::Join);
        
        // Test From<SendError>
        let err: Error = SendError("test").into();
        assert_eq!(err.kind(), ErrorKind::Send);
        
        // Test From<DatabaseError>
        let err: Error = DatabaseError::from("database error").into();
        assert_eq!(err.kind(), ErrorKind::Database);
        
        // Test From<RedisError>
        let err: Error = RedisError::from("cache error").into();
        assert_eq!(err.kind(), ErrorKind::Cache);
        
        // Test From<RateLimitError>
        let err: Error = RateLimitError::from("rate limit error").into();
        assert_eq!(err.kind(), ErrorKind::RateLimit);
        
        // Test From<SecurityError>
        let err: Error = SecurityError::from("security error").into();
        assert_eq!(err.kind(), ErrorKind::Security);
        
        // Test From<ValidationError>
        let err: Error = ValidationError::from("validation error").into();
        assert_eq!(err.kind(), ErrorKind::Validation);
        
        // Test From<AgentError>
        let err: Error = AgentError::from("agent error").into();
        assert_eq!(err.kind(), ErrorKind::Agent);
    }
    
    #[test]
    fn test_error_builder() {
        let err = error("test error")
            .with_context("outer context")
            .with_context("inner context")
            .build();
        
        assert_eq!(
            format!("{}", err),
            "Other error: outer context -> inner context: test error"
        );
    }
    
    #[test]
    fn test_error_extensions() {
        let result: Result<(), IoError> = Err(IoError::new(
            std::io::ErrorKind::NotFound,
            "test error",
        ));
        
        // Test into_custom
        let err = result.into_custom();
        assert_eq!(err.kind(), ErrorKind::Io);
        
        // Test with_context
        let err = result.with_context("context");
        assert_eq!(err.kind(), ErrorKind::Io);
        
        // Test into_graphql
        let err = result.into_graphql();
        assert!(matches!(err, GraphQLError { .. }));
        
        // Test into_http
        let err = result.into_http();
        assert!(matches!(err, ActixError { .. }));
    }
    
    #[test]
    fn test_error_kind() {
        let result: Result<(), IoError> = Err(IoError::new(
            std::io::ErrorKind::NotFound,
            "test error",
        ));
        
        assert_eq!(result.kind(), ErrorKind::Io);
        
        let result: Result<(), GraphQLError> = Err(GraphQLError::new("test error"));
        assert_eq!(result.kind(), ErrorKind::GraphQL);
        
        let result: Result<(), ActixError> = Err(ActixError::from("test error"));
        assert_eq!(result.kind(), ErrorKind::Actix);
        
        let result: Result<(), DatabaseError> = Err(DatabaseError::from("database error"));
        assert_eq!(result.kind(), ErrorKind::Database);
        
        let result: Result<(), RedisError> = Err(RedisError::from("cache error"));
        assert_eq!(result.kind(), ErrorKind::Cache);
    }
}
