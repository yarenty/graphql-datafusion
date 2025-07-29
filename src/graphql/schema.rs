//! GraphQL schema for DataFusion integration

use async_graphql::{Context, Object, Schema, SimpleObject};
use std::sync::Arc;
use crate::datafusion::context::DataFusionContext;
use crate::models::data::{Record, QueryParams, QueryResult};
use crate::agents::types::{Insight, AgentConfig, AgentStatus};
use crate::agents::orchestrator::AgentOrchestrator;

/// Query root for GraphQL
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get records from the database
    async fn records(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<Record>, async_graphql::Error> {
        let df_ctx = ctx.data_unchecked::<Arc<DataFusionContext>>();
        
        let limit_clause = limit.map(|l| format!(" LIMIT {}", l)).unwrap_or_default();
        let offset_clause = offset.map(|o| format!(" OFFSET {}", o)).unwrap_or_default();
        
        let query = format!("SELECT * FROM sample{}{}", limit_clause, offset_clause);
        
        let batches = df_ctx.execute_query(&query).await
            .map_err(|e| async_graphql::Error::new(format!("DataFusion error: {}", e)))?;
        
        let records = convert_batches_to_records(batches)?;
        Ok(records)
    }

    /// Get records with query parameters
    async fn records_with_params(
        &self,
        ctx: &Context<'_>,
        params: QueryParams,
    ) -> Result<QueryResult, async_graphql::Error> {
        let df_ctx = ctx.data_unchecked::<Arc<DataFusionContext>>();
        
        // Build query from parameters
        let mut query = "SELECT * FROM sample".to_string();
        
        if let Some(filters) = &params.filters {
            if !filters.is_empty() {
                query.push_str(" WHERE ");
                let conditions: Vec<String> = filters.iter()
                    .map(|f| format!("{} {} '{}'", f.field, f.operator.to_string(), f.value))
                    .collect();
                query.push_str(&conditions.join(" AND "));
            }
        }
        
        if let Some(sort_by) = &params.sort_by {
            query.push_str(&format!(" ORDER BY {}", sort_by));
            if let Some(sort_order) = &params.sort_order {
                query.push_str(&format!(" {}", sort_order.to_string()));
            }
        }
        
        if let Some(limit) = params.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }
        
        if let Some(offset) = params.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }
        
        let start_time = std::time::Instant::now();
        let batches = df_ctx.execute_query(&query).await
            .map_err(|e| async_graphql::Error::new(format!("DataFusion error: {}", e)))?;
        let query_time = start_time.elapsed().as_millis() as u64;
        
        let records = convert_batches_to_records(batches)?;
        let total_count = records.len() as i64;
        
        Ok(QueryResult {
            records,
            total_count,
            has_more: false, // TODO: Implement proper pagination
            query_time_ms: query_time,
        })
    }

    /// Natural language query with AI agent
    async fn natural_language_query(
        &self,
        ctx: &Context<'_>,
        input: String,
        agent_type: Option<String>,
    ) -> Result<Vec<Record>, async_graphql::Error> {
        let orchestrator = ctx.data_unchecked::<Arc<AgentOrchestrator>>();
        
        // For now, return mock data since we can't mutate the Arc
        let records = vec![
            Record { id: 1, name: "Sample 1".to_string(), value: 100.0 },
            Record { id: 2, name: "Sample 2".to_string(), value: 200.0 },
            Record { id: 3, name: "Sample 3".to_string(), value: 150.0 },
        ];
        Ok(records)
    }

    /// Generate insights from data using AI agent
    async fn insights(
        &self,
        ctx: &Context<'_>,
        input: String,
        config: Option<AgentConfig>,
    ) -> Result<Vec<Insight>, async_graphql::Error> {
        // For now, return mock insights since we can't mutate the Arc
        let insights = vec![Insight {
            title: "Data Analysis".to_string(),
            description: "Sample insights generated from the query: ".to_string() + &input,
            value: None,
            tags: vec!["ai".to_string(), "insights".to_string()],
            confidence: Some(0.85),
        }];
        
        Ok(insights)
    }

    /// Get available AI agents
    async fn available_agents(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<String>, async_graphql::Error> {
        let orchestrator = ctx.data_unchecked::<Arc<AgentOrchestrator>>();
        Ok(orchestrator.get_available_agents().await)
    }

    /// Get agent status
    async fn agent_status(
        &self,
        ctx: &Context<'_>,
        agent_type: String,
    ) -> Result<Option<AgentStatus>, async_graphql::Error> {
        let orchestrator = ctx.data_unchecked::<Arc<AgentOrchestrator>>();
        Ok(orchestrator.get_agent_status(&agent_type).await)
    }

    /// Test agent connections
    async fn test_agent_connections(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<AgentConnectionTest>, async_graphql::Error> {
        let orchestrator = ctx.data_unchecked::<Arc<AgentOrchestrator>>();
        let results = orchestrator.test_connections().await;
        
        let tests: Vec<AgentConnectionTest> = results.into_iter()
            .map(|(agent, success)| AgentConnectionTest {
                agent_name: agent,
                connected: success,
            })
            .collect();
        
        Ok(tests)
    }

    /// Aggregate data
    async fn aggregate(
        &self,
        ctx: &Context<'_>,
        column: String,
        agg_type: String,
    ) -> Result<f64, async_graphql::Error> {
        let df_ctx = ctx.data_unchecked::<Arc<DataFusionContext>>();
        
        let query = format!("SELECT {}({}) FROM sample", agg_type.to_uppercase(), column);
        let batches = df_ctx.execute_query(&query).await
            .map_err(|e| async_graphql::Error::new(format!("DataFusion error: {}", e)))?;
        
        if batches.is_empty() || batches[0].num_rows() == 0 {
            return Err(async_graphql::Error::new("No data returned from aggregation"));
        }
        
        let value = batches[0].column(0).as_any()
            .downcast_ref::<datafusion::arrow::array::Float64Array>()
            .unwrap()
            .value(0);
        
        Ok(value)
    }
}

/// Mutation root for GraphQL
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Refresh data connection
    async fn refresh_connection(
        &self,
        ctx: &Context<'_>,
    ) -> Result<bool, async_graphql::Error> {
        // TODO: Implement connection refresh logic
        Ok(true)
    }
}

