# GraphQL DataFusion with Agentic AI

A Rust-based GraphQL server that integrates Apache DataFusion for data querying and Agentic AI for natural language processing and insights generation. This project provides a scalable, secure, and feature-rich solution for data analysis with AI-powered insights.

## Features

### Core Features

- **GraphQL API**: 
  - Real-time subscriptions
  - Complex query support
  - Input validation
  - Error handling

- **Data Processing**:
  - Apache DataFusion integration
  - Multiple data source support
  - Query optimization
  - Caching layer

- **AI Integration**:
  - Natural language processing
  - Multi-agent orchestration
  - Real-time insights
  - Visualization generation

- **Real-time Updates**:
  - WebSocket support
  - Broadcast channels
  - Status monitoring
  - Insights streaming

### Security Features

- **Authentication**:
  - JWT token-based
  - Role-based access
  - Token expiration
  - Secure storage

- **Rate Limiting**:
  - Per-endpoint limits
  - Burst protection
  - IP tracking
  - Window-based

- **Input Validation**:
  - Parameter validation
  - SQL injection prevention
  - Size limits
  - Content validation

- **Security Headers**:
  - HSTS
  - XSS protection
  - Frame protection
  - CSP
  - CORS

## Getting Started

### Prerequisites

- Rust 1.80.0 or higher
- Apache DataFusion
- AI Agent API (e.g., Grok API)
- PostgreSQL (optional)
- Redis (optional)

### Installation

1. Clone the repository:
```bash
git clone [repository-url]
cd graphql-datafusion
```

2. Install dependencies:
```bash
cargo install
```

3. Set environment variables:
```bash
# Required
export AGENT_API_URL="https://api.x.ai/grok"
export AGENT_API_KEY="your-api-key"
export JWT_SECRET="your-secret-key"

# Optional
export DATABASE_URL="postgresql://user:pass@localhost/db"
export CACHE_URL="redis://localhost:6379"
export LOG_LEVEL="info"
```

4. Run the server:
```bash
cargo run
```

The server will start on `http://localhost:8000` with GraphQL Playground available at `/playground`.

## API Documentation

### GraphQL Endpoints

- `POST /graphql` - Main GraphQL endpoint
- `GET /graphql` - GraphQL subscription endpoint
- `GET /playground` - GraphQL Playground UI
- `GET /schema` - GraphQL schema download

### WebSocket Endpoints

- `ws://localhost:8001/ws/insights/{query}` - Insights updates
- `ws://localhost:8001/ws/status/{agent_type}` - Agent status updates
- `ws://localhost:8001/ws/agents` - Agent monitoring

### GraphQL Schema

```graphql
type Query {
  records: [Record!]! # Get raw records
  naturalLanguageQuery(
    input: String!
    agentType: String
    limit: Int
    offset: Int
  ): (records: [Record!]!, insights: String!) # Process natural language query
  availableAgents: [String!]! # List available agents
  insights(
    input: String!
    config: AgentConfig
  ): [Insight!]! # Generate insights
  translateQuery(input: String!): String # Translate natural language to SQL
  generateVisualization(
    input: String!
    config: VisualizationConfig
  ): Visualization! # Generate data visualization
}

type Subscription {
  insightsUpdates(query: String!): Insight! # Subscribe to insights updates
  agentStatus(agentType: String!): AgentStatus! # Subscribe to agent status
  agentMetrics: AgentMetrics! # Real-time agent metrics
}

input AgentConfig {
  agentType: String!
  visualization: VisualizationConfig
  aggregation: Aggregation
}

input VisualizationConfig {
  preferredTypes: [String!]!
  filters: [Filter!]!
  aggregation: Aggregation
}

input Filter {
  field: String!
  value: String!
  operator: String!
}

input Aggregation {
  function: String!
  field: String!
  groupBy: [String!]!
  timePeriod: String
}

# Example query
query ExampleQuery {
  records {
    id
    value
    timestamp
  }
  insights(input: "Show sales trends", config: {
    agentType: "sales-agent",
    visualization: {
      preferredTypes: ["line"],
      filters: [{
        field: "category",
        value: "electronics",
        operator: "="
      }]
    }
  }) {
    title
    description
    visualization {
      kind
      series {
        name
        data
      }
    }
  }
}
```

## Security Features

### Authentication

- JWT-based authentication
- Role-based access control
- Token expiration (1 hour)
- Secure token storage
- Required headers:
  - `Authorization: Bearer <token>`
- Role-based permissions:
  - `query`: Read access
  - `admin`: Full access
  - `analytics`: Insights access

### Rate Limiting

- Per-endpoint rate limits
- Burst protection
- IP-based tracking
- Window-based limits
- Headers:
  - `X-RateLimit-Limit`
  - `X-RateLimit-Remaining`
  - `Retry-After`
- Default limits:
  - Queries: 100/minute
  - Subscriptions: 50/minute
  - Burst: 5/second

### Input Validation

- Query parameter validation
- Filter validation
- Aggregation validation
- SQL injection prevention
- Size limits
- Content validation
- Type checking

### Security Headers

- HSTS (Strict-Transport-Security)
- XSS protection (X-XSS-Protection)
- Frame protection (X-Frame-Options)
- Content type options (X-Content-Type-Options)
- CSP (Content-Security-Policy)
- CORS configuration
- Referrer policy
- Feature policy

## Configuration

### Environment Variables

```bash
# Required
AGENT_API_URL="https://api.x.ai/grok"
AGENT_API_KEY="your-api-key"
JWT_SECRET="your-secret-key"

# Optional
AGENT_RETRY_ATTEMPTS=3
AGENT_RETRY_DELAY_MS=1000
RATE_LIMIT_WINDOW=60
RATE_LIMIT_COUNT=100
BURST_LIMIT=5
DATABASE_URL="postgresql://user:pass@localhost/db"
CACHE_URL="redis://localhost:6379"
LOG_LEVEL="info"
```

### Logging

The application uses `env_logger` for logging. Set the log level via environment variable:

```bash
export RUST_LOG="info" # or "debug", "warn", "error"
export RUST_TRACING="info" # for distributed tracing
```

## Development

### Running Tests

```bash
cargo test --test unit
cargo test --test integration
```

### Running with Debug

```bash
RUST_LOG=debug cargo run
```

### Code Style

- Follow Rustfmt rules
- Use clippy for linting
- Maintain consistent error handling
- Document public APIs
- Use async/await consistently

## Performance Optimization

### Caching

- Query results caching
- Token caching
- Agent responses caching
- Data source metadata caching

### Connection Pooling

- Database connections
- Agent API connections
- WebSocket connections

### Memory Management

- Efficient data structures
- Proper resource cleanup
- Memory profiling support

## Monitoring and Metrics

### Prometheus Metrics

- Request metrics
- Error rates
- Response times
- Cache hits/misses
- Agent performance

### Tracing

- Distributed tracing
- Request tracing
- Error tracing
- Performance profiling

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

### Guidelines

- Write tests for new features
- Update documentation
- Follow code style
- Maintain compatibility
- Add performance metrics

## License

MIT License
