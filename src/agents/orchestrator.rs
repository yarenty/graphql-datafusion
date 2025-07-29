//! Agent orchestrator for managing multiple AI agents

use crate::agents::client::AgentClient;
use crate::agents::types::{AgentConfig, AgentStatus};
use crate::models::data::Customer;
use async_graphql::Error;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::{error, info, warn};

/// Agent orchestrator for managing multiple AI agents
#[derive(Debug)]
pub struct AgentOrchestrator {
    clients: HashMap<String, Arc<AgentClient>>,
    default_agent: String,
    agent_stats: HashMap<String, u64>,
}

impl AgentOrchestrator {
    pub fn new() -> Self {
        let mut clients = HashMap::new();

        // Initialize default agent
        let default_client = Arc::new(AgentClient::new(
            "http://localhost:11434".to_string(),
            "llama2".to_string(),
        ));
        clients.insert("default".to_string(), default_client);

        Self {
            clients,
            default_agent: "default".to_string(),
            agent_stats: HashMap::new(),
        }
    }

    pub fn with_agent(mut self, agent_type: String, client: AgentClient) -> Self {
        self.clients.insert(agent_type.clone(), Arc::new(client));
        self
    }

    pub async fn process_query(
        &mut self,
        input: &str,
        agent_type: Option<String>,
    ) -> Result<(Vec<Customer>, String), Error> {
        let agent_name = agent_type.unwrap_or_else(|| self.default_agent.clone());

        // Update stats
        *self.agent_stats.entry(agent_name.clone()).or_insert(0) += 1;

        let client = self
            .clients
            .get(&agent_name)
            .ok_or_else(|| Error::new(format!("Agent '{}' not found", agent_name)))?;

        self.attempt_process_query(client, input).await
    }

    async fn attempt_process_query(
        &self,
        client: &Arc<AgentClient>,
        input: &str,
    ) -> Result<(Vec<Customer>, String), Error> {
        // Step 1: Translate natural language to SQL
        let sql = client.translate_to_sql(input).await?;
        info!("Generated SQL: {}", sql);

        // Step 2: Execute SQL (in production, this would use DataFusion)
        // For now, return mock data
        let records = vec![
            Customer {
                c_custkey: 1,
                c_name: "Customer#000000001".to_string(),
                c_address: "Sample Address 1".to_string(),
                c_nationkey: 1,
                c_phone: "25-989-741-2988".to_string(),
                c_acctbal: 100.0,
                c_mktsegment: "BUILDING".to_string(),
                c_comment: "Sample customer 1".to_string(),
            },
            Customer {
                c_custkey: 2,
                c_name: "Customer#000000002".to_string(),
                c_address: "Sample Address 2".to_string(),
                c_nationkey: 2,
                c_phone: "23-768-687-3665".to_string(),
                c_acctbal: 200.0,
                c_mktsegment: "AUTOMOBILE".to_string(),
                c_comment: "Sample customer 2".to_string(),
            },
        ];

        // Step 3: Generate insights from the data
        let insights = client.generate_insights(records.clone()).await?;

        Ok((records, insights))
    }

    pub async fn get_available_agents(&self) -> Vec<String> {
        self.clients.keys().cloned().collect()
    }

    pub async fn get_agent_status(&self, agent_type: &str) -> Option<AgentStatus> {
        let requests = self.agent_stats.get(agent_type).copied().unwrap_or(0);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();

        Some(AgentStatus {
            agent_type: agent_type.to_string(),
            status: "active".to_string(),
            last_update: now.to_string(),
            model: "llama2".to_string(),
            requests_processed: requests,
        })
    }

    pub async fn test_connections(&self) -> HashMap<String, bool> {
        let mut results = HashMap::new();

        for (agent_name, client) in &self.clients {
            match client.test_connection().await {
                Ok(success) => {
                    results.insert(agent_name.clone(), success);
                    if success {
                        info!("Agent '{}' connection test successful", agent_name);
                    } else {
                        warn!("Agent '{}' connection test failed", agent_name);
                    }
                }
                Err(e) => {
                    error!("Agent '{}' connection test error: {:?}", agent_name, e);
                    results.insert(agent_name.clone(), false);
                }
            }
        }

        results
    }
}