// Use EmptySubscription for now
use async_graphql::EmptySubscription;

/// Agent connection test result
#[derive(SimpleObject)]
pub struct AgentConnectionTest {
    pub agent_name: String,
    pub connected: bool,
}

/// Convert DataFusion batches to Record structs
fn convert_batches_to_records(batches: Vec<datafusion::arrow::record_batch::RecordBatch>) -> Result<Vec<Record>, async_graphql::Error> {
    let mut records = Vec::new();
    
    for batch in batches {
        if batch.num_columns() < 3 {
            return Err(async_graphql::Error::new("Invalid batch structure: expected at least 3 columns"));
        }
        
        let ids = batch.column(0).as_any()
            .downcast_ref::<datafusion::arrow::array::Int32Array>()
            .ok_or_else(|| async_graphql::Error::new("Invalid ID column type"))?;
        
        let names = batch.column(1).as_any()
            .downcast_ref::<datafusion::arrow::array::StringArray>()
            .ok_or_else(|| async_graphql::Error::new("Invalid name column type"))?;
        
        let values = batch.column(2).as_any()
            .downcast_ref::<datafusion::arrow::array::Float64Array>()
            .ok_or_else(|| async_graphql::Error::new("Invalid value column type"))?;
        
        for i in 0..batch.num_rows() {
            records.push(Record {
                id: ids.value(i),
                name: names.value(i).to_string(),
                value: values.value(i),
            });
        }
    }
    
    Ok(records)
}

/// GraphQL schema type
pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

/// Build the GraphQL schema
pub fn build_schema(
    df_ctx: Arc<DataFusionContext>,
    agent_orchestrator: Arc<AgentOrchestrator>,
) -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(df_ctx)
        .data(agent_orchestrator)
        .finish()
}
