//! GraphQL DataFusion main entry point

use graphql_datafusion::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    let config = Config::from_env();
    println!("GraphQL DataFusion starting with config:");
    println!("  HTTP Port: {}", config.http_port);
    println!("  Data Path: {}", config.data_path);
    println!("  Table Name: {}", config.table_name);
    println!("  Ollama URL: {}", config.ollama_url);
    println!("  Ollama Model: {}", config.ollama_model);
    
    // For now, just print the config and exit
    // TODO: Start the actual server
    println!("Server would start here...");
    
    Ok(())
} 