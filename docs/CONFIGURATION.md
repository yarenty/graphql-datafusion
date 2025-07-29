# Configuration Guide

## 🚀 Overview

This guide covers all configuration options for the GraphQL DataFusion API, including data discovery, AI integration, server settings, and performance tuning.

## 📊 Data Discovery Configuration

### Data Directory Configuration

The API automatically discovers and loads data from configured directories:

```rust
// src/config.rs
pub struct Config {
    // Data discovery settings
    pub data_path: String,           // Directory containing data files
    pub auto_discovery: bool,        // Enable automatic schema discovery
    pub supported_formats: Vec<String>, // File formats to process
    pub max_file_size: usize,        // Maximum file size in bytes
    pub scan_interval: u64,          // Schema refresh interval in seconds
}
```

### Environment Variables

```bash
# Data discovery
DATA_PATH=/path/to/data/directory
AUTO_DISCOVERY=true
SUPPORTED_FORMATS=csv,parquet,json,jsonl
MAX_FILE_SIZE=10737418240  # 10GB
SCAN_INTERVAL=300          # 5 minutes

# File format specific settings
CSV_DELIMITER=,
CSV_HAS_HEADER=true
JSON_LINES=false
PARQUET_COMPRESSION=snappy
```

### Supported File Formats

#### CSV Configuration
```toml
[data.csv]
delimiter = ","
has_header = true
encoding = "utf-8"
null_values = ["", "null", "NULL", "NA"]
date_formats = ["%Y-%m-%d", "%m/%d/%Y", "%d-%m-%Y"]
```

#### Parquet Configuration
```toml
[data.parquet]
compression = "snappy"
row_group_size = 100000
enable_dictionary = true
```

#### JSON Configuration
```toml
[data.json]
lines_format = false
batch_size = 1000
```

## 🤖 AI Integration Configuration

### Ollama Configuration

```rust
pub struct OllamaConfig {
    pub base_url: String,           // Ollama server URL
    pub model: String,              // Model name (e.g., "llama2", "mistral")
    pub timeout: Duration,          // Request timeout
    pub max_tokens: usize,          // Maximum response tokens
    pub temperature: f32,           // Response creativity (0.0-1.0)
    pub context_window: usize,      // Context window size
}
```

### Environment Variables

```bash
# Ollama settings
OLLAMA_BASE_URL=http://localhost:11434
OLLAMA_MODEL=llama2
OLLAMA_TIMEOUT=30
OLLAMA_MAX_TOKENS=2048
OLLAMA_TEMPERATURE=0.7
OLLAMA_CONTEXT_WINDOW=4096

# AI prompt templates
NATURAL_LANGUAGE_PROMPT="Convert this natural language query to SQL: {query}"
INSIGHTS_PROMPT="Analyze this data and provide business insights: {data}"
```

### AI Prompt Templates

```toml
[ai.prompts]
natural_language = """
You are a SQL expert. Convert the following natural language query to SQL.
Available tables: {tables}
Query: {query}
Return only the SQL query, no explanations.
"""

insights = """
You are a business analyst. Analyze the following data and provide insights.
Data: {data}
Focus on trends, patterns, and actionable recommendations.
"""

schema_analysis = """
Analyze this table schema and suggest relationships with other tables.
Schema: {schema}
Return JSON with discovered relationships.
"""
```

## 🌐 Server Configuration

### HTTP Server Settings

```rust
pub struct ServerConfig {
    pub host: String,               // Server host
    pub port: u16,                  // Server port
    pub workers: usize,             // Number of worker threads
    pub max_connections: usize,     // Maximum concurrent connections
    pub request_timeout: Duration,  // Request timeout
    pub keep_alive: Duration,       // Keep-alive timeout
}
```

### Environment Variables

```bash
# Server settings
HOST=0.0.0.0
PORT=8080
WORKERS=4
MAX_CONNECTIONS=1000
REQUEST_TIMEOUT=30
KEEP_ALIVE=75

# CORS settings
CORS_ALLOW_ORIGIN=*
CORS_ALLOW_METHODS=GET,POST,OPTIONS
CORS_ALLOW_HEADERS=Content-Type,Authorization
```

### CORS Configuration

```toml
[server.cors]
allow_origin = "*"
allow_methods = ["GET", "POST", "OPTIONS"]
allow_headers = ["Content-Type", "Authorization"]
allow_credentials = true
max_age = 86400
```

## 📈 Performance Configuration

### DataFusion Settings

```rust
pub struct DataFusionConfig {
    pub memory_limit: usize,        // Memory limit in bytes
    pub batch_size: usize,          // Query batch size
    pub partition_size: usize,      // Partition size for large datasets
    pub cache_size: usize,          // Cache size in bytes
    pub enable_optimization: bool,  // Enable query optimization
}
```

### Environment Variables

```bash
# DataFusion performance
DATAFUSION_MEMORY_LIMIT=1073741824  # 1GB
DATAFUSION_BATCH_SIZE=8192
DATAFUSION_PARTITION_SIZE=100000
DATAFUSION_CACHE_SIZE=268435456     # 256MB
DATAFUSION_ENABLE_OPTIMIZATION=true
```

### Query Optimization

```toml
[performance.optimization]
enable_predicate_pushdown = true
enable_column_pruning = true
enable_join_reordering = true
enable_aggregation_pushdown = true
max_concurrent_queries = 10
```

## 🔒 Security Configuration

### Authentication (Optional)

