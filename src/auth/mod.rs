use async_graphql::{Context, Error, Result};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // Subject (user ID)
    pub exp: usize,   // Expiration time
    pub iat: usize,   // Issued at
    pub role: String, // User role
}

pub struct AuthConfig {
    pub secret_key: String,
    pub token_expiration: Duration,
}

pub struct AuthGuard;

impl AuthGuard {
    pub fn new(config: AuthConfig) -> Self {
        Self { config }
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        let validation = Validation::new(Algorithm::HS256);
        let decoding_key = DecodingKey::from_secret(self.config.secret_key.as_bytes());

        decode::<Claims>(token, &decoding_key, &validation)
            .map(|token_data| token_data.claims)
            .map_err(|e| Error::new(format!("Invalid token: {}", e)))
    }

    pub fn create_token(&self, user_id: &str, role: &str) -> Result<String> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let expiration = now + self.config.token_expiration;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration.as_secs() as usize,
            iat: now.as_secs() as usize,
            role: role.to_string(),
        };

        let encoding_key = EncodingKey::from_secret(self.config.secret_key.as_bytes());
        encode(&Header::default(), &claims, &encoding_key)
            .map_err(|e| Error::new(format!("Failed to create token: {}", e)))
    }
}

pub fn get_auth_guard(ctx: &Context<'_>) -> Result<&AuthGuard> {
    ctx.data::<Arc<AuthGuard>>()
        .map_err(|_| Error::new("Auth guard not available"))
}

pub fn get_claims(ctx: &Context<'_>) -> Result<Claims> {
    let auth_guard = get_auth_guard(ctx)?;
    let token = ctx
        .data::<String>()
        .map_err(|_| Error::new("No token provided"))?;

    auth_guard.verify_token(&token)
}
