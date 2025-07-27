# Configuration Guide

## Environment Variables

### Required

```bash
# AI Agent Configuration
AGENT_API_URL="https://api.x.ai/grok"       # URL for AI agent API
AGENT_API_KEY="your-api-key"                # API key for authentication

# Security
JWT_SECRET="your-secret-key"                # JWT secret for token signing

# Server
PORT=8000                                   # HTTP server port
WS_PORT=8001                                # WebSocket server port
```

### Optional

```bash
# Data Sources
DATABASE_URL="postgresql://user:pass@localhost/db"  # Database connection
CACHE_URL="redis://localhost:6379"                  # Redis cache connection

# Rate Limiting
RATE_LIMIT_WINDOW=60                             # Window size in seconds
RATE_LIMIT_COUNT=100                             # Request limit per window
BURST_LIMIT=5                                    # Burst limit per second

# Agent Configuration
AGENT_RETRY_ATTEMPTS=3                          # Number of retry attempts
AGENT_RETRY_DELAY_MS=1000                       # Delay between retries in ms

# Logging
LOG_LEVEL="info"                                 # Log level (debug, info, warn, error)
RUST_LOG="info"                                 # Rust log level
RUST_TRACING="info"                             # Tracing level

# Cache
CACHE_TTL=3600                                  # Cache TTL in seconds
CACHE_SIZE=1000000                              # Maximum cache size in bytes

# Metrics
METRICS_ENABLED=true                            # Enable Prometheus metrics
METRICS_PORT=9090                               # Metrics server port
```

## File Configuration

### Logging Configuration

Create a `logging.yml` file:

```yaml
version: 1
formatters:
  default:
    format: '%(asctime)s - %(name)s - %(levelname)s - %(message)s'

handlers:
  console:
    class: logging.StreamHandler
    formatter: default
    level: ${LOG_LEVEL:-info}
    stream: ext://sys.stdout

  file:
    class: logging.handlers.RotatingFileHandler
    formatter: default
    level: ${LOG_LEVEL:-info}
    filename: logs/app.log
    maxBytes: 10485760
    backupCount: 5

loggers:
  root:
    level: ${LOG_LEVEL:-info}
    handlers: [console, file]

  datafusion:
    level: ${LOG_LEVEL:-info}
    handlers: [console, file]

  agents:
    level: ${LOG_LEVEL:-info}
    handlers: [console, file]
```

### Prometheus Metrics

Add Prometheus configuration:

```yaml
scrape_configs:
  - job_name: 'graphql-datafusion'
    static_configs:
      - targets: ['localhost:9090']

  - job_name: 'agent-metrics'
    static_configs:
      - targets: ['localhost:9091']
```

## Database Configuration

### PostgreSQL

```sql
-- Create database
CREATE DATABASE datafusion;

-- Create user
CREATE USER datafusion WITH PASSWORD 'your-password';

-- Grant permissions
GRANT ALL PRIVILEGES ON DATABASE datafusion TO datafusion;
```

### Redis Cache

```bash
# Install Redis
sudo apt-get install redis-server

# Configure Redis
sudo nano /etc/redis/redis.conf

# Add configuration
maxmemory 100mb
maxmemory-policy allkeys-lru
```

## Security Configuration

### JWT Configuration

```rust
pub struct AuthConfig {
    pub secret_key: String,           // JWT secret key
    pub token_expiration: Duration,   // Token expiration time
    pub refresh_expiration: Duration, // Refresh token expiration
    pub issuer: String,              // JWT issuer
    pub audience: String,            // JWT audience
}
```

### Rate Limiting

```rust
pub struct RateLimitConfig {
    pub window: Duration,            // Rate limiting window
    pub limit: u32,                 // Request limit per window
    pub burst: u32,                 // Burst limit
    pub ip_tracking: bool,          // Enable IP tracking
    pub headers: bool,              // Enable rate limit headers
}
```

## Monitoring Configuration

### Prometheus

Add metrics endpoints:

```rust
pub struct MetricsConfig {
    pub enabled: bool,              // Enable metrics
    pub port: u16,                 // Metrics server port
    pub path: String,              // Metrics endpoint path
    pub scrape_interval: Duration, // Scrape interval
}
```

### Tracing

Configure tracing:

```rust
pub struct TracingConfig {
    pub enabled: bool,              // Enable tracing
    pub level: String,             // Tracing level
    pub service_name: String,      // Service name
    pub collector_url: String,     // Collector URL
    pub batch_size: usize,         // Batch size
}
```

## Example Configuration

### Full Example

```bash
# Basic Configuration
AGENT_API_URL="https://api.x.ai/grok"
AGENT_API_KEY="your-api-key"
JWT_SECRET="your-secret-key"

# Server Configuration
PORT=8000
WS_PORT=8001

# Data Sources
DATABASE_URL="postgresql://datafusion:password@localhost/datafusion"
CACHE_URL="redis://localhost:6379"

# Rate Limiting
RATE_LIMIT_WINDOW=60
RATE_LIMIT_COUNT=100
BURST_LIMIT=5

# Agent Configuration
AGENT_RETRY_ATTEMPTS=3
AGENT_RETRY_DELAY_MS=1000

# Logging
LOG_LEVEL="info"
RUST_LOG="info"
RUST_TRACING="info"

# Cache
CACHE_TTL=3600
CACHE_SIZE=1000000

# Metrics
METRICS_ENABLED=true
METRICS_PORT=9090
```

### Docker Configuration

```docker-compose.yml
version: '3'
services:
  graphql-datafusion:
    build: .
    ports:
      - "8000:8000"
      - "8001:8001"
      - "9090:9090"
    environment:
      - AGENT_API_URL=https://api.x.ai/grok
      - AGENT_API_KEY=your-api-key
      - JWT_SECRET=your-secret-key
      - DATABASE_URL=postgresql://datafusion:password@db/datafusion
      - CACHE_URL=redis://cache:6379
    depends_on:
      - db
      - cache

  db:
    image: postgres:14
    ports:
      - "5432:5432"
    environment:
      - POSTGRES_USER=datafusion
      - POSTGRES_PASSWORD=password
      - POSTGRES_DB=datafusion

  cache:
    image: redis:6
    ports:
      - "6379:6379"
```

## Best Practices

### Security

1. Never commit sensitive credentials
2. Use environment variables for configuration
3. Enable rate limiting
4. Configure proper CORS
5. Use secure JWT secrets
6. Enable HTTPS in production

### Performance

1. Configure proper cache sizes
2. Set appropriate TTLs
3. Monitor memory usage
4. Use connection pooling
5. Enable metrics collection

### Monitoring

1. Enable Prometheus metrics
2. Configure tracing
3. Set up alerting
4. Monitor error rates
5. Track response times

### Logging

1. Configure proper log levels
2. Use structured logging
3. Enable log rotation
4. Monitor logs
5. Set up error notifications
