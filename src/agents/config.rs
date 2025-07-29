use std::env;
use thiserror::Error;

#[derive(Debug)]
pub struct AgentConfig {
    pub api_url: String,
    pub api_key: String,
    pub retry_attempts: u32,
    pub retry_delay_ms: u64,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing required environment variable: {0}")]
    MissingEnvVar(String),
    #[error("Invalid configuration value: {0}")]
    InvalidValue(String),
}

impl AgentConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let api_url =
            env::var("AGENT_API_URL").unwrap_or_else(|_| "https://api.x.ai/grok".to_string());

        let api_key = env::var("AGENT_API_KEY")
            .map_err(|_| ConfigError::MissingEnvVar("AGENT_API_KEY".to_string()))?;

        let retry_attempts = env::var("AGENT_RETRY_ATTEMPTS")
            .unwrap_or_else(|_| "3".to_string())
            .parse()
            .map_err(|_| ConfigError::InvalidValue("AGENT_RETRY_ATTEMPTS".to_string()))?;

        let retry_delay_ms = env::var("AGENT_RETRY_DELAY_MS")
            .unwrap_or_else(|_| "1000".to_string())
            .parse()
            .map_err(|_| ConfigError::InvalidValue("AGENT_RETRY_DELAY_MS".to_string()))?;

        Ok(Self {
            api_url,
            api_key,
            retry_attempts,
            retry_delay_ms,
        })
    }
}
