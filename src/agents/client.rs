//! Agent client for Ollama integration

use crate::agents::types::{OllamaRequest, OllamaResponse, OllamaOptions};
use crate::models::data::Customer;
use async_graphql::Error;
use reqwest::Client;
use tracing::error;

/// Agent client for interacting with Ollama
#[derive(Debug, Clone)]
pub struct AgentClient {
    client: Client,
    ollama_url: String,
    model: String,
    options: OllamaOptions,
}

impl AgentClient {
    pub fn new(ollama_url: String, model: String) -> Self {
        Self {
            client: Client::new(),
            ollama_url,
            model,
            options: OllamaOptions::default(),
        }
    }

    /// Translate natural language to SQL
    pub async fn translate_to_sql(&self, input: &str) -> Result<String, Error> {
        let prompt = format!(
            "Translate this natural language query to SQL for TPCH database: '{}'. 
            Available tables: customer, orders, lineitem, part, supplier, nation, region, partsupp.
            Return only the SQL query, no explanations.",
            input
        );
        
        self.call_ollama(&prompt).await
    }

    /// Generate insights from data
    pub async fn generate_insights(&self, customers: Vec<Customer>) -> Result<String, Error> {
        if customers.is_empty() {
            return Ok("No data available for analysis.".to_string());
        }

        let summary = self.summarize_customers(&customers);
        let prompt = format!(
            "Analyze this customer data and provide business insights: {}",
            summary
        );
        
        self.call_ollama(&prompt).await
    }

    /// Test connection to Ollama
    pub async fn test_connection(&self) -> Result<bool, Error> {
        let prompt = "Hello, this is a connection test.";
        match self.call_ollama(&prompt).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Generic method to call Ollama
    async fn call_ollama(&self, prompt: &str) -> Result<String, Error> {
        let request = OllamaRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            stream: false,
            options: Some(self.options.clone()),
        };

        let response = self.client
            .post(&format!("{}/api/generate", self.ollama_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                error!("Ollama API error: {}", e);
                Error::new(format!("Failed to call Ollama: {}", e))
            })?;

        if !response.status().is_success() {
            return Err(Error::new(format!(
                "Ollama API returned error status: {}",
                response.status()
            )));
        }

        let ollama_response: OllamaResponse = response.json().await.map_err(|e| {
            error!("Failed to parse Ollama response: {}", e);
            Error::new(format!("Failed to parse response: {}", e))
        })?;

        Ok(ollama_response.response)
    }

    /// Summarize customer data for analysis
    fn summarize_customers(&self, customers: &[Customer]) -> String {
        if customers.is_empty() {
            return "No customer data available".to_string();
        }

        let total_customers = customers.len();
        let total_balance: f64 = customers.iter().map(|c| c.c_acctbal).sum();
        let avg_balance = total_balance / total_customers as f64;
        
        let market_segments: std::collections::HashMap<String, usize> = 
            customers.iter()
                .fold(std::collections::HashMap::new(), |mut acc, c| {
                    *acc.entry(c.c_mktsegment.clone()).or_insert(0) += 1;
                    acc
                });

        format!(
            "Total customers: {}, Average account balance: {:.2}, Market segments: {:?}",
            total_customers, avg_balance, market_segments
        )
    }
}
