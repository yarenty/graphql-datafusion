//! Basic GraphQL Queries Example
//! 
//! This example demonstrates how to interact with the GraphQL DataFusion API
//! using basic queries against the TPCH (Transaction Processing Performance Council)
//! benchmark dataset.
//! 
//! The TPCH dataset contains realistic business data including:
//! - Customers with account balances and market segments
//! - Orders with total prices and order dates
//! - Line items with quantities and extended prices
//! - Parts, suppliers, nations, and regions
//! 
//! This example shows how to:
//! 1. Query basic customer data
//! 2. Get order information
//! 3. Retrieve table metadata
//! 4. Execute simple aggregations

use reqwest::Client;
use serde_json::json;
use std::time::Instant;

/// GraphQL client for TPCH data queries
pub struct TPCHClient {
    client: Client,
    base_url: String,
}

impl TPCHClient {
    /// Creates a new TPCH client instance
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

    /// Example 1: Get available tables in the TPCH dataset
    pub async fn get_available_tables(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== Example 1: Available Tables ===");
        
        let query = r#"
            query {
                tables
            }
        "#;

        let start = Instant::now();
        let result = self.execute_query(query).await?;
        let duration = start.elapsed();

        println!("Response time: {:?}", duration);
        println!("Available tables: {}", serde_json::to_string_pretty(&result["data"]["tables"])?);
        println!();
        
        Ok(())
    }

    /// Example 2: Get customer data with pagination
    pub async fn get_customers(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== Example 2: Customer Data ===");
        
        let query = r#"
            query {
                customers(limit: 5) {
                    c_custkey
                    c_name
                    c_address
                    c_acctbal
                    c_mktsegment
                }
            }
        "#;

        let start = Instant::now();
        let result = self.execute_query(query).await?;
        let duration = start.elapsed();

        println!("Response time: {:?}", duration);
        println!("Customer data: {}", serde_json::to_string_pretty(&result["data"]["customers"])?);
        println!();
        
        Ok(())
    }

    /// Example 3: Get order data
    pub async fn get_orders(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== Example 3: Order Data ===");
        
        let query = r#"
            query {
                orders(limit: 3) {
                    o_orderkey
                    o_custkey
                    o_orderstatus
                    o_totalprice
                    o_orderdate
                }
            }
        "#;

        let start = Instant::now();
        let result = self.execute_query(query).await?;
        let duration = start.elapsed();

        println!("Response time: {:?}", duration);
        println!("Order data: {}", serde_json::to_string_pretty(&result["data"]["orders"])?);
        println!();
        
        Ok(())
    }

    /// Example 4: Get table row counts
    pub async fn get_table_counts(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== Example 4: Table Row Counts ===");
        
        let query = r#"
            query {
                customerCount: tableCount(tableName: "customer")
                orderCount: tableCount(tableName: "orders")
                lineItemCount: tableCount(tableName: "lineitem")
                partCount: tableCount(tableName: "part")
            }
        "#;

        let start = Instant::now();
        let result = self.execute_query(query).await?;
        let duration = start.elapsed();

        println!("Response time: {:?}", duration);
        println!("Table counts:");
        println!("  Customers: {}", result["data"]["customerCount"]);
        println!("  Orders: {}", result["data"]["orderCount"]);
        println!("  Line Items: {}", result["data"]["lineItemCount"]);
        println!("  Parts: {}", result["data"]["partCount"]);
        println!();
        
        Ok(())
    }

    /// Example 5: Get sales analytics
    pub async fn get_sales_analytics(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== Example 5: Sales Analytics ===");
        
        let query = r#"
            query {
                salesAnalytics {
                    totalSales
                    totalOrders
                    avgOrderValue
                    topCustomers {
                        customer {
                            c_name
                            c_mktsegment
                        }
                        totalSpent
                        orderCount
                    }
                    salesByRegion {
                        region
                        totalSales
                        customerCount
                    }
                }
            }
        "#;

        let start = Instant::now();
        let result = self.execute_query(query).await?;
        let duration = start.elapsed();

        println!("Response time: {:?}", duration);
        let analytics = &result["data"]["salesAnalytics"];
        
        println!("Sales Analytics:");
        println!("  Total Sales: ${:.2}", analytics["totalSales"].as_f64().unwrap_or(0.0));
        println!("  Total Orders: {}", analytics["totalOrders"]);
        println!("  Average Order Value: ${:.2}", analytics["avgOrderValue"].as_f64().unwrap_or(0.0));
        
        println!("\nTop Customers:");
        for customer in analytics["topCustomers"].as_array().unwrap_or(&vec![]) {
            let cust = &customer["customer"];
            println!("  {} ({}): ${:.2}", 
                cust["c_name"], 
                cust["c_mktsegment"], 
                customer["totalSpent"].as_f64().unwrap_or(0.0));
        }
        
        println!("\nSales by Region:");
        for region in analytics["salesByRegion"].as_array().unwrap_or(&vec![]) {
            println!("  {}: ${:.2} ({} customers)", 
                region["region"], 
                region["totalSales"].as_f64().unwrap_or(0.0),
                region["customerCount"]);
        }
        println!();
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 TPCH GraphQL DataFusion - Basic Queries Example");
    println!("==================================================");
    println!("This example demonstrates basic queries against the TPCH dataset.");
    println!("Make sure the server is running on http://localhost:8080");
    println!();

    let client = TPCHClient::new();

    // Run all examples
    client.get_available_tables().await?;
    client.get_customers().await?;
    client.get_orders().await?;
    client.get_table_counts().await?;
    client.get_sales_analytics().await?;

    println!("✅ All examples completed successfully!");
    println!("\n💡 Try these queries in the GraphQL Playground at http://localhost:8080/playground");
    
    Ok(())
} 