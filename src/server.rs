//! GraphQL DataFusion server

use graphql_datafusion::Config;
use graphql_datafusion::datafusion::context::DataFusionContext;
use graphql_datafusion::graphql::schema::{AppSchema, build_schema};
use graphql_datafusion::agents::client::AgentClient;
use graphql_datafusion::agents::orchestrator::AgentOrchestrator;
use actix_web::{App, HttpResponse, HttpServer, middleware::Logger, web};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tracing::{info, error};

async fn graphql_handler(
    schema: web::Data<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(async_graphql::http::playground_source(
            async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
        ))
}

pub async fn start_server(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    unsafe {
        std::env::set_var("RUST_LOG", &config.log_level);
    }
    env_logger::init();

    info!("Starting GraphQL DataFusion server on port {}", config.http_port);

    // Initialize DataFusion context
    let df_ctx = Arc::new(DataFusionContext::new(&config.data_path, &config.table_name).await
        .map_err(|e| format!("Failed to initialize DataFusion: {}", e))?);

    // Initialize agent system
    let mut clients = HashMap::new();
    let client = Arc::new(AgentClient::new(
        config.ollama_url.clone(),
        config.ollama_model.clone(),
    ));
    clients.insert("default".to_string(), client);

    let orchestrator = Arc::new(AgentOrchestrator::new(
        clients,
        "default".to_string(),
        3,
        Duration::from_secs(1),
    ));

    // Build GraphQL schema
    let schema = web::Data::new(build_schema(df_ctx, orchestrator));

    // Start server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(schema.clone())
            .service(web::resource("/graphql").route(web::post().to(graphql_handler)))
            .service(web::resource("/playground").route(web::get().to(playground)))
    })
    .bind(format!("0.0.0.0:{}", config.http_port))?
    .run()
    .await
    .map_err(|e| format!("Failed to start server: {}", e).into())
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env();
    start_server(config).await
}
