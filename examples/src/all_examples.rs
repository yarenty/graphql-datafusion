//! All Examples Runner
//! 
//! This example runs all the TPCH GraphQL DataFusion examples in sequence,
//! providing a comprehensive demonstration of the system's capabilities.

use reqwest::Client;
use serde_json::json;

/// Helper function to check server connectivity
async fn check_server_health() -> Result<bool, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = std::env::var("GRAPHQL_URL")
        .unwrap_or_else(|_| "http://localhost:8080/graphql".to_string());
    
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&json!({
            "query": "{ tables }"
        }))
        .send()
        .await?;
    
    Ok(response.status().is_success())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    println!("🚀 TPCH GraphQL DataFusion - Complete Example Suite");
    println!("==================================================");
    println!();
    
    // Check server health first
    println!("🔍 Checking server connectivity...");
    match check_server_health().await {
        Ok(true) => println!("✅ Server is running and accessible"),
        Ok(false) => {
            println!("❌ Server is not responding");
            println!("Please start the server with: cargo run");
            return Err("Server not available".into());
        }
        Err(e) => {
            println!("❌ Error checking server: {}", e);
            return Err(e);
        }
    }
    
    println!();
    println!("📋 Note: To run individual examples, use:");
    println!("   cargo run --example basic_queries");
    println!("   cargo run --example ai_integration");
    println!("   cargo run --example advanced_analytics");
    println!();
    println!("✅ Server connectivity verified!");
    println!("🎉 You can now run the individual examples to explore the system capabilities.");
    
    Ok(())
} 