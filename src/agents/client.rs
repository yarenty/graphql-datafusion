use async_graphql::Error;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tracing::{error, warn};

pub struct AgentClient {
    client: Client,
    api_url: String,
    api_key: String,
    cache: Arc<Mutex<HashMap<String, String>>>, // Cache for SQL translations
    retry_attempts: u32,
    retry_delay: Duration,
}

impl AgentClient {
    pub fn new(
        api_url: String,
        api_key: String,
        retry_attempts: u32,
        retry_delay: Duration,
    ) -> Self {
        Self {
            client: Client::new(),
            api_url,
            api_key,
            cache: Arc::new(Mutex::new(HashMap::new())),
            retry_attempts,
            retry_delay,
        }
    }

    pub async fn translate_to_sql(&self, input: &str) -> Result<String, Error> {
        // Check cache first
        let cache = self.cache.lock().await;
        if let Some(sql) = cache.get(input) {
            return Ok(sql.clone());
        }
        drop(cache);

        // Try with retries
        for attempt in 0..self.retry_attempts {
            match self.attempt_translate(input).await {
                Ok(sql) => {
                    // Cache the result
                    let mut cache = self.cache.lock().await;
                    cache.insert(input.to_string(), sql.clone());
                    return Ok(sql);
                }
                Err(e) => {
                    if attempt == self.retry_attempts - 1 {
                        return Err(e);
                    }
                    warn!("Attempt {} failed: {:?}. Retrying...", attempt + 1, e);
                    tokio::time::sleep(self.retry_delay).await;
                }
            }
        }

        Err(Error::new("All retries failed"))
    }

    async fn attempt_translate(&self, input: &str) -> Result<String, Error> {
        let response = self
            .client
            .post(&self.api_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({ "query": input }))
            .send()
            .await
            .map_err(|e| Error::new(format!("Agent API error: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::new(format!(
                "API error: {}",
                response.status().as_u16()
            )));
        }

        let json: Value = response
            .json()
            .await
            .map_err(|e| Error::new(format!("JSON error: {}", e)))?;

        if let Some(error) = json.get("error") {
            return Err(Error::new(format!(
                "API error: {}",
                error.as_str().unwrap_or("Unknown error")
            )));
        }

        Ok(json["sql"]
            .as_str()
            .ok_or_else(|| Error::new("Invalid response format"))?
            .to_string())
    }

    pub async fn generate_insights(&self, data: Vec<Record>) -> Result<String, Error> {
        for attempt in 0..self.retry_attempts {
            match self.attempt_generate_insights(data.clone()).await {
                Ok(insights) => return Ok(insights),
                Err(e) => {
                    if attempt == self.retry_attempts - 1 {
                        return Err(e);
                    }
                    warn!("Attempt {} failed: {:?}. Retrying...", attempt + 1, e);
                    tokio::time::sleep(self.retry_delay).await;
                }
            }
        }

        Err(Error::new("All retries failed"))
    }

    async fn attempt_generate_insights(&self, data: Vec<Record>) -> Result<String, Error> {
        let response = self
            .client
            .post(&self.api_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({ "data": data }))
            .send()
            .await
            .map_err(|e| Error::new(format!("Agent API error: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::new(format!(
                "API error: {}",
                response.status().as_u16()
            )));
        }

        let json: Value = response
            .json()
            .await
            .map_err(|e| Error::new(format!("JSON error: {}", e)))?;

        if let Some(error) = json.get("error") {
            return Err(Error::new(format!(
                "API error: {}",
                error.as_str().unwrap_or("Unknown error")
            )));
        }

        Ok(json["insights"]
            .as_str()
            .ok_or_else(|| Error::new("Invalid response format"))?
            .to_string())
    }
}
