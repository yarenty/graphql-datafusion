use crate::agents::client::AgentClient;
use crate::agents::orchestrator::AgentOrchestrator;
use crate::agents::types::{AgentConfig, Filter};
use crate::datafusion::context::DataFusionContext;
use crate::graphql::helpers::{apply_filters, parse_insights};
use crate::graphql::schema::build_schema;
use crate::models::data::Record;
use async_graphql::EmptyMutation;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::Duration;

#[tokio::test]
async fn test_query_execution() {
    let ctx = DataFusionContext::new().await.unwrap();
    let batches = ctx.execute_query("SELECT * FROM sample").await.unwrap();
    assert!(!batches.is_empty());
    assert_eq!(batches[0].num_rows(), 100); // Assuming sample data has 100 rows
}

#[tokio::test]
async fn test_agent_client() {
    let client = AgentClient::new(
        "https://api.x.ai/grok".to_string(),
        "test-api-key".to_string(),
    );

    let sql = client
        .translate_to_sql("Show records with value > 100")
        .await
        .unwrap();
    assert_eq!(sql, "SELECT * FROM records WHERE value > 100");

    let insights = client.generate_insights(vec![]).await.unwrap();
    assert!(!insights.is_empty());
}

#[tokio::test]
async fn test_agent_orchestrator() {
    let client = AgentClient::new(
        "https://api.x.ai/grok".to_string(),
        "test-api-key".to_string(),
    );

    let orchestrator = AgentOrchestrator::new(
        HashMap::from([("default".to_string(), Arc::new(client.clone()))]),
        "default".to_string(),
        3,
        Duration::from_secs(1),
    );

    let (records, insights) = orchestrator
        .process_query("Show records with value > 100", None)
        .await
        .unwrap();

    assert!(!records.is_empty());
    assert!(!insights.is_empty());
}

#[tokio::test]
async fn test_schema_building() {
    let ctx = Arc::new(DataFusionContext::new().await.unwrap());
    let client = AgentClient::new(
        "https://api.x.ai/grok".to_string(),
        "test-api-key".to_string(),
    );
    let orchestrator = Arc::new(AgentOrchestrator::new(
        HashMap::from([("default".to_string(), Arc::new(client.clone()))]),
        "default".to_string(),
        3,
        Duration::from_secs(1),
    ));

    let schema = build_schema(ctx, orchestrator);
    assert!(schema.is_valid());
}

#[tokio::test]
async fn test_insight_parsing() {
    let insights_text = "Total sales: $10000\nAverage value: $500\nHighest value: $1000";
    let records = vec![];
    let config = AgentConfig {
        agent_type: "test-agent".to_string(),
        visualization: None,
        filters: None,
    };

    let insights = parse_insights(insights_text.to_string(), &records, &config).unwrap();

    assert_eq!(insights.len(), 3);
    assert_eq!(insights[0].title, "Total sales");
    assert_eq!(insights[0].value.unwrap(), "$10000");
    assert_eq!(insights[1].title, "Average value");
    assert_eq!(insights[1].value.unwrap(), "$500");
    assert_eq!(insights[2].title, "Highest value");
    assert_eq!(insights[2].value.unwrap(), "$1000");
}

#[tokio::test]
async fn test_data_filtering() {
    let records = vec![
        Record {
            id: 1,
            name: "Product A".to_string(),
            value: 100.0,
        },
        Record {
            id: 2,
            name: "Product B".to_string(),
            value: 200.0,
        },
    ];

    let filters = vec![Filter {
        field: "value".to_string(),
        operator: ">".to_string(),
        value: "150".to_string(),
    }];

    let filtered = apply_filters(records, &filters);
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].name, "Product B");
}
