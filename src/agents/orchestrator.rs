use crate::agents::client::AgentClient;
use crate::models::data::Record;
use async_graphql::{Error};
use async_trait::async_trait;
use futures::Stream;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::broadcast;
use tracing::{error, info, warn};

#[derive(Debug, Clone)]
pub struct AgentStatus {
    pub agent_type: String,
    pub status: String,
    pub last_update: String,
    pub metrics: serde_json::Value,
}

#[derive(Debug)]
pub struct AgentOrchestrator {
    clients: HashMap<String, Arc<AgentClient>>,
    default_agent: String,
    retry_attempts: u32,
    retry_delay: Duration,

    // Subscription channels
    insight_channels: Arc<Mutex<HashMap<String, broadcast::Sender<Insight>>>>,
    status_channels: Arc<Mutex<HashMap<String, broadcast::Sender<AgentStatus>>>>,
    subscribers: Arc<Mutex<HashSet<String>>>,
}

impl AgentOrchestrator {
    pub fn new(
        clients: HashMap<String, Arc<AgentClient>>,
        default_agent: String,
        retry_attempts: u32,
        retry_delay: Duration,
    ) -> Self {
        Self {
            clients,
            default_agent,
            retry_attempts,
            retry_delay,
            insight_channels: Arc::new(Mutex::new(HashMap::new())),
            status_channels: Arc::new(Mutex::new(HashMap::new())),
            subscribers: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    pub async fn process_query(
        &self,
        input: &str,
        agent_type: Option<String>,
    ) -> Result<(Vec<Record>, String), Error> {
        let agent_name = agent_type
            .clone()
            .unwrap_or_else(|| self.default_agent.clone());

        let client = self
            .clients
            .get(&agent_name)
            .ok_or_else(|| Error::new(format!("Agent {} not found", agent_name)))?;

        info!("Processing query with agent: {}", agent_name);

        // Try with retries
        for attempt in 0..self.retry_attempts {
            match self.attempt_process_query(client, input).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if attempt == self.retry_attempts - 1 {
                        error!("All attempts failed for agent {}: {:?}", agent_name, e);
                        return Err(e);
                    }
                    warn!(
                        "Attempt {} failed for agent {}: {:?}. Retrying...",
                        attempt + 1,
                        agent_name,
                        e
                    );
                    tokio::time::sleep(self.retry_delay).await;
                }
            }
        }

        Err(Error::new("All retries failed"))
    }

    async fn attempt_process_query(
        &self,
        client: &Arc<AgentClient>,
        input: &str,
    ) -> Result<(Vec<Record>, String), Error> {
        // Step 1: Translate natural language to SQL
        let sql = client.translate_to_sql(input).await?;
        info!("Generated SQL: {}", sql);

        // Step 2: Execute SQL and get records
        let records = vec![]; // Placeholder - will be replaced with actual DataFusion execution
        info!("Retrieved {} records", records.len());

        // Step 3: Generate insights
        let insights = client.generate_insights(records.clone()).await?.to_string();
        info!("Generated insights: {}", insights);

        // Broadcast insights to subscribers
        self.broadcast_insight(&input, &insights).await;

        Ok((records, insights))
    }

    async fn broadcast_insight(&self, query: &str, insights: &str) {
        let mut channels = self.insight_channels.lock().unwrap();
        if let Some(sender) = channels.get(query) {
            let insight = Insight {
                title: query.to_string(),
                description: insights.to_string(),
                value: None,
                visualization: None,
                tags: vec!["realtime".to_string()],
            };
            let _ = sender.send(insight);
        }
    }

    pub fn subscribe_insights(&self, query: String) -> impl Stream<Item = Result<Insight, Error>> {
        let (tx, rx) = broadcast::channel(100);
        let mut channels = self.insight_channels.lock().unwrap();
        channels.insert(query.clone(), tx);

        //        let mut subscribers = self.subscribers.lock().unwrap();
        //         subscribers.insert(query.clone());
        //
        //         Box::pin(rx)

        struct BroadcastStream<T>(broadcast::Receiver<T>);

        impl<T> Stream for BroadcastStream<T> {
            type Item = Result<T, Error>;

            fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
                let this = self.get_mut();
                match Pin::new(&mut this.0).poll_recv(cx) {
                    Poll::Ready(Some(Ok(value))) => Poll::Ready(Some(Ok(value))),
                    Poll::Ready(Some(Err(e))) => {
                        Poll::Ready(Some(Err(Error::Other(e.to_string()))))
                    }
                    Poll::Ready(None) => Poll::Ready(None),
                    Poll::Pending => Poll::Pending,
                }
            }
        }

        Box::pin(BroadcastStream(rx))
    }

    pub fn subscribe_agent_status(
        &self,
        agent_type: String,
    ) -> impl Stream<Item = Result<AgentStatus, Error>> {
        let (tx, rx) = broadcast::channel(100);
        let mut channels = self.status_channels.lock().unwrap();
        channels.insert(agent_type.clone(), tx);

        //        let mut subscribers = self.subscribers.lock().unwrap();
        //         subscribers.insert(agent_type.clone());
        //
        //         Box::pin(rx)
        struct BroadcastStream<T>(broadcast::Receiver<T>);

        impl<T> Stream for BroadcastStream<T> {
            type Item = Result<T, Error>;

            fn poll_next(
                self: Pin<&mut Self>,
                cx: &mut Context<'_>,
            ) -> Poll::Ready<Option<Self::Item>> {
                let this = self.get_mut();
                match Pin::new(&mut this.0).poll_recv(cx) {
                    Poll::Ready(Some(Ok(value))) => Poll::Ready(Some(Ok(value))),
                    Poll::Ready(Some(Err(e))) => {
                        Poll::Ready(Some(Err(Error::Other(e.to_string()))))
                    }
                    Poll::Ready(None) => Poll::Ready(None),
                    Poll::Pending => Poll::Pending,
                }
            }
        }

        Box::pin(BroadcastStream(rx))
    }

    pub async fn get_available_agents(&self) -> Vec<String> {
        self.clients.keys().cloned().collect()
    }

    pub async fn subscribe_to_updates(&self) -> Result<bool, Error> {
        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers.insert("updates".to_string());
        Ok(true)
    }
}

#[async_trait]
pub trait AgentOrchestratorTrait {
    async fn execute(&self, task: &str) -> Result<String, String>;
}
