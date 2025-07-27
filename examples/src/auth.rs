use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use tracing::{info, error};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,       // Subject (user ID)
    exp: usize,        // Expiration time
    iat: usize,        // Issued at
    role: String,      // User role
    scope: Vec<String>, // User permissions
}

pub struct AuthManager {
    secret: String,
    issuer: String,
    audience: String,
}

impl AuthManager {
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

    pub fn check_permission(&self, claims: &Claims, permission: &str) -> bool {
        claims.scope.contains(&permission.to_string())
    }

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
