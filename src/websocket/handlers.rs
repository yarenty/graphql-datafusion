use actix_web_actors::ws::{self, Message, ProtocolError};
use futures::StreamExt;
use std::sync::Arc;
use crate::agents::orchestrator::AgentOrchestrator;
use serde_json::json;

pub struct InsightsWebSocket {
    orchestrator: Arc<AgentOrchestrator>,
    query: String,
}

impl InsightsWebSocket {
    pub fn new(orchestrator: Arc<AgentOrchestrator>) -> Self {
        Self {
            orchestrator,
            query: String::new(),
        }
    }
}

impl ws::Actor for InsightsWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl ws::StreamHandler<Result<ws::Message, ProtocolError>> for InsightsWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                self.query = text;
                let _ = ctx.address().do_send(SubscribeToInsights(self.query.clone()));
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

impl actix::Handler<SubscribeToInsights> for InsightsWebSocket {
    type Result = ();

    fn handle(&mut self, msg: SubscribeToInsights, ctx: &mut Self::Context) {
        let stream = self.orchestrator.subscribe_to_insights(msg.query.clone());
        
        actix::spawn(async move {
            let mut stream = Box::pin(stream);
            while let Some(Ok(insight)) = stream.next().await {
                if let Err(err) = ctx.text(json!(insight).to_string()) {
                    error!("Failed to send insight: {}", err);
                    break;
                }
            }
        });
    }
}

pub struct StatusWebSocket {
    orchestrator: Arc<AgentOrchestrator>,
    agent_type: String,
}

impl StatusWebSocket {
    pub fn new(orchestrator: Arc<AgentOrchestrator>) -> Self {
        Self {
            orchestrator,
            agent_type: String::new(),
        }
    }
}

impl ws::Actor for StatusWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl ws::StreamHandler<Result<ws::Message, ProtocolError>> for StatusWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                self.agent_type = text;
                let _ = ctx.address().do_send(SubscribeToStatus(self.agent_type.clone()));
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

impl actix::Handler<SubscribeToStatus> for StatusWebSocket {
    type Result = ();

    fn handle(&mut self, msg: SubscribeToStatus, ctx: &mut Self::Context) {
        let stream = self.orchestrator.subscribe_to_status(msg.agent_type.clone());
        
        actix::spawn(async move {
            let mut stream = Box::pin(stream);
            while let Some(Ok(status)) = stream.next().await {
                if let Err(err) = ctx.text(json!(status).to_string()) {
                    error!("Failed to send status: {}", err);
                    break;
                }
            }
        });
    }
}

pub struct SubscribeToInsights(pub String);
impl actix::Message for SubscribeToInsights {
    type Result = ();
}

pub struct SubscribeToStatus(pub String);
impl actix::Message for SubscribeToStatus {
    type Result = ();
}
