//! GraphQL Client Example
//! This module demonstrates how to interact with the GraphQL DataFusion API
//! including query execution, WebSocket subscriptions, and JWT authentication.

use async_graphql::Request;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use tokio::time::Duration;
use tracing::{info, error};

/// JWT Claims structure used for token validation
#[derive(Debug, Serialize, Deserialize)]
struct AuthClaims {
    /// Subject (user ID)
    sub: String,
    /// Expiration time (Unix timestamp)
    exp: usize,
    /// Issued at time (Unix timestamp)
    iat: usize,
    /// User role (e.g., "admin", "user")
    role: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct QueryResponse {
    records: Vec<serde_json::Value>,
    insights: String,
}

/// GraphQL client implementation with authentication and WebSocket support
pub struct GraphQLClient {
    /// HTTP client for making requests
    client: Client,
    /// Base URL for GraphQL endpoint
    base_url: String,
    /// JWT token for authentication
    token: String,
}

impl GraphQLClient {
    /// Creates a new GraphQL client instance
    /// 
    /// # Arguments
    /// 
    /// * `token` - JWT token for authentication
    /// 
    /// # Returns
    /// 
    /// * `GraphQLClient` instance
    pub fn new(token: String) -> Self {
        let client = Client::new();
        let base_url = env::var("GRAPHQL_URL").unwrap_or_else(|_| "http://localhost:8000/graphql".to_string());
        
        Self {
            client,
            base_url,
            token,
        }
    }

    /// Executes a GraphQL query with authentication
    /// 
    /// # Arguments
    /// 
    /// * `query` - GraphQL query string
    /// * `variables` - Optional variables for the query
    /// 
    /// # Returns
    /// 
    /// * `Ok(QueryResponse)` - Query results and insights
    /// * `Err` - Error if query fails
    pub async fn execute_query(&self, query: &str, variables: Option<serde_json::Value>) -> Result<QueryResponse, Box<dyn std::error::Error>> {
        // Create GraphQL request with variables
        let request = Request::new(query)
            .variables(variables.unwrap_or_default());

        // Send authenticated request
        let response = self.client
            .post(&self.base_url)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&request)
            .send()
            .await?;

        // Check for HTTP errors
        if !response.status().is_success() {
            error!("GraphQL request failed: {}", response.status());
            return Err(format!("GraphQL request failed: {}", response.status()).into());
        }

        // Parse JSON response
        let body = response.json::<serde_json::Value>().await?;
        
        // Handle GraphQL errors
        if let Some(errors) = body.get("errors") {
            error!("GraphQL errors: {}", errors);
            return Err("GraphQL query failed with errors".into());
        }

        // Extract data from response
        let data = body.get("data").ok_or_else(|| "No data in response".into())?;
        
        // Extract records and insights
        let records = data.get("naturalLanguageQuery")
            .and_then(|q| q.get("records"))
            .ok_or_else(|| "No records in response".into())?;
            
        let insights = data.get("naturalLanguageQuery")
            .and_then(|q| q.get("insights"))
            .and_then(|i| i.as_str())
            .ok_or_else(|| "No insights in response".into())?
            .to_string();

        // Return formatted response
        Ok(QueryResponse {
            records: serde_json::from_value(records.clone())?,
            insights,
        })
    }

    pub async fn subscribe(&self, query: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("ws://localhost:8001/ws/insights/{}", query);
        let (mut ws_stream, _) = tokio_tungstenite::connect_async(&url).await?;

        loop {
            let msg = ws_stream.next().await.unwrap()?;
            if let tokio_tungstenite::tungstenite::Message::Text(text) = msg {
                let insight: serde_json::Value = serde_json::from_str(&text)?;
                info!("Received insight: {}", insight);
            }
        }
    }

    pub async fn validate_token(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
        let decoding_key = jsonwebtoken::DecodingKey::from_secret(
            env::var("JWT_SECRET")?.as_bytes()
        );

        let token_data = jsonwebtoken::decode::<AuthClaims>(&self.token, &decoding_key, &validation)?;
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)?
            .as_secs() as usize;

        Ok(token_data.claims.exp > now)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let token = env::var("JWT_TOKEN")?;
    let client = GraphQLClient::new(token);

    // Validate token first
    if !client.validate_token().await? {
        error!("Invalid or expired token");
        return Err("Invalid token".into());
    }

    // Execute a query
    let query = r#"
        query Analysis {
            naturalLanguageQuery(
                input: "Show monthly sales by category"
                agentType: "sales-agent"
                config: {
                    aggregation: {
                        function: "sum"
                        field: "sales_amount"
                        groupBy: ["category"]
                        timePeriod: "month"
                    }
                }
            ) {
                records {
                    category
                    month
                    total_sales
                }
                insights
            }
        }
    "#;

    let result = client.execute_query(query, None).await?;
    info!("Query result: {}", serde_json::to_string_pretty(&result)?);

    // Start subscription
    let subscription_query = "Show monthly sales by category";
    client.subscribe(subscription_query).await?;

    Ok(())
}
