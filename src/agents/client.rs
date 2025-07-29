//! Agent client for Ollama integration

use crate::agents::types::{OllamaRequest, OllamaResponse, OllamaOptions};
use crate::models::data::Record;
use async_graphql::Error;
use reqwest::Client;
use serde_json::Value;
use tracing::{error, info, warn};

/// Agent client for interacting with Ollama
#[derive(Debug)]
pub struct AgentClient {
    client: Client,
    ollama_url: String,
    model: String,
    options: OllamaOptions,
}

impl AgentClient {
    /// Create a new agent client
    pub fn new(ollama_url: String, model: String) -> Self {
        Self {
            client: Client::new(),
            ollama_url,
            model,
            options: OllamaOptions::default(),
        }
    }

    /// Create a new agent client with custom options
    pub fn with_options(ollama_url: String, model: String, options: OllamaOptions) -> Self {
        Self {
            client: Client::new(),
            ollama_url,
            model,
            options,
        }
    }

    /// Translate natural language query to SQL
    pub async fn translate_to_sql(&self, input: &str) -> Result<String, Error> {
        let prompt = format!(
            "You are a SQL expert. Convert the following natural language query to SQL. \
            Only return the SQL query, nothing else. \
            Available table: sample(id, name, value) \
            Query: {}",
            input
        );

        let response = self.call_ollama(&prompt).await?;
        
        // Clean up the response to extract just the SQL
        let sql = response.trim().to_string();
        
        // Basic validation - ensure it looks like SQL
        if !sql.to_uppercase().contains("SELECT") {
            return Err(Error::new("Generated response doesn't look like SQL"));
        }

        info!("Generated SQL: {}", sql);
        Ok(sql)
    }

    /// Generate insights from data
    pub async fn generate_insights(&self, data: Vec<Record>) -> Result<String, Error> {
        if data.is_empty() {
            return Ok("No data available for analysis.".to_string());
        }

        let data_summary = self.summarize_data(&data);
        let prompt = format!(
            "You are a data analyst. Analyze the following data and provide insights. \
            Be concise but insightful. Focus on patterns, trends, and actionable insights. \
            Data: {}",
            data_summary
        );

        let response = self.call_ollama(&prompt).await?;
        info!("Generated insights: {}", response);
        Ok(response)
    }

    /// Call Ollama API
    async fn call_ollama(&self, prompt: &str) -> Result<String, Error> {
        let request = OllamaRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            stream: false,
            options: Some(self.options.clone()),
        };

        let url = format!("{}/api/generate", self.ollama_url);
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                error!("Ollama API request failed: {}", e);
                Error::new(format!("Failed to call Ollama API: {}", e))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            error!("Ollama API returned error status {}: {}", status, error_text);
            return Err(Error::new(format!("Ollama API error: {} - {}", status, error_text)));
        }

        let ollama_response: OllamaResponse = response.json().await.map_err(|e| {
            error!("Failed to parse Ollama response: {}", e);
            Error::new(format!("Failed to parse Ollama response: {}", e))
        })?;

        Ok(ollama_response.response)
    }

    /// Summarize data for analysis
    fn summarize_data(&self, data: &[Record]) -> String {
        if data.is_empty() {
            return "No data available".to_string();
        }

        let count = data.len();
        let total_value: f64 = data.iter().map(|r| r.value).sum();
        let avg_value = total_value / count as f64;
        let min_value = data.iter().map(|r| r.value).fold(f64::INFINITY, f64::min);
        let max_value = data.iter().map(|r| r.value).fold(f64::NEG_INFINITY, f64::max);

        format!(
            "Dataset contains {} records. Values range from {:.2} to {:.2} with average {:.2}. Total sum: {:.2}",
            count, min_value, max_value, avg_value, total_value
        )
    }

    /// Test connection to Ollama
    pub async fn test_connection(&self) -> Result<bool, Error> {
        let url = format!("{}/api/tags", self.ollama_url);
        
        match self.client.get(&url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    info!("Ollama connection test successful");
                    Ok(true)
                } else {
                    warn!("Ollama connection test failed with status: {}", response.status());
                    Ok(false)
                }
            }
            Err(e) => {
                error!("Ollama connection test failed: {}", e);
                Err(Error::new(format!("Failed to connect to Ollama: {}", e)))
            }
        }
    }
}
