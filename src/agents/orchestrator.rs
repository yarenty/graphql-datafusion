//! Agent orchestrator for managing multiple AI agents

use crate::agents::client::AgentClient;
use crate::agents::types::{AgentStatus, Insight};
use crate::models::data::Record;
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
    retry_attempts: u32,
    retry_delay: Duration,
    agent_stats: HashMap<String, u64>,
}

impl AgentOrchestrator {
    /// Create a new agent orchestrator
    pub fn new(
        clients: HashMap<String, Arc<AgentClient>>,
        default_agent: String,
        retry_attempts: u32,
        retry_delay: Duration,
    ) -> Self {
        let mut agent_stats = HashMap::new();
        for agent_name in clients.keys() {
            agent_stats.insert(agent_name.clone(), 0);
        }

        Self {
            clients,
            default_agent,
            retry_attempts,
            retry_delay,
            agent_stats,
        }
    }

    /// Process a natural language query
    pub async fn process_query(
        &mut self,
        input: &str,
        agent_type: Option<String>,
    ) -> Result<(Vec<Record>, String), Error> {
        let agent_name = agent_type.unwrap_or_else(|| self.default_agent.clone());
        
        let client = self.clients.get(&agent_name)
            .ok_or_else(|| Error::new(format!("Agent {} not found", agent_name)))?;

        info!("Processing query with agent: {}", agent_name);

        // Try with retries
        for attempt in 0..self.retry_attempts {
            match self.attempt_process_query(client, input).await {
                Ok(result) => {
                    // Update stats
                    if let Some(stats) = self.agent_stats.get_mut(&agent_name) {
                        *stats += 1;
                    }
                    return Ok(result);
                }
                Err(e) => {
                    if attempt == self.retry_attempts - 1 {
                        error!("All attempts failed for agent {}: {:?}", agent_name, e);
                        return Err(e);
                    }
                    warn!("Attempt {} failed for agent {}: {:?}. Retrying...", 
                          attempt + 1, agent_name, e);
                    tokio::time::sleep(self.retry_delay).await;
                }
            }
        }

        Err(Error::new("All retries failed"))
    }

    /// Attempt to process a query with a specific agent
    async fn attempt_process_query(
        &self,
        client: &Arc<AgentClient>,
        input: &str,
    ) -> Result<(Vec<Record>, String), Error> {
        // Step 1: Translate natural language to SQL
        let sql = client.translate_to_sql(input).await?;
        info!("Generated SQL: {}", sql);

        // Step 2: For now, return mock data
        // In production, this would execute SQL via DataFusion
        let records = vec![
            Record { id: 1, name: "Sample 1".to_string(), value: 100.0 },
            Record { id: 2, name: "Sample 2".to_string(), value: 200.0 },
            Record { id: 3, name: "Sample 3".to_string(), value: 150.0 },
        ];
        
        info!("Retrieved {} records", records.len());

        // Step 3: Generate insights
        let insights = client.generate_insights(records.clone()).await?;
        info!("Generated insights: {}", insights);

        Ok((records, insights))
    }

    /// Get available agents
    pub async fn get_available_agents(&self) -> Vec<String> {
        self.clients.keys().cloned().collect()
    }

    /// Get agent status
    pub async fn get_agent_status(&self, agent_type: &str) -> Option<AgentStatus> {
        let client = self.clients.get(agent_type)?;
        let requests_processed = self.agent_stats.get(agent_type).copied().unwrap_or(0);
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Some(AgentStatus {
            agent_type: agent_type.to_string(),
            status: "active".to_string(),
            last_update: now.to_string(),
            model: "llama2".to_string(), // This should come from the client
            requests_processed,
        })
    }

    /// Test connection to all agents
    pub async fn test_connections(&self) -> HashMap<String, bool> {
        let mut results = HashMap::new();
        
        for (agent_name, client) in &self.clients {
            match client.test_connection().await {
                Ok(success) => {
                    results.insert(agent_name.clone(), success);
                    info!("Agent {} connection test: {}", agent_name, success);
                }
                Err(e) => {
                    error!("Agent {} connection test failed: {:?}", agent_name, e);
                    results.insert(agent_name.clone(), false);
                }
            }
        }
        
        results
    }
}
