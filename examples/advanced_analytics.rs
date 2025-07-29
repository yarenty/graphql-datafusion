//! Advanced Analytics Example
//! 
//! This example demonstrates advanced analytics capabilities of the GraphQL DataFusion API
//! including complex aggregations, business metrics, and data exploration patterns.
//! 
//! The TPCH dataset provides rich business data for advanced analytics:
//! - Customer segmentation and behavior analysis
//! - Sales performance and trend analysis
//! - Supply chain and inventory insights
//! - Regional and market analysis
//! 
//! This example shows how to:
//! 1. Perform complex data aggregations
//! 2. Analyze business metrics and KPIs
//! 3. Explore data relationships and patterns
//! 4. Generate business intelligence reports

use reqwest::Client;
use serde_json::json;
use std::time::Instant;

/// Advanced analytics client for TPCH data
pub struct AnalyticsClient {
    client: Client,
    base_url: String,
}

impl AnalyticsClient {
    /// Creates a new analytics client instance
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

    /// Example 1: Customer segmentation analysis
    pub async fn customer_segmentation(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== Example 1: Customer Segmentation Analysis ===");
        
        let query = r#"
            query {
                salesAnalytics {
                    topCustomers {
                        customer {
                            c_custkey
                            c_name
                            c_mktsegment
                            c_acctbal
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
        
        // Analyze customer segments
        println!("\nCustomer Segmentation Analysis:");
        let mut segments = std::collections::HashMap::new();
        
        for customer in analytics["topCustomers"].as_array().unwrap_or(&vec![]) {
            let segment = customer["customer"]["c_mktsegment"].as_str().unwrap_or("Unknown");
            let spent = customer["totalSpent"].as_f64().unwrap_or(0.0);
            let count = customer["orderCount"].as_i64().unwrap_or(0);
            
            let entry = segments.entry(segment).or_insert((0.0, 0, 0));
            entry.0 += spent;
            entry.1 += count;
            entry.2 += 1;
        }
        
        for (segment, (total_spent, total_orders, customer_count)) in segments {
            let avg_spent = total_spent / customer_count as f64;
            let avg_orders = total_orders as f64 / customer_count as f64;
            
            println!("  {} Segment:", segment);
            println!("    Total Customers: {}", customer_count);
            println!("    Total Revenue: ${:.2}", total_spent);
            println!("    Average Revenue per Customer: ${:.2}", avg_spent);
            println!("    Average Orders per Customer: {:.1}", avg_orders);
            println!();
        }
        
        Ok(())
    }

    /// Example 2: Sales performance analysis
    pub async fn sales_performance(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== Example 2: Sales Performance Analysis ===");
        
        let query = r#"
            query {
                salesAnalytics {
                    totalSales
                    totalOrders
                    avgOrderValue
                    monthlyTrends {
                        month
                        totalSales
                        orderCount
                    }
                }
            }
        "#;

        let start = Instant::now();
        let result = self.execute_query(query).await?;
        let duration = start.elapsed();

        println!("Response time: {:?}", duration);
        let analytics = &result["data"]["salesAnalytics"];
        
        let total_sales = analytics["totalSales"].as_f64().unwrap_or(0.0);
        let total_orders = analytics["totalOrders"].as_i64().unwrap_or(0);
        let avg_order_value = analytics["avgOrderValue"].as_f64().unwrap_or(0.0);
        
        println!("\nSales Performance Metrics:");
        println!("  Total Revenue: ${:.2}", total_sales);
        println!("  Total Orders: {}", total_orders);
        println!("  Average Order Value: ${:.2}", avg_order_value);
        println!("  Revenue per Order: ${:.2}", total_sales / total_orders as f64);
        
        // Analyze monthly trends
        println!("\nMonthly Sales Trends:");
        for trend in analytics["monthlyTrends"].as_array().unwrap_or(&vec![]) {
            let month = trend["month"].as_str().unwrap_or("Unknown");
            let sales = trend["totalSales"].as_f64().unwrap_or(0.0);
            let orders = trend["orderCount"].as_i64().unwrap_or(0);
            let avg = if orders > 0 { sales / orders as f64 } else { 0.0 };
            
            println!("  {}: ${:.2} ({} orders, avg: ${:.2})", month, sales, orders, avg);
        }
        
        Ok(())
    }

    /// Example 3: Regional market analysis
    pub async fn regional_analysis(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== Example 3: Regional Market Analysis ===");
        
        let query = r#"
            query {
                salesAnalytics {
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
        let regions = &result["data"]["salesAnalytics"]["salesByRegion"];
        
        println!("\nRegional Market Analysis:");
        let mut total_revenue = 0.0;
        let mut total_customers = 0;
        
        for region in regions.as_array().unwrap_or(&vec![]) {
            let region_name = region["region"].as_str().unwrap_or("Unknown");
            let sales = region["totalSales"].as_f64().unwrap_or(0.0);
            let customers = region["customerCount"].as_i64().unwrap_or(0);
            
            total_revenue += sales;
            total_customers += customers;
            
            let revenue_per_customer = if customers > 0 { sales / customers as f64 } else { 0.0 };
            let market_share = if total_revenue > 0.0 { (sales / total_revenue) * 100.0 } else { 0.0 };
            
            println!("  {}:", region_name);
            println!("    Revenue: ${:.2}", sales);
            println!("    Customers: {}", customers);
            println!("    Revenue per Customer: ${:.2}", revenue_per_customer);
            println!("    Market Share: {:.1}%", market_share);
            println!();
        }
        
        println!("Total Market:");
        println!("  Total Revenue: ${:.2}", total_revenue);
        println!("  Total Customers: {}", total_customers);
        println!("  Average Revenue per Customer: ${:.2}", total_revenue / total_customers as f64);
        
        Ok(())
    }

    /// Example 4: Business intelligence dashboard
    pub async fn business_intelligence_dashboard(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== Example 4: Business Intelligence Dashboard ===");
        
        // Comprehensive analytics query
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
                            c_acctbal
                        }
                        totalSpent
                        orderCount
                    }
                    salesByRegion {
                        region
                        totalSales
                        customerCount
                    }
                    monthlyTrends {
                        month
                        totalSales
                        orderCount
                    }
                }
            }
        "#;

        let start = Instant::now();
        let result = self.execute_query(query).await?;
        let duration = start.elapsed();

        println!("Response time: {:?}", duration);
        let analytics = &result["data"]["salesAnalytics"];
        
        // Generate comprehensive business report
        println!("\n📊 BUSINESS INTELLIGENCE DASHBOARD");
        println!("==================================");
        
        // Key Performance Indicators
        let total_sales = analytics["totalSales"].as_f64().unwrap_or(0.0);
        let total_orders = analytics["totalOrders"].as_i64().unwrap_or(0);
        let avg_order_value = analytics["avgOrderValue"].as_f64().unwrap_or(0.0);
        
        println!("\n🎯 KEY PERFORMANCE INDICATORS");
        println!("  Total Revenue: ${:,.2}", total_sales);
        println!("  Total Orders: {:,}", total_orders);
        println!("  Average Order Value: ${:.2}", avg_order_value);
        println!("  Revenue per Order: ${:.2}", total_sales / total_orders as f64);
        
        // Top Customers Analysis
        println!("\n👥 TOP CUSTOMERS ANALYSIS");
        let top_customers = analytics["topCustomers"].as_array().unwrap_or(&vec![]);
        for (i, customer) in top_customers.iter().take(5).enumerate() {
            let name = customer["customer"]["c_name"].as_str().unwrap_or("Unknown");
            let segment = customer["customer"]["c_mktsegment"].as_str().unwrap_or("Unknown");
            let spent = customer["totalSpent"].as_f64().unwrap_or(0.0);
            let orders = customer["orderCount"].as_i64().unwrap_or(0);
            
            println!("  {}. {} ({})", i + 1, name, segment);
            println!("     Revenue: ${:.2} | Orders: {}", spent, orders);
        }
        
        // Regional Performance
        println!("\n🌍 REGIONAL PERFORMANCE");
        let regions = analytics["salesByRegion"].as_array().unwrap_or(&vec![]);
        for region in regions {
            let region_name = region["region"].as_str().unwrap_or("Unknown");
            let sales = region["totalSales"].as_f64().unwrap_or(0.0);
            let customers = region["customerCount"].as_i64().unwrap_or(0);
            let market_share = (sales / total_sales) * 100.0;
            
            println!("  {}: ${:,.2} ({:.1}% market share, {} customers)", 
                region_name, sales, market_share, customers);
        }
        
        // Monthly Trends
        println!("\n📈 MONTHLY TRENDS");
        let trends = analytics["monthlyTrends"].as_array().unwrap_or(&vec![]);
        for trend in trends {
            let month = trend["month"].as_str().unwrap_or("Unknown");
            let sales = trend["totalSales"].as_f64().unwrap_or(0.0);
            let orders = trend["orderCount"].as_i64().unwrap_or(0);
            
            println!("  {}: ${:,.2} ({} orders)", month, sales, orders);
        }
        
        // Business Insights
        println!("\n💡 BUSINESS INSIGHTS");
        println!("  • Total market value: ${:,.2}", total_sales);
        println!("  • Average customer value: ${:.2}", total_sales / total_customers as f64);
        println!("  • Order frequency: {:.1} orders per customer", total_orders as f64 / total_customers as f64);
        
        // Find best performing segment
        let mut segment_performance = std::collections::HashMap::new();
        for customer in top_customers {
            let segment = customer["customer"]["c_mktsegment"].as_str().unwrap_or("Unknown");
            let spent = customer["totalSpent"].as_f64().unwrap_or(0.0);
            let entry = segment_performance.entry(segment).or_insert(0.0);
            *entry += spent;
        }
        
        if let Some((best_segment, revenue)) = segment_performance.iter().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()) {
            println!("  • Best performing segment: {} (${:,.2})", best_segment, revenue);
        }
        
        Ok(())
    }

    /// Example 5: Predictive analytics simulation
    pub async fn predictive_analytics(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== Example 5: Predictive Analytics Simulation ===");
        
        // Get historical data for trend analysis
        let query = r#"
            query {
                salesAnalytics {
                    totalSales
                    totalOrders
                    avgOrderValue
                    monthlyTrends {
                        month
                        totalSales
                        orderCount
                    }
                }
            }
        "#;

        let start = Instant::now();
        let result = self.execute_query(query).await?;
        let duration = start.elapsed();

        println!("Response time: {:?}", duration);
        let analytics = &result["data"]["salesAnalytics"];
        
        let trends = analytics["monthlyTrends"].as_array().unwrap_or(&vec![]);
        let total_sales = analytics["totalSales"].as_f64().unwrap_or(0.0);
        
        if trends.len() >= 3 {
            println!("\n📊 PREDICTIVE ANALYTICS");
            println!("======================");
            
            // Calculate growth rates
            let mut sales_data: Vec<f64> = trends.iter()
                .map(|t| t["totalSales"].as_f64().unwrap_or(0.0))
                .collect();
            
            // Calculate month-over-month growth
            let mut growth_rates = Vec::new();
            for i in 1..sales_data.len() {
                let growth = if sales_data[i-1] > 0.0 {
                    ((sales_data[i] - sales_data[i-1]) / sales_data[i-1]) * 100.0
                } else {
                    0.0
                };
                growth_rates.push(growth);
            }
            
            // Calculate average growth rate
            let avg_growth = growth_rates.iter().sum::<f64>() / growth_rates.len() as f64;
            
            println!("Historical Growth Analysis:");
            for (i, growth) in growth_rates.iter().enumerate() {
                println!("  Month {}: {:.1}% growth", i + 2, growth);
            }
            println!("  Average Growth Rate: {:.1}%", avg_growth);
            
            // Simple forecasting
            let last_month_sales = sales_data.last().unwrap_or(&0.0);
            let forecast_3_months = last_month_sales * (1.0 + avg_growth / 100.0).powi(3);
            let forecast_6_months = last_month_sales * (1.0 + avg_growth / 100.0).powi(6);
            
            println!("\n📈 SALES FORECASTING");
            println!("  Current Monthly Sales: ${:,.2}", last_month_sales);
            println!("  3-Month Forecast: ${:,.2}", forecast_3_months);
            println!("  6-Month Forecast: ${:,.2}", forecast_6_months);
            println!("  Annual Growth Projection: {:.1}%", avg_growth * 12.0);
            
            // Market opportunity analysis
            let market_potential = total_sales * 1.2; // Assume 20% growth potential
            let opportunity = market_potential - total_sales;
            
            println!("\n🎯 MARKET OPPORTUNITY ANALYSIS");
            println!("  Current Market Size: ${:,.2}", total_sales);
            println!("  Market Potential: ${:,.2}", market_potential);
            println!("  Growth Opportunity: ${:,.2}", opportunity);
            println!("  Opportunity Percentage: {:.1}%", (opportunity / total_sales) * 100.0);
        } else {
            println!("Insufficient data for predictive analytics (need at least 3 months)");
        }
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 TPCH GraphQL DataFusion - Advanced Analytics Example");
    println!("=======================================================");
    println!("This example demonstrates advanced analytics capabilities:");
    println!("- Customer segmentation and behavior analysis");
    println!("- Sales performance and trend analysis");
    println!("- Regional market analysis");
    println!("- Business intelligence dashboards");
    println!("- Predictive analytics and forecasting");
    println!();
    println!("Make sure the server is running on http://localhost:8080");
    println!();

    let client = AnalyticsClient::new();

    // Run all examples
    client.customer_segmentation().await?;
    client.sales_performance().await?;
    client.regional_analysis().await?;
    client.business_intelligence_dashboard().await?;
    client.predictive_analytics().await?;

    println!("✅ All advanced analytics examples completed successfully!");
    println!("\n💡 Key Insights:");
    println!("   - Customer segmentation reveals market opportunities");
    println!("   - Regional analysis helps with expansion strategies");
    println!("   - Trend analysis enables predictive planning");
    println!("   - Business intelligence drives data-driven decisions");
    
    Ok(())
} 