use actix_web::{
    web,
    App,
    HttpServer,
    HttpResponse,
    Error,
    web::{self, ServiceConfig},
};
use async_graphql_actix_web::{
    GraphQLRequest,
    GraphQLResponse,
    GraphQLSubscription,
};
use std::sync::Arc;
use tracing::{info, error};
use crate::datafusion::context::DataFusionContext;
use crate::agents::client::AgentClient;
use crate::agents::orchestrator::AgentOrchestrator;
use crate::graphql::schema::{AppSchema, build_schema};
use crate::websocket::handlers::{InsightsWebSocket, StatusWebSocket};
use actix_web_actors::ws;
use futures::StreamExt;

async fn graphql_handler(
    schema: web::Data,
    req: GraphQLRequest,
) -> Result<GraphQLResponse, Error> {
    Ok(schema.execute(req.into_inner()).await.into())
}

async fn graphql_subscription(
    schema: web::Data,
    req: GraphQLRequest,
) -> Result<impl StreamExt<Item = Result<GraphQLResponse, Error>>, Error> {
    Ok(schema.execute_stream(req.into_inner()))
}

async fn playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(async_graphql::http::playground_source(
            async_graphql::http::GraphQLPlaygroundConfig::new("/graphql")
                .subscription_endpoint("/graphql"),
        ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    info!("Starting GraphQL server with WebSocket support...");

    // Initialize DataFusion context
    let df_ctx = Arc::new(DataFusionContext::new().await.unwrap());

    // Initialize agent client
    let agent_client = AgentClient::new(
        std::env::var("AGENT_API_URL").unwrap_or("https://api.x.ai/grok".to_string()),
        std::env::var("AGENT_API_KEY").expect("AGENT_API_KEY required"),
    );

    // Initialize agent orchestrator
    let orchestrator = Arc::new(AgentOrchestrator::new(
        HashMap::from([("default".to_string(), Arc::new(agent_client.clone()))]),
        "default".to_string(),
        3,
        std::time::Duration::from_secs(1),
    ));

    // Build GraphQL schema
    let schema = web::Data::new(build_schema(df_ctx, orchestrator.clone()));

    // Start WebSocket server for agent updates
    let orchestrator_clone = orchestrator.clone();
    tokio::spawn(async move {
        if let Err(e) = websocket_server(orchestrator_clone).await {
            error!("WebSocket server error: {}", e);
        }
    });

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(schema.clone())
            .service(web::resource("/graphql").route(web::post().to(graphql_handler)))
            .service(web::resource("/graphql").route(web::get().to(graphql_subscription)))
            .service(web::resource("/playground").route(web::get().to(playground)))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

async fn websocket_server(orchestrator: Arc<AgentOrchestrator>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(
                web::resource("/ws/insights/{query}")
                    .route(web::get().to(|r: web::Path<String>| ws::start(
                        InsightsWebSocket::new(orchestrator.clone()),
                        r,
                    ))),
            )
            .service(
                web::resource("/ws/status/{agent_type}")
                    .route(web::get().to(|r: web::Path<String>| ws::start(
                        StatusWebSocket::new(orchestrator.clone()),
                        r,
                    ))),
            )
    })
    .bind("127.0.0.1:8001")?
    .run()
    .await
}