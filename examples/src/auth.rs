//! Authentication Manager Example
//! This module demonstrates JWT-based authentication with role-based access control

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use tracing::{info, error};

/// JWT Claims structure containing user information and permissions
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    /// Subject (user ID)
    sub: String,
    /// Expiration time (Unix timestamp)
    exp: usize,
    /// Issued at time (Unix timestamp)
    iat: usize,
    /// User role (e.g., "admin", "user")
    role: String,
    /// User permissions (scopes)
    scope: Vec<String>,
}

/// Authentication manager handling JWT operations and permission checks
pub struct AuthManager {
    /// JWT secret key for signing/verification
    secret: String,
    /// JWT issuer claim
    issuer: String,
    /// JWT audience claim
    audience: String,
}

impl AuthManager {
    /// Creates a new authentication manager instance
    /// 
    /// # Returns
    /// 
    /// * `AuthManager` instance
    pub fn new() -> Self {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let issuer = env::var("JWT_ISSUER").unwrap_or_else(|_| "graphql-datafusion".to_string());
        let audience = env::var("JWT_AUDIENCE").unwrap_or_else(|_| "graphql-datafusion".to_string());
        
        Self {
            secret,
            issuer,
            audience,
        }
    }

    /// Creates a new JWT token for a user
    /// 
    /// # Arguments
    /// 
    /// * `user_id` - User identifier
    /// * `role` - User role (e.g., "admin", "user")
    /// * `scope` - User permissions (scopes)
    /// 
    /// # Returns
    /// 
    /// * `Ok(String)` - JWT token
    /// * `Err` - Error if token creation fails
    pub fn create_token(&self, user_id: &str, role: &str, scope: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
        let now = Utc::now();
        let expiration = now + Duration::hours(1);

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration.timestamp() as usize,
            iat: now.timestamp() as usize,
            role: role.to_string(),
            scope,
        };

        let token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )?;

        info!("Created JWT token for user: {}", user_id);
        Ok(token)
    }

    /// Validates a JWT token
    /// 
    /// # Arguments
    /// 
    /// * `token` - JWT token to validate
    /// 
    /// # Returns
    /// 
    /// * `Ok(Claims)` - Validated claims
    /// * `Err` - Error if token is invalid or expired
    pub fn validate_token(&self, token: &str) -> Result<Claims, Box<dyn std::error::Error>> {
        let validation = Validation {
            validate_nbf: false,
            ..Default::default()
        };

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )?;

        // Additional validation
        if token_data.claims.iat > token_data.claims.exp {
            return Err("Invalid token: iat > exp".into());
        }

        if token_data.claims.exp < Utc::now().timestamp() as usize {
            return Err("Token expired".into());
        }

        info!("Validated JWT token for user: {}", token_data.claims.sub);
        Ok(token_data.claims)
    }

    /// Checks if a user has a specific permission
    /// 
    /// # Arguments
    /// 
    /// * `claims` - User claims
    /// * `permission` - Permission to check
    /// 
    /// # Returns
    /// 
    /// * `true` - User has the permission
    /// * `false` - User does not have the permission
    pub fn check_permission(&self, claims: &Claims, permission: &str) -> bool {
        claims.scope.contains(&permission.to_string())
    }

    /// Gets the user's role from claims
    /// 
    /// # Arguments
    /// 
    /// * `claims` - User claims
    /// 
    /// # Returns
    /// 
    /// * `&str` - User role
    pub fn get_role(&self, claims: &Claims) -> &str {
        &claims.role
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let auth = AuthManager::new();

    // Create token example
    let token = auth.create_token(
        "user123",
        "admin",
        vec!["query:read", "query:write", "admin:access"],
    )?;

    info!("Generated token: {}", token);

    // Validate token example
    let claims = auth.validate_token(&token)?;
    info!("Validated claims: {:?}", claims);

    // Check permissions
    let has_read_permission = auth.check_permission(&claims, "query:read");
    info!("Has read permission: {}", has_read_permission);

    let has_write_permission = auth.check_permission(&claims, "query:write");
    info!("Has write permission: {}", has_write_permission);

    Ok(())
}
