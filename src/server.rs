use crate::Config;
use actix_web::{
    App,
    HttpServer,
    HttpResponse,
    middleware::Logger,
    web,
};
use async_graphql::Schema;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use std::sync::Arc;
use tracing::{info, error};
use crate::graphql::schema::SchemaBuilder;
use crate::graphql::context::GraphQLContext;
use crate::security::SecurityMiddleware;
use crate::rate_limit::RateLimitMiddleware;
use crate::auth::AuthMiddleware;

/// Starts the GraphQL DataFusion server
pub async fn start_server(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    let log_level = config.log_level.clone();
    let tracing_level = config.tracing_level.clone();
    
    std::env::set_var("RUST_LOG", &log_level);
    std::env::set_var("RUST_TRACING", &tracing_level);
    
    env_logger::init();
    
    info!("Starting GraphQL DataFusion server");
    
    // Build GraphQL schema
    let schema = SchemaBuilder::new()
        .build()
        .await
        .map_err(|e| format!("Failed to build schema: {}", e))?;
    
    let schema = Arc::new(schema);
    
    // Create context
    let context = GraphQLContext::new(config.clone())
        .await
        .map_err(|e| format!("Failed to create context: {}", e))?;
    
    // Create server
    let server = HttpServer::new(move || {
        App::new()
            // Add security middleware
            .wrap(SecurityMiddleware::new(config.security.clone()))
            // Add rate limiting middleware
            .wrap(RateLimitMiddleware::new(config.rate_limit.clone()))
            // Add authentication middleware
            .wrap(AuthMiddleware::new(config.jwt_secret.clone()))
            // Add logging middleware
            .wrap(Logger::default())
            // Add GraphQL endpoints
            .service(
                web::resource("/graphql")
                    .route(web::post().to(|ctx: web::Data<GraphQLContext>,
                                         schema: web::Data<Schema>,
                                         req: GraphQLRequest|
                                         -> impl actix_web::Responder {
                        async move {
                            let resp = schema
                                .execute(req.into_inner(), &ctx)
                                .await;
                            
                            HttpResponse::Ok().json(resp)
                        }
                    }))
            )
            // Add WebSocket endpoint
            .service(
                web::resource("/ws/{query}")
                    .route(web::get().to(|ctx: web::Data<GraphQLContext>,
                                        schema: web::Data<Schema>,
                                        req: web::Path<String>|
                                        -> impl actix_web::Responder {
                        async move {
                            let query = req.into_inner();
                            let resp = schema
                                .execute(
                                    GraphQLRequest::new(query, None, None),
                                    &ctx
                                )
                                .await;
                            
                            HttpResponse::Ok().json(resp)
                        }
                    }))
            )
    });
    
    // Start server
    server
        .bind(format!("0.0.0.0:{}", config.http_port))?
        .run()
        .await
        .map_err(|e| format!("Failed to start server: {}", e).into())
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::default();
    start_server(config).await
}
