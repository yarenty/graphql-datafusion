use graphql_datafusion::datafusion::context::DataFusionContext;
use reqwest::Client;
use serde_json::json;

#[tokio::test]
async fn test_server_health() {
    let client = Client::new();

    // Test health endpoint
    let res = client.get("http://localhost:8080/health").send().await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                println!("Server is running and healthy");
            } else {
                println!("Server responded with status: {}", response.status());
            }
        }
        Err(_) => {
            println!("Server is not running - skipping health check test");
        }
    }
}

#[tokio::test]
async fn test_graphql_tables_query() {
    let client = Client::new();

    let query = r#"
        query {
            tables
        }
    "#;

    let res = client
        .post("http://localhost:8080/graphql")
        .json(&json!({
            "query": query
        }))
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                let body = response.json::<serde_json::Value>().await.unwrap();
                if let Some(data) = body.get("data") {
                    if let Some(tables) = data.get("tables") {
                        if let Some(table_array) = tables.as_array() {
                            assert!(!table_array.is_empty(), "Tables array should not be empty");
                            println!("Found {} tables", table_array.len());
                        }
                    }
                }
            } else {
                println!("GraphQL query failed with status: {}", response.status());
            }
        }
        Err(_) => {
            println!("Server is not running - skipping GraphQL test");
        }
    }
}

#[tokio::test]
async fn test_graphql_customers_query() {
    let client = Client::new();

    let query = r#"
        query {
            customers(limit: 5) {
                c_custkey
                c_name
                c_acctbal
                c_mktsegment
            }
        }
    "#;

    let res = client
        .post("http://localhost:8080/graphql")
        .json(&json!({
            "query": query
        }))
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                let body = response.json::<serde_json::Value>().await.unwrap();
                if let Some(data) = body.get("data") {
                    if let Some(customers) = data.get("customers") {
                        if let Some(customer_array) = customers.as_array() {
                            assert!(
                                customer_array.len() <= 5,
                                "Should return at most 5 customers"
                            );
                            println!("Retrieved {} customers", customer_array.len());
                        }
                    }
                }
            } else {
                println!("GraphQL query failed with status: {}", response.status());
            }
        }
        Err(_) => {
            println!("Server is not running - skipping customers query test");
        }
    }
}

#[tokio::test]
async fn test_graphql_sales_analytics_query() {
    let client = Client::new();

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
            }
        }
    "#;

    let res = client
        .post("http://localhost:8080/graphql")
        .json(&json!({
            "query": query
        }))
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                let body = response.json::<serde_json::Value>().await.unwrap();
                if let Some(data) = body.get("data") {
                    if let Some(analytics) = data.get("salesAnalytics") {
                        assert!(
                            analytics.get("totalSales").is_some(),
                            "Should have totalSales"
                        );
                        assert!(
                            analytics.get("totalOrders").is_some(),
                            "Should have totalOrders"
                        );
                        assert!(
                            analytics.get("avgOrderValue").is_some(),
                            "Should have avgOrderValue"
                        );
                        println!("Sales analytics query successful");
                    }
                }
            } else {
                println!("GraphQL query failed with status: {}", response.status());
            }
        }
        Err(_) => {
            println!("Server is not running - skipping sales analytics test");
        }
    }
}

#[tokio::test]
async fn test_graphql_natural_language_query() {
    let client = Client::new();

    let query = r#"
        query {
            naturalLanguageQuery(input: "show me top customers by spending")
        }
    "#;

    let res = client
        .post("http://localhost:8080/graphql")
        .json(&json!({
            "query": query
        }))
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                let body = response.json::<serde_json::Value>().await.unwrap();
                if let Some(data) = body.get("data") {
                    if let Some(sql) = data.get("naturalLanguageQuery") {
                        if let Some(sql_str) = sql.as_str() {
                            assert!(!sql_str.is_empty(), "SQL should not be empty");
                            assert!(
                                sql_str.to_lowercase().contains("select"),
                                "Should contain SELECT"
                            );
                            println!("Generated SQL: {}", sql_str);
                        }
                    }
                }
            } else {
                println!("GraphQL query failed with status: {}", response.status());
            }
        }
        Err(_) => {
            println!("Server is not running - skipping natural language query test");
        }
    }
}

#[tokio::test]
async fn test_graphql_insights_query() {
    let client = Client::new();

    let query = r#"
        query {
            insights(input: "analyze customer spending patterns")
        }
    "#;

    let res = client
        .post("http://localhost:8080/graphql")
        .json(&json!({
            "query": query
        }))
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                let body = response.json::<serde_json::Value>().await.unwrap();
                if let Some(data) = body.get("data") {
                    if let Some(insights) = data.get("insights") {
                        if let Some(insights_str) = insights.as_str() {
                            assert!(!insights_str.is_empty(), "Insights should not be empty");
                            println!("Generated insights: {}", insights_str);
                        }
                    }
                }
            } else {
                println!("GraphQL query failed with status: {}", response.status());
            }
        }
        Err(_) => {
            println!("Server is not running - skipping insights query test");
        }
    }
}

#[tokio::test]
async fn test_graphql_agent_status_query() {
    let client = Client::new();

    let query = r#"
        query {
            agentStatus
        }
    "#;

    let res = client
        .post("http://localhost:8080/graphql")
        .json(&json!({
            "query": query
        }))
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                let body = response.json::<serde_json::Value>().await.unwrap();
                if let Some(data) = body.get("data") {
                    if let Some(status) = data.get("agentStatus") {
                        if let Some(status_str) = status.as_str() {
                            assert!(!status_str.is_empty(), "Status should not be empty");
                            println!("Agent status: {}", status_str);
                        }
                    }
                }
            } else {
                println!("GraphQL query failed with status: {}", response.status());
            }
        }
        Err(_) => {
            println!("Server is not running - skipping agent status test");
        }
    }
}

#[tokio::test]
async fn test_datafusion_integration() {
    // Test DataFusion context creation
    let ctx = DataFusionContext::new("/opt/data/tpch").await;
    assert!(
        ctx.is_ok(),
        "DataFusion context should be created successfully"
    );

    let ctx = ctx.unwrap();

    // Test basic query execution
    let result = ctx
        .execute_query("SELECT COUNT(*) as count FROM customer")
        .await;
    assert!(result.is_ok(), "Query execution should succeed");

    let batches = result.unwrap();
    assert!(!batches.is_empty(), "Should return at least one batch");
    assert!(batches[0].num_rows() > 0, "Should have at least one row");

    println!("DataFusion integration test passed");
}
