//! AI Integration Example
//! 
//! This example demonstrates how to use the AI-powered features of the GraphQL DataFusion API
//! including natural language queries and automated insights generation.
//! 
//! The system integrates with Ollama (local LLM) to provide:
//! - Natural language to SQL translation
//! - Automated data insights and analysis
//! - Business intelligence recommendations
//! 
//! This example shows how to:
//! 1. Execute natural language queries
//! 2. Generate AI-powered insights
//! 3. Get agent status and recommendations
//! 4. Handle AI-generated SQL queries

use reqwest::Client;
use serde_json::json;
use std::time::Instant;

/// AI-powered GraphQL client for TPCH data analysis
pub struct AIClient {
    client: Client,
    base_url: String,
}

impl AIClient {
    /// Creates a new AI client instance
    pub fn new() -> Self {
        let client = Client::new();
        let base_url = std::env::var("GRAPHQL_URL")
            .unwrap_or_else(|_| "http://localhost:8080/graphql".to_string());
        
        Self { client, base_url }
    }

    /// Executes a GraphQL query and returns the JSON response
    async fn execute_query(&self, query: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let request_body = json!({
            "query": query
        });

        let response = self.client
            .post(&self.base_url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("HTTP error: {}", response.status()).into());
        }

        let body: serde_json::Value = response.json().await?;
        
        // Check for GraphQL errors
        if let Some(errors) = body.get("errors") {
            return Err(format!("GraphQL errors: {}", errors).into());
        }

        Ok(body)
    }

    /// Example 1: Natural language query translation
    pub async fn natural_language_query(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== Example 1: Natural Language Query ===");
        
        let query = r#"
            query {
                naturalLanguageQuery(input: "show me top customers by spending")
            }
        "#;

        let start = Instant::now();
        let result = self.execute_query(query).await?;
        let duration = start.elapsed();

        println!("Response time: {:?}", duration);
        println!("AI Generated SQL:");
        println!("{}", result["data"]["naturalLanguageQuery"]);
        println!();
        
        Ok(())
    }

    /// Example 2: AI-powered insights generation
    pub async fn generate_insights(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== Example 2: AI Insights Generation ===");
        
        let query = r#"
            query {
                insights(input: "analyze customer spending patterns and market segments")
            }
        "#;

        let start = Instant::now();
        let result = self.execute_query(query).await?;
        let duration = start.elapsed();

        println!("Response time: {:?}", duration);
        println!("AI Generated Insights:");
        println!("{}", result["data"]["insights"]);
        println!();
        
        Ok(())
    }

    /// Example 3: Agent status check
    pub async fn check_agent_status(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== Example 3: Agent Status ===");
        
        let query = r#"
            query {
                agentStatus
            }
        "#;

        let start = Instant::now();
        let result = self.execute_query(query).await?;
        let duration = start.elapsed();

        println!("Response time: {:?}", duration);
        println!("Agent Status: {}", result["data"]["agentStatus"]);
        println!();
        
        Ok(())
    }

    /// Example 4: Business intelligence analysis
    pub async fn business_intelligence(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== Example 4: Business Intelligence ===");
        
        // Multiple AI queries for comprehensive analysis
        let queries = vec![
            "analyze sales performance by region",
            "identify top performing market segments",
            "find seasonal trends in order patterns",
            "recommend customer retention strategies"
        ];

        for (i, query_text) in queries.iter().enumerate() {
            println!("Analysis {}: {}", i + 1, query_text);
            
            let query = format!(r#"
                query {{
                    insights(input: "{}")
                }}
            "#, query_text);

            let start = Instant::now();
            let result = self.execute_query(&query).await?;
            let duration = start.elapsed();

            println!("Response time: {:?}", duration);
            println!("Insights: {}", result["data"]["insights"]);
            println!();
        }
        
        Ok(())
    }

    /// Example 5: Interactive data exploration
    pub async fn interactive_exploration(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== Example 5: Interactive Data Exploration ===");
        
        // Simulate a conversation with the AI system
        let conversation = vec![
            "What are the main customer segments?",
            "Which segments have the highest average order value?",
            "Show me customers with declining spending patterns",
            "What factors correlate with customer loyalty?"
        ];

        for (i, question) in conversation.iter().enumerate() {
            println!("Question {}: {}", i + 1, question);
            
            let query = format!(r#"
                query {{
                    insights(input: "{}")
                }}
            "#, question);

            let start = Instant::now();
            let result = self.execute_query(&query).await?;
            let duration = start.elapsed();

            println!("Response time: {:?}", duration);
            println!("Answer: {}", result["data"]["insights"]);
            println!();
        }
        
        Ok(())
    }

    /// Example 6: Performance comparison
    pub async fn performance_comparison(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== Example 6: Performance Comparison ===");
        
        // Compare traditional SQL vs AI-generated queries
        let traditional_query = r#"
            query {
                salesAnalytics {
                    totalSales
                    totalOrders
                    avgOrderValue
                }
            }
        "#;

        let ai_query = r#"
            query {
                insights(input: "calculate total sales, order count, and average order value")
            }
        "#;

        // Traditional query
        println!("Traditional SQL Query:");
        let start = Instant::now();
        let traditional_result = self.execute_query(traditional_query).await?;
        let traditional_duration = start.elapsed();
        println!("Response time: {:?}", traditional_duration);
        println!("Result: {}", serde_json::to_string_pretty(&traditional_result["data"]["salesAnalytics"])?);
        println!();

        // AI query
        println!("AI-Generated Query:");
        let start = Instant::now();
        let ai_result = self.execute_query(ai_query).await?;
        let ai_duration = start.elapsed();
        println!("Response time: {:?}", ai_duration);
        println!("Result: {}", ai_result["data"]["insights"]);
        println!();

        println!("Performance Summary:");
        println!("  Traditional SQL: {:?}", traditional_duration);
        println!("  AI-Generated: {:?}", ai_duration);
        println!("  Difference: {:?}", traditional_duration.saturating_sub(ai_duration));
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 TPCH GraphQL DataFusion - AI Integration Example");
    println!("==================================================");
    println!("This example demonstrates AI-powered features including:");
    println!("- Natural language to SQL translation");
    println!("- Automated insights generation");
    println!("- Business intelligence analysis");
    println!("- Interactive data exploration");
    println!();
    println!("Make sure the server is running on http://localhost:8080");
    println!("and Ollama is available for AI features");
    println!();

    let client = AIClient::new();

    // Run all examples
    client.check_agent_status().await?;
    client.natural_language_query().await?;
    client.generate_insights().await?;
    client.business_intelligence().await?;
    client.interactive_exploration().await?;
    client.performance_comparison().await?;

    println!("✅ All AI integration examples completed successfully!");
    println!("\n💡 Try these natural language queries in the GraphQL Playground:");
    println!("   - 'show me top customers by spending'");
    println!("   - 'analyze sales trends by region'");
    println!("   - 'find customers with high account balances'");
    
    Ok(())
} 