```rust
pub struct AuthConfig {
    pub enabled: bool,              // Enable authentication
    pub jwt_secret: String,         // JWT secret key
    pub token_expiry: Duration,     // Token expiry time
    pub allowed_origins: Vec<String>, // Allowed CORS origins
}
```

### Environment Variables

```bash
# Authentication (optional)
AUTH_ENABLED=false
JWT_SECRET=your-secret-key
TOKEN_EXPIRY=3600
ALLOWED_ORIGINS=http://localhost:3000,https://yourdomain.com
```

### Rate Limiting

```toml
[security.rate_limit]
enabled = true
requests_per_minute = 100
burst_limit = 10
window_size = 60
```

## 📝 Logging Configuration

### Log Levels

```rust
pub struct LogConfig {
    pub level: String,              // Log level (debug, info, warn, error)
    pub format: String,             // Log format (json, text)
    pub output: String,             // Output destination (stdout, file)
    pub file_path: Option<String>,  // Log file path
}
```

### Environment Variables

```bash
# Logging
RUST_LOG=info
LOG_FORMAT=json
LOG_OUTPUT=stdout
LOG_FILE_PATH=/var/log/graphql-datafusion.log
```

### Log Configuration

```toml
[logging]
level = "info"
format = "json"
output = "stdout"
file_path = "/var/log/graphql-datafusion.log"

[logging.filters]
sql_queries = true
ai_requests = true
performance_metrics = true
```

## 🔧 Development Configuration

### Development Settings

```toml
[development]
debug_mode = true
hot_reload = true
mock_ai_responses = true
sample_data_path = "./sample_data"
```

### Environment Variables

```bash
# Development
DEBUG_MODE=true
HOT_RELOAD=true
MOCK_AI_RESPONSES=true
SAMPLE_DATA_PATH=./sample_data
```

## 📊 Monitoring Configuration

### Metrics Collection

```rust
pub struct MetricsConfig {
    pub enabled: bool,              // Enable metrics collection
    pub prometheus_port: u16,       // Prometheus metrics port
    pub custom_metrics: bool,       // Enable custom metrics
    pub health_check: bool,         // Enable health check endpoint
}
```

### Environment Variables

```bash
# Monitoring
METRICS_ENABLED=true
PROMETHEUS_PORT=9090
CUSTOM_METRICS=true
HEALTH_CHECK=true
```

### Health Check Configuration

```toml
[monitoring.health_check]
enabled = true
endpoint = "/health"
check_database = true
check_ai_service = true
timeout = 5
```

## 🚀 Production Configuration

### Production Settings

```toml
[production]
data_path = "/data"
auto_discovery = true
max_file_size = 10737418240  # 10GB
workers = 8
memory_limit = 2147483648     # 2GB
cache_size = 536870912        # 512MB

[production.ai]
base_url = "http://ollama:11434"
model = "llama2"
timeout = 60
max_tokens = 4096

[production.server]
host = "0.0.0.0"
port = 8080
max_connections = 2000
request_timeout = 60
```

### Docker Configuration

```dockerfile
# Environment variables for Docker
ENV DATA_PATH=/data
ENV OLLAMA_BASE_URL=http://ollama:11434
ENV RUST_LOG=info
ENV WORKERS=8
ENV MAX_CONNECTIONS=2000
```

## 🔍 Configuration Validation

### Configuration Schema

```json
{
  "type": "object",
  "properties": {
    "data_path": {
      "type": "string",
      "description": "Path to data directory"
    },
    "auto_discovery": {
      "type": "boolean",
      "description": "Enable automatic schema discovery"
    },
    "ollama_base_url": {
      "type": "string",
      "format": "uri",
      "description": "Ollama server URL"
    },
    "server_port": {
      "type": "integer",
      "minimum": 1,
      "maximum": 65535,
      "description": "Server port number"
    }
  },
  "required": ["data_path", "ollama_base_url"]
}
```

### Configuration Validation

```rust
impl Config {
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate data path exists
        if !std::path::Path::new(&self.data_path).exists() {
            return Err(ConfigError::InvalidDataPath);
        }
        
        // Validate Ollama URL
        if !self.ollama_base_url.starts_with("http") {
            return Err(ConfigError::InvalidOllamaUrl);
        }
        
        // Validate port range
        if self.server_port < 1 || self.server_port > 65535 {
            return Err(ConfigError::InvalidPort);
        }
        
        Ok(())
    }
}
```

## 🔧 Configuration Examples

### Minimal Configuration

```toml
# config.toml
data_path = "./data"
ollama_base_url = "http://localhost:11434"
server_port = 8080
```

### Development Configuration

```toml
# config.dev.toml
data_path = "./sample_data"
auto_discovery = true
ollama_base_url = "http://localhost:11434"
ollama_model = "llama2"
server_port = 8080
debug_mode = true
mock_ai_responses = true
```

### Production Configuration

```toml
# config.prod.toml
data_path = "/data"
auto_discovery = true
max_file_size = 10737418240
ollama_base_url = "http://ollama:11434"
ollama_model = "llama2"
ollama_timeout = 60
server_port = 8080
workers = 8
max_connections = 2000
memory_limit = 2147483648
cache_size = 536870912
metrics_enabled = true
health_check = true
```

## 🔗 Related Documentation

- [API Documentation](API.md) - Complete API reference
- [Deployment Guide](DEPLOYMENT.md) - Production deployment instructions
- [Troubleshooting Guide](TROUBLESHOOTING.md) - Common issues and solutions
