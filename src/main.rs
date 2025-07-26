use actix_web::{
    web,
    App,
    HttpServer,
    HttpResponse,
    Error,
    web::{self, ServiceConfig},
    middleware::{self, Middleware},
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
use crate::auth::{AuthGuard, AuthConfig};
use crate::rate_limit::{RateLimiter, RateLimitMiddleware};
use crate::security::SecurityHeadersMiddleware;
use actix_web_actors::ws;
use futures::StreamExt;
use jsonwebtoken::Validation;

struct AuthMiddleware;

impl Middleware for AuthMiddleware {
    async fn handle(
        &self,
        req: actix_web::HttpRequest,
        next: actix_web::dev::ServiceRequest,
    ) -> Result<actix_web::dev::ServiceResponse, actix_web::Error> {
        let auth_header = req.headers().get("Authorization");
        
        if let Some(header) = auth_header {
            if let Ok(token) = header.to_str() {
                if let Some(token) = token.strip_prefix("Bearer ") {
                    let auth_guard = req.app_data::<Arc<AuthGuard>>().unwrap();
                    if let Ok(claims) = auth_guard.verify_token(token) {
                        req.extensions_mut().insert(claims);
                        return Ok(next.call(req).await?);
                    }
                }
            }
        }
        
        Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
    }
}

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
    let auth_config = AuthConfig {
        secret_key: std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key-change-in-production".to_string()),
        token_expiration: std::time::Duration::from_secs(3600), // 1 hour
    };
    let auth_guard = Arc::new(AuthGuard::new(auth_config));
    
    // Initialize rate limiter
    let rate_limiter = Arc::new(RateLimiter::new());
    
    HttpServer::new(move || {
        App::new()
            .app_data(schema.clone())
            .app_data(auth_guard.clone())
            .app_data(rate_limiter.clone())
            .wrap(AuthMiddleware)
            .wrap(RateLimitMiddleware::new(rate_limiter.clone()))
            .wrap(SecurityHeadersMiddleware)
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