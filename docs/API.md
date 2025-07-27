# GraphQL DataFusion API Documentation

## Overview

The GraphQL DataFusion API provides a powerful interface for querying and analyzing data with AI-powered insights. The API is built on top of Apache DataFusion and integrates with Agentic AI services for natural language processing and insights generation.

## Authentication

### JWT Tokens

The API uses JWT (JSON Web Tokens) for authentication. All requests must include a valid JWT token in the Authorization header:

```http
Authorization: Bearer <token>
```

### Roles

- `query`: Basic query access
- `admin`: Full access
- `analytics`: Insights and visualization access

## Rate Limiting

### Query Limits

- 100 queries per minute per IP
- 5 queries per second burst limit
- Window-based tracking
- Headers:
  - `X-RateLimit-Limit`: Maximum number of requests
  - `X-RateLimit-Remaining`: Number of requests remaining
  - `Retry-After`: Seconds to wait before retrying

## GraphQL Schema

### Query Types

```graphql
type Query {
  # Get raw records from data source
  records: [Record!]!
  
  # Process natural language query
  naturalLanguageQuery(
    input: String!
    agentType: String
    limit: Int
    offset: Int
  ): (records: [Record!]!, insights: String!)
  
  # List available AI agents
  availableAgents: [String!]!
  
  # Generate insights
  insights(
    input: String!
    config: AgentConfig
  ): [Insight!]!
  
  # Translate natural language to SQL
  translateQuery(input: String!): String
  
  # Generate data visualization
  generateVisualization(
    input: String!
    config: VisualizationConfig
  ): Visualization!
}
```

### Subscription Types

```graphql
type Subscription {
  # Subscribe to insights updates
  insightsUpdates(query: String!): Insight!
  
  # Subscribe to agent status
  agentStatus(agentType: String!): AgentStatus!
  
  # Subscribe to agent metrics
  agentMetrics: AgentMetrics!
}
```

### Input Types

```graphql
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
```

### Example Queries

#### Basic Query

```graphql
query GetRecords {
  records {
    id
    value
    timestamp
  }
}
```

#### Natural Language Query

```graphql
query NaturalLanguageQuery {
  naturalLanguageQuery(
    input: "Show records with value > 100"
    agentType: "analysis-agent"
    limit: 50
    offset: 0
  ) {
    records {
      id
      value
    }
    insights
  }
}
```

#### Complex Query with Aggregation

```graphql
query ComplexQuery {
  naturalLanguageQuery(
    input: "Show monthly sales by category"
    agentType: "sales-agent"
    config: {
      aggregation: {
        function: "sum"
        field: "sales_amount"
        groupBy: ["category"]
        timePeriod: "month"
      }
    }
  ) {
    records {
      category
      month
      total_sales
    }
    insights
  }
}
```

#### Multiple Filters Query

```graphql
query FilteredQuery {
  naturalLanguageQuery(
    input: "Show high-value customers in Europe"
    agentType: "customer-agent"
    config: {
      visualization: {
        preferredTypes: ["bar"]
        filters: [
          {
            field: "region"
            value: "Europe"
            operator: "="
          },
          {
            field: "value"
            value: "1000"
            operator: ">"
          }
        ]
      }
    }
  ) {
    records {
      customer_id
      region
      total_value
    }
    insights
  }
}
```

#### Time Series Analysis

```graphql
query TimeSeries {
  naturalLanguageQuery(
    input: "Analyze weekly trends in customer engagement"
    agentType: "engagement-agent"
    config: {
      visualization: {
        preferredTypes: ["line"]
        filters: [
          {
            field: "date"
            value: "2024-01-01"
            operator: ">="
          },
          {
            field: "date"
            value: "2024-12-31"
            operator: "<="
          }
        ]
      }
    }
  ) {
    records {
      week
      engagement_score
    }
    insights
  }
}
```

#### Real-time Monitoring

```graphql
query RealTimeMonitoring {
  naturalLanguageQuery(
    input: "Show current system metrics"
    agentType: "monitoring-agent"
    config: {
      visualization: {
        preferredTypes: ["line", "gauge"]
      }
    }
  ) {
    records {
      metric_name
      current_value
      timestamp
    }
    insights
  }
}
```

#### Batch Processing

```graphql
query BatchProcessing {
  naturalLanguageQuery(
    input: "Process and analyze large dataset"
    agentType: "batch-agent"
    config: {
      visualization: {
        preferredTypes: ["heatmap"]
        filters: [
          {
            field: "status"
            value: "completed"
            operator: "="
          }
        ]
      }
    }
  ) {
    records {
      batch_id
      status
      completion_time
    }
    insights
  }
}
```

#### Custom Visualization

```graphql
query CustomVisualization {
  generateVisualization(
    input: "Show correlation between price and sales"
    config: {
      visualization: {
        preferredTypes: ["scatter"]
        filters: [
          {
            field: "product_type"
            value: "electronics"
            operator: "="
          }
        ]
      }
    }
  ) {
    title
    description
    visualization {
      kind
      series {
        name
        data
        xField
        yField
      }
    }
  }
}

## WebSocket API

### Example WebSocket Usage

#### Subscribe to Insights

```javascript
// Connect to WebSocket
const ws = new WebSocket('ws://localhost:8001/ws/insights/sales-analysis');

// Handle connection
ws.onopen = () => {
  console.log('Connected to insights stream');
};

// Handle messages
ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('New insight:', data);
};

// Handle errors
ws.onerror = (error) => {
  console.error('WebSocket error:', error);
};

// Handle close
ws.onclose = () => {
  console.log('Connection closed');
};
```

#### Subscribe to Agent Status

```javascript
// Connect to status WebSocket
const statusWs = new WebSocket('ws://localhost:8001/ws/status/sales-agent');

// Handle status updates
statusWs.onmessage = (event) => {
  const status = JSON.parse(event.data);
  console.log('Agent status:', status);
};
```

#### Error Handling

```javascript
// Error handling example
const ws = new WebSocket('ws://localhost:8001/ws/insights/error-case');

ws.onerror = (error) => {
  console.error('Connection error:', error);
  // Retry logic
  setTimeout(() => {
    ws.close();
    // Reconnect
  }, 5000);
};
```

## Error Handling Examples

### Common Errors

```json
{
  "errors": [
    {
      "message": "Invalid input parameters",
      "extensions": {
        "code": "INVALID_INPUT",
        "details": {
          "field": "value",
          "message": "Value must be greater than 0"
        }
      }
    }
  ]
}
```

```json
{
  "errors": [
    {
      "message": "Rate limit exceeded",
      "extensions": {
        "code": "RATE_LIMIT",
        "details": {
          "limit": 100,
          "window": 60,
          "remaining": 0
        }
      }
    }
  ]
}
```

```json
{
  "errors": [
    {
      "message": "Unauthorized access",
      "extensions": {
        "code": "UNAUTHORIZED",
        "details": {
          "required_role": "admin"
        }
      }
    }
  ]
}
```

## Performance Optimization Examples

### Query Optimization

```graphql
query OptimizedQuery {
  naturalLanguageQuery(
    input: "Show recent transactions"
    agentType: "transaction-agent"
    config: {
      optimization: {
        use_cache: true,
        batch_size: 100,
        parallel: true
      }
    }
  ) {
    records {
      id
      amount
      timestamp
    }
    insights
  }
}
```

### Caching Strategy

```graphql
query CachedQuery {
  naturalLanguageQuery(
    input: "Show most popular products"
    agentType: "product-agent"
    config: {
      cache: {
        ttl: 3600,
        key: "popular_products",
        refresh: true
      }
    }
  ) {
    records {
      product_id
      popularity_score
    }
    insights
  }
}
```

## Security Examples

### Role-based Access

```graphql
query AdminQuery {
  naturalLanguageQuery(
    input: "Show system configuration"
    agentType: "admin-agent"
    config: {
      security: {
        required_role: "admin",
        audit: true
      }
    }
  ) {
    records {
      config_key
      config_value
    }
    insights
  }
}
```

### Input Validation

```graphql
query ValidatedQuery {
  naturalLanguageQuery(
    input: "Show user activity"
    agentType: "activity-agent"
    config: {
      validation: {
        max_records: 1000,
        time_window: "1d",
        field_limits: {
          "user_id": { "type": "string", "max_length": 100 },
          "activity_type": { "type": "enum", "values": ["login", "logout", "action"] }
        }
      }
    }
  ) {
    records {
      user_id
      activity_type
      timestamp
    }
    insights
  }
}
```

### Endpoints

- `ws://localhost:8001/ws/insights/{query}`: Subscribe to insights updates
- `ws://localhost:8001/ws/status/{agent_type}`: Subscribe to agent status
- `ws://localhost:8001/ws/agents`: Monitor agent metrics

### Message Format

```json
{
  "type": "insight_update",
  "data": {
    "title": "Sales Trend",
    "description": "Monthly sales analysis",
    "value": 123456,
    "visualization": {
      "kind": "line",
      "series": [
        {
          "name": "Monthly Sales",
          "data": [1000, 1500, 2000, 2500]
        }
      ]
    }
  }
}
```

## Error Handling

### HTTP Status Codes

- 200: Success
- 400: Bad Request
- 401: Unauthorized
- 403: Forbidden
- 429: Too Many Requests
- 500: Internal Server Error

### GraphQL Error Format

```json
{
  "errors": [
    {
      "message": "Error message",
      "extensions": {
        "code": "ERROR_CODE",
        "details": "Additional details"
      }
    }
  ]
}
```

## Security Headers

### Request Headers

- `X-Content-Type-Options`: nosniff
- `X-Frame-Options`: DENY
- `X-XSS-Protection`: 1; mode=block
- `X-Permitted-Cross-Domain-Policies`: none
- `Strict-Transport-Security`: max-age=31536000; includeSubDomains; preload
- `Content-Security-Policy`: default-src 'self'; script-src 'self' 'unsafe-inline' 'unsafe-eval'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; connect-src 'self' ws: wss:

### Response Headers

- `Access-Control-Allow-Origin`: *
- `Access-Control-Allow-Methods`: GET, POST, OPTIONS
- `Access-Control-Allow-Headers`: Content-Type, Authorization

## Performance Optimization

### Caching

- Query results
- Token validation
- Agent responses
- Data source metadata

### Connection Pooling

- Database connections
- Agent API connections
- WebSocket connections

### Memory Management

- Efficient data structures
- Resource cleanup
- Memory profiling

## Monitoring

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
