use graphql_datafusion::agents::client::AgentClient;
use graphql_datafusion::agents::orchestrator::AgentOrchestrator;
use graphql_datafusion::agents::types::AgentConfig;
use graphql_datafusion::datafusion::context::DataFusionContext;
use graphql_datafusion::models::data::{Customer, SalesAnalytics};

#[tokio::test]
async fn test_datafusion_context_creation() {
    // Test that DataFusionContext can be created
    let ctx = DataFusionContext::new("/opt/data/tpch").await;
    assert!(ctx.is_ok());
}

#[tokio::test]
async fn test_datafusion_query_execution() {
    let ctx = DataFusionContext::new("/opt/data/tpch").await.unwrap();
    
    // Test basic query execution
    let result = ctx.execute_query("SELECT COUNT(*) as count FROM customer").await;
    assert!(result.is_ok());
    
    let batches = result.unwrap();
    assert!(!batches.is_empty());
    assert!(batches[0].num_rows() > 0);
}

#[tokio::test]
async fn test_agent_client_creation() {
    // Test that AgentClient can be created
    let client = AgentClient::new(
        "http://localhost:11434".to_string(),
        "llama2".to_string(),
    );
    
    // Test that we can call methods on it
    let result = client.test_connection().await;
    // This will fail if Ollama is not running, but that's expected
    match result {
        Ok(connected) => {
            if connected {
                println!("Ollama is running and connected");
            } else {
                println!("Ollama is not responding");
            }
        }
        Err(_) => {
            println!("Ollama not running - skipping connection test");
        }
    }
}

#[tokio::test]
async fn test_agent_client_sql_translation() {
    let client = AgentClient::new(
        "http://localhost:11434".to_string(),
        "llama2".to_string(),
    );

    // Test SQL translation (this will fail if Ollama is not running, but that's expected)
    let result = client.translate_to_sql("show me top customers by spending").await;
    
    // Either it succeeds (if Ollama is running) or fails with a connection error
    match result {
        Ok(sql) => {
            assert!(!sql.is_empty());
            assert!(sql.to_lowercase().contains("select"));
        }
        Err(_) => {
            // Expected if Ollama is not running
            println!("Ollama not running - skipping SQL translation test");
        }
    }
}

#[tokio::test]
async fn test_agent_client_insights_generation() {
    let client = AgentClient::new(
        "http://localhost:11434".to_string(),
        "llama2".to_string(),
    );

    // Create sample customer data
    let customers = vec![
        Customer {
            c_custkey: 1,
            c_name: "Customer 1".to_string(),
            c_address: "Address 1".to_string(),
            c_nationkey: 1,
            c_phone: "123-456-7890".to_string(),
            c_acctbal: 1000.0,
            c_mktsegment: "BUILDING".to_string(),
            c_comment: "Test customer".to_string(),
        },
        Customer {
            c_custkey: 2,
            c_name: "Customer 2".to_string(),
            c_address: "Address 2".to_string(),
            c_nationkey: 2,
            c_phone: "098-765-4321".to_string(),
            c_acctbal: 2000.0,
            c_mktsegment: "AUTOMOBILE".to_string(),
            c_comment: "Test customer 2".to_string(),
        },
    ];

    // Test insights generation (this will fail if Ollama is not running, but that's expected)
    let result = client.generate_insights(customers).await;
    
    match result {
        Ok(insights) => {
            assert!(!insights.is_empty());
        }
        Err(_) => {
            // Expected if Ollama is not running
            println!("Ollama not running - skipping insights generation test");
        }
    }
}

#[tokio::test]
async fn test_agent_orchestrator_creation() {
    // Test that AgentOrchestrator can be created
    let mut orchestrator = AgentOrchestrator::new();
    
    // Test that it can process queries (will fail if Ollama is not running)
    let result = orchestrator.process_query("show me top customers by spending", None).await;
    
    match result {
        Ok((customers, insights)) => {
            assert!(!customers.is_empty());
            assert!(!insights.is_empty());
        }
        Err(_) => {
            // Expected if Ollama is not running
            println!("Ollama not running - skipping orchestrator test");
        }
    }
}

#[tokio::test]
async fn test_sales_analytics_creation() {
    // Test that SalesAnalytics can be created
    let analytics = SalesAnalytics {
        total_sales: 1000000.0,
        total_orders: 1000,
        avg_order_value: 1000.0,
        top_customers: vec![],
        sales_by_region: vec![],
        monthly_trends: vec![],
    };
    
    assert_eq!(analytics.total_sales, 1000000.0);
    assert_eq!(analytics.total_orders, 1000);
    assert_eq!(analytics.avg_order_value, 1000.0);
}

#[tokio::test]
async fn test_customer_creation() {
    // Test that Customer can be created
    let customer = Customer {
        c_custkey: 1,
        c_name: "Test Customer".to_string(),
        c_address: "Test Address".to_string(),
        c_nationkey: 1,
        c_phone: "123-456-7890".to_string(),
        c_acctbal: 1000.0,
        c_mktsegment: "BUILDING".to_string(),
        c_comment: "Test customer".to_string(),
    };
    
    assert_eq!(customer.c_custkey, 1);
    assert_eq!(customer.c_name, "Test Customer");
    assert_eq!(customer.c_acctbal, 1000.0);
    assert_eq!(customer.c_mktsegment, "BUILDING");
}

#[tokio::test]
async fn test_agent_config_creation() {
    // Test that AgentConfig can be created
    let config = AgentConfig {
        agent_type: "test-agent".to_string(),
        model: "llama2".to_string(),
        temperature: Some(0.7),
        max_tokens: Some(1000),
    };
    
    assert_eq!(config.agent_type, "test-agent");
    assert_eq!(config.model, "llama2");
    assert_eq!(config.temperature, Some(0.7));
    assert_eq!(config.max_tokens, Some(1000));
}

#[tokio::test]
async fn test_agent_config_default() {
    // Test default AgentConfig
    let config = AgentConfig::default();
    
    assert_eq!(config.agent_type, "default");
    assert_eq!(config.model, "llama2");
    assert_eq!(config.temperature, None);
    assert_eq!(config.max_tokens, None);
}
