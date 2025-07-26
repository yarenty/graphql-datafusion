use async_graphql::{Schema, Object, Context, InputObject};
use std::sync::Arc;
use crate::datafusion::context::DataFusionContext;
use crate::models::data::Record;
use crate::graphql::query_translator::{QueryTranslator, QueryParams};
use crate::agents::types::{Insight, Visualization, Series, AgentConfig, VisualizationConfig, Filter};
use crate::graphql::helpers::{parse_insights, apply_filters};

pub struct QueryRoot;

#[derive(SimpleObject)]
struct Query {
    // Add your GraphQL queries here
}
#[Object]
impl QueryRoot {
    async fn records(
        &self,
        ctx: &Context<'_>,
        params: QueryParams,
    ) -> Result<Vec<Record>, async_graphql::Error> {
        let df_ctx = ctx.data_unchecked::<Arc<DataFusionContext>>();
        let translator = QueryTranslator::new();
        let query = translator.translate(&params).map_err(|e| async_graphql::Error::new(format!("Query translation error: {}", e)))?;
        
        let batches = df_ctx.execute_query(&query).await.map_err(|e| async_graphql::Error::new(format!("DataFusion error: {}", e)))?;
        let records = batches
            .into_iter()
            .flat_map(|batch| {
                let ids = batch.column(0).as_any().downcast_ref::<Int32Array>().unwrap();
                let names = batch.column(1).as_any().downcast_ref::<StringArray>().unwrap();
                let values = batch.column(2).as_any().downcast_ref::<Float64Array>().unwrap();
                (0..batch.num_rows()).map(move |i| Record {
                    id: ids.value(i),
                    name: names.value(i).to_string(),
                    value: values.value(i),
                }).collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Ok(records)
    }

    #[graphql(guard = "AuthGuard")]
    async fn natural_language_query(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Natural language query")] input: String,
        #[graphql(desc = "Type of agent to use (optional)")] agent_type: Option<String>,
        #[graphql(desc = "Maximum number of results to return (optional)")] limit: Option<i32>,
        #[graphql(desc = "Number of results to skip (optional)")] offset: Option<i32>,
    ) -> Result<(Vec<Record>, String), async_graphql::Error> {
        // Get user claims from context
        let claims = ctx.data::<Claims>()?;
        
        // Check if user has required permissions
        if !claims.role.contains("query") {
            return Err(async_graphql::Error::new("Unauthorized: Query permission required"));
        }

        // Validate input parameters
        let query_input = QueryInput {
            query: input,
            agent_type,
            limit,
            offset,
        };
        
        crate::validation::validate_query_input(ctx, query_input)?;

        let orchestrator = ctx.data_unchecked::<Arc<AgentOrchestrator>>();
        orchestrator.process_query(&input, agent_type).await
    }

    async fn available_agents(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<String>, async_graphql::Error> {
        let orchestrator = ctx.data_unchecked::<Arc<AgentOrchestrator>>();
        Ok(orchestrator.get_available_agents().await)
    }

    async fn insights(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Natural language query")] input: String,
        #[graphql(desc = "Agent configuration")] config: AgentConfig,
    ) -> Result<Vec<Insight>, async_graphql::Error> {
        let orchestrator = ctx.data_unchecked::<Arc<AgentOrchestrator>>();
        let (records, insights_text) = orchestrator.process_query(&input, Some(config.agent_type)).await?;
        
        // Parse insights text into structured format
        let insights = parse_insights(insights_text, &records, &config)?;
        Ok(insights)
    }

    async fn generate_visualization(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Records to visualize")] records: Vec<Record>,
        #[graphql(desc = "Visualization configuration")] config: VisualizationConfig,
    ) -> Result<Visualization, async_graphql::Error> {
        let orchestrator = ctx.data_unchecked::<Arc<AgentOrchestrator>>();
        
        // Use visualization agent
        let (mut records, insights) = orchestrator.process_query(
            &"Generate visualization for these records",
            Some("visualization-agent".to_string()),
        ).await?;
        
        // Apply filters if specified
        if let Some(filters) = &config.filters {
            records = apply_filters(records, filters);
        }
        
        // Generate visualization
        let visualization = orchestrator.process_query(
            &format!(
                "Generate visualization for {} records with config: {:?}",
                records.len(),
                config
            ),
            Some("visualization-agent".to_string()),
        ).await?;
        
        Ok(visualization)
    }

    async fn aggregate(
        &self,
        ctx: &Context<'_>,
        params: QueryParams,
    ) -> Result<f64, async_graphql::Error> {
        let df_ctx = ctx.data_unchecked::<Arc<DataFusionContext>>();
        let translator = QueryTranslator::new();
        let query = translator.translate(&params).map_err(|e| async_graphql::Error::new(format!("Query translation error: {}", e)))?;
        
        let batches = df_ctx.execute_query(&query).await.map_err(|e| async_graphql::Error::new(format!("DataFusion error: {}", e)))?;
        let value = batches[0].column(0).as_any().downcast_ref::<Float64Array>().unwrap().value(0);
        Ok(value)
    }
}

#[derive(SimpleObject)]
struct Query {
    // Add your GraphQL queries here
}

#[derive(SimpleObject)]
struct Mutation {
    // Add your GraphQL mutations here
}

pub type Schema = async_graphql::Schema<Query, Mutation, async_graphql::EmptySubscription>;

pub type AppSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn subscribe_to_updates(&self, ctx: &Context<'_>) -> Result<bool, async_graphql::Error> {
        let orchestrator = ctx.data_unchecked::<Arc<AgentOrchestrator>>();
        orchestrator.subscribe_to_updates().await
    }
}

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn insights_updates(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Query to monitor for updates")] query: String,
    ) -> impl Stream<Item = Result<Insight, async_graphql::Error>> {
        let orchestrator = ctx.data_unchecked::<Arc<AgentOrchestrator>>();
        orchestrator.subscribe_to_insights(query).await
    }

    async fn agent_status(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Agent type to monitor")] agent_type: String,
    ) -> impl Stream<Item = Result<AgentStatus, async_graphql::Error>> {
        let orchestrator = ctx.data_unchecked::<Arc<AgentOrchestrator>>();
        orchestrator.subscribe_to_status(agent_type).await
    }
}

pub fn build_schema(
    df_ctx: Arc<DataFusionContext>,
    agent_orchestrator: Arc<AgentOrchestrator>,
) -> AppSchema {
    Schema::build(
        QueryRoot,
        MutationRoot,
        SubscriptionRoot,
    )
    .data(df_ctx)
    .data(agent_orchestrator)
    .finish()
}
