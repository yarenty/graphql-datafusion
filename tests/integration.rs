use crate::datafusion::context::DataFusionContext;
use datafusion::prelude::*;
use reqwest::Client;
use tokio::time::Duration;
use serde_json::json;

#[tokio::test]
async fn test_insights_query() {
    let client = Client::new();
    let query = r#"
        query GetInsights($input: String!) {
            insights(input: $input) {
                title
                description
                value
                visualization {
                    kind
                    series {
                        name
                        data
                    }
                }
            }
        }
    "#;
    
    let variables = json!({
        "input": "Show records with value > 100"
    });
    
    let res = client
        .post("http://localhost:8000/graphql")
        .json(&json!({
            "query": query,
            "variables": variables
        }))
        .send()
        .await
        .unwrap();
    
    assert!(res.status().is_success());
    let body = res.json::<serde_json::Value>().await.unwrap();
    assert!(body["data"]["insights"].as_array().unwrap().len() > 0);
}

#[tokio::test]
async fn test_subscription() {
    let client = Client::new();
    let query = r#"
        subscription GetUpdates {
            insightsUpdates(query: "Show sales trends") {
                title
                description
                value
            }
        }
    "#;
    
    let res = client
        .post("http://localhost:8000/graphql")
        .json(&json!({
            "query": query
        }))
        .send()
        .await
        .unwrap();
    
    assert!(res.status().is_success());
}

#[tokio::test]
async fn test_websocket_insights() {
    let (mut ws_stream, _) = tokio_tungstenite::connect_async(
        "ws://localhost:8001/ws/insights/sales-trends"
    ).await.unwrap();
    
    // Wait for initial connection
    tokio::time::sleep(Duration::from_secs(1)).await;
    
    // Send a query to trigger updates
    let client = Client::new();
    let res = client
        .post("http://localhost:8000/graphql")
        .json(&json!({
            "query": "query { naturalLanguageQuery(input: \"Show sales trends\") }"
        }))
        .send()
        .await
        .unwrap();
    
    assert!(res.status().is_success());
    
    // Wait for WebSocket message
    let msg = ws_stream.next().await.unwrap().unwrap();
    let insight: serde_json::Value = serde_json::from_str(&msg.to_text().unwrap()).unwrap();
    assert!(insight["title"].as_str().unwrap().contains("sales"));
}

#[tokio::test]
async fn test_websocket_status() {
    let (mut ws_stream, _) = tokio_tungstenite::connect_async(
        "ws://localhost:8001/ws/status/sales-agent"
    ).await.unwrap();
    
    // Wait for initial connection
    tokio::time::sleep(Duration::from_secs(1)).await;
    
    // Send a query to trigger status updates
    let client = Client::new();
    let res = client
        .post("http://localhost:8000/graphql")
        .json(&json!({
            "query": "query { agentStatus(agentType: \"sales-agent\") { status metrics } }"
        }))
        .send()
        .await
        .unwrap();
    
    assert!(res.status().is_success());
    
    // Wait for WebSocket message
    let msg = ws_stream.next().await.unwrap().unwrap();
    let status: serde_json::Value = serde_json::from_str(&msg.to_text().unwrap()).unwrap();
    assert_eq!(status["agent_type"].as_str().unwrap(), "sales-agent");
}

#[tokio::test]
async fn test_query_translator() {
    let client = Client::new();
    let query = r#"
        query TranslateQuery($input: String!) {
            translateQuery(input: $input) {
                sql
                parameters
            }
        }
    "#;
    
    let variables = json!({
        "input": "Show records with value > 100"
    });
    
    let res = client
        .post("http://localhost:8000/graphql")
        .json(&json!({
            "query": query,
            "variables": variables
        }))
        .send()
        .await
        .unwrap();
    
    assert!(res.status().is_success());
    let body = res.json::<serde_json::Value>().await.unwrap();
    assert_eq!(body["data"]["translateQuery"]["sql"].as_str().unwrap(), "SELECT * FROM records WHERE value > 100");
}

#[tokio::test]
async fn test_data_aggregation() {
    let client = Client::new();
    let query = r#"
        query AggregateData($input: String!, $config: AgentConfig!) {
            insights(input: $input, config: $config) {
                title
                description
                value
                visualization {
                    kind
                    series {
                        name
                        data
                    }
                }
            }
        }
    "#;
    
    let variables = json!({
        "input": "Analyze monthly sales",
        "config": {
            "agentType": "sales-agent",
            "visualization": {
                "preferredTypes": ["line"],
                "aggregation": {
                    "timePeriod": "month",
                    "function": "sum",
                    "groupBy": ["category"]
                }
            }
        }
    });
    
    let res = client
        .post("http://localhost:8000/graphql")
        .json(&json!({
            "query": query,
            "variables": variables
        }))
        .send()
        .await
        .unwrap();
    
    assert!(res.status().is_success());
    let body = res.json::<serde_json::Value>().await.unwrap();
    assert_eq!(body["data"]["insights"].as_array().unwrap().len(), 1);
    assert_eq!(body["data"]["insights"][0]["visualization"]["kind"].as_str().unwrap(), "line");
}

#[tokio::test]
async fn test_datafusion_query() {
    let ctx = DataFusionContext::new();
    let df = ctx.ctx.sql("SELECT 1").await.unwrap();
    let results = df.collect().await.unwrap();
    assert_eq!(results.len(), 1);
}
