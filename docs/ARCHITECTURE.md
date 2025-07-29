# GraphQL DataFusion Architecture

## Overview

GraphQL DataFusion is a modern data analytics platform that combines the power of Apache DataFusion for high-performance data processing with GraphQL for flexible API access and AI agents for intelligent insights generation. The system is designed to be modular, scalable, and easily extensible.

## High-Level Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   User/Client   │    │   AI Agent      │    │   External API  │
│                 │    │                 │    │                 │
│ • GraphQL       │    │ • Natural Lang  │    │ • Ollama        │
│ • REST API      │    │ • SQL Gen       │    │ • Other LLMs    │
│ • WebSocket     │    │ • Insights      │    │                 │
└─────────┬───────┘    └─────────┬───────┘    └─────────┬───────┘
          │                      │                      │
          └──────────────────────┼──────────────────────┘
                                 │
                    ┌─────────────▼─────────────┐
                    │    GraphQL Server         │
                    │   (Actix-Web + GraphQL)   │
                    │                           │
                    │ • Query Resolution        │
                    │ • Schema Management       │
                    │ • Authentication          │
                    │ • Rate Limiting           │
                    └─────────────┬─────────────┘
                                  │
                    ┌─────────────▼─────────────┐
                    │   Agent Orchestrator      │
                    │                           │
                    │ • Multi-Agent Management  │
                    │ • Request Routing         │
                    │ • Response Aggregation    │
                    │ • Error Handling          │
                    └─────────────┬─────────────┘
                                  │
                    ┌─────────────▼─────────────┐
                    │   DataFusion Engine       │
                    │                           │
                    │ • SQL Query Execution     │
                    │ • Data Processing         │
                    │ • Schema Inference        │
                    │ • Performance Optimization│
                    └─────────────┬─────────────┘
                                  │
                    ┌─────────────▼─────────────┐
                    │   Data Sources            │
                    │                           │
                    │ • CSV Files               │
                    │ • Parquet Files           │
                    │ • JSON/JSONL Files        │
                    │ • Databases (Future)      │
                    └───────────────────────────┘
```

## System Components

### 1. **HTTP Server Layer** (`src/server.rs`)
- **Purpose**: Main entry point for all HTTP requests
- **Technology**: Actix-Web framework
- **Key Functions**:
  - Request routing and middleware application
  - GraphQL endpoint management (`/graphql`)
  - Health check endpoint (`/health`)
  - CORS and security headers
  - Request/response logging

### 2. **GraphQL Layer** (`src/graphql/`)
- **Purpose**: GraphQL schema definition and query resolution
- **Technology**: async-graphql crate
- **Key Components**:
  - **Schema** (`src/graphql/schema.rs`): Defines all GraphQL types and resolvers
  - **Resolvers** (`src/graphql/resolvers.rs`): Business logic for query execution
  - **Helpers** (`src/graphql/helpers.rs`): Utility functions for data processing

### 3. **Agent Layer** (`src/agents/`)
- **Purpose**: AI-powered natural language processing and insights generation
- **Technology**: Ollama integration for local LLM inference
- **Key Components**:
  - **Client** (`src/agents/client.rs`): Direct communication with Ollama API
  - **Orchestrator** (`src/agents/orchestrator.rs`): Multi-agent coordination
  - **Types** (`src/agents/types.rs`): Agent-related data structures
  - **Config** (`src/agents/config.rs`): Agent configuration management

### 4. **DataFusion Layer** (`src/datafusion/`)
- **Purpose**: High-performance data processing and SQL execution
- **Technology**: Apache Arrow DataFusion
- **Key Components**:
  - **Context** (`src/datafusion/context.rs`): DataFusion session management
  - **Query Execution**: SQL parsing and execution engine
  - **Schema Management**: Dynamic schema inference and validation

### 5. **Data Models** (`src/models/`)
- **Purpose**: Data structures and type definitions
- **Key Components**:
  - **Data** (`src/models/data.rs`): Core data structures (Customer, Order, etc.)
  - **Schema Inference** (`src/models/schema_inference.rs`): Dynamic schema detection

### 6. **Configuration** (`src/config.rs`)
- **Purpose**: Centralized application configuration
- **Features**:
  - Environment variable support
  - Configuration validation
  - Default value management
  - Hot-reload capability (future)

### 7. **Middleware Layer** (`src/auth.rs`, `src/rate_limit.rs`, `src/security.rs`)
- **Purpose**: Cross-cutting concerns and security
- **Components**:
  - **Authentication**: JWT-based authentication (minimal implementation)
  - **Rate Limiting**: Request throttling (minimal implementation)
  - **Security**: Security headers and validation (minimal implementation)

## Detailed Request Flow

### 1. **Natural Language Query Flow**

```
User Request: "Show me top customers by spending"
```

#### Step 1: HTTP Request Reception
**File**: `src/server.rs`
```rust
// Request arrives at Actix-Web server
async fn graphql_handler(
    schema: web::Data<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
```

#### Step 2: GraphQL Schema Resolution
**File**: `src/graphql/schema.rs`
```rust
#[Object]
impl QueryRoot {
    async fn natural_language_query(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Natural language query")] input: String,
    ) -> Result<String, Error> {
        // Extract agent client from context
        let agent_client = ctx.data_unchecked::<AgentClient>();
        
        // Translate natural language to SQL
        let sql = agent_client.translate_to_sql(&input).await?;
        
        Ok(sql)
    }
}
```

#### Step 3: AI Agent Processing
**File**: `src/agents/client.rs`
```rust
pub async fn translate_to_sql(&self, input: &str) -> Result<String, Error> {
    let prompt = format!(
        "Translate this natural language query to SQL for TPCH database: '{}'. 
        Available tables: customer, orders, lineitem, part, supplier, nation, region, partsupp.
        Return only the SQL query, no explanations.",
        input
    );
    
    self.call_ollama(&prompt).await
}
```

#### Step 4: Ollama API Call
**File**: `src/agents/client.rs`
```rust
async fn call_ollama(&self, prompt: &str) -> Result<String, Error> {
    let request = OllamaRequest {
        model: self.model.clone(),
        prompt: prompt.to_string(),
        stream: false,
        options: Some(self.options.clone()),
    };

    let response = self.client
        .post(&format!("{}/api/generate", self.ollama_url))
        .json(&request)
        .send()
        .await?;

    let ollama_response: OllamaResponse = response.json().await?;
    Ok(ollama_response.response)
}
```

#### Step 5: Response Flow
The generated SQL is returned through the GraphQL response:
```json
{
  "data": {
    "naturalLanguageQuery": "SELECT c_custkey, c_name, c_acctbal FROM customer ORDER BY c_acctbal DESC LIMIT 10"
  }
}
```

### 2. **Data Query Flow**

```
User Request: GraphQL query for customer data
```

#### Step 1: GraphQL Query Resolution
**File**: `src/graphql/schema.rs`
```rust
async fn customers(
    &self,
    ctx: &Context<'_>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<CustomerQueryResult, Error> {
    let df_ctx = ctx.data_unchecked::<Arc<DataFusionContext>>();
    
    let limit = limit.unwrap_or(10);
    let offset = offset.unwrap_or(0);
    
    let query = format!(
        "SELECT c_custkey, c_name, c_address, c_nationkey, c_phone, 
                c_acctbal, c_mktsegment, c_comment 
         FROM customer 
         LIMIT {} OFFSET {}",
        limit, offset
    );
    
    let batches = df_ctx.execute_query(&query).await?;
    // Convert batches to Customer structs...
}
```

#### Step 2: DataFusion Query Execution
**File**: `src/datafusion/context.rs`
```rust
pub async fn execute_query(&self, query: &str) -> Result<Vec<RecordBatch>, DataFusionError> {
    let df = self.ctx.sql(query).await?;
    df.collect().await
}
```

#### Step 3: Data Conversion
**File**: `src/graphql/schema.rs`
```rust
// Convert DataFusion RecordBatch to Customer structs
let customers: Vec<Customer> = batches
    .into_iter()
    .flat_map(|batch| {
        let custkeys = batch.column(0).as_any().downcast_ref::<Int64Array>().unwrap();
        let names = batch.column(1).as_any().downcast_ref::<StringArray>().unwrap();
        // ... convert other columns
        
        (0..batch.num_rows()).map(move |i| Customer {
            c_custkey: custkeys.value(i),
            c_name: names.value(i).to_string(),
            // ... other fields
        }).collect::<Vec<_>>()
    })
    .collect();
```

### 3. **Analytics Query Flow**

```
User Request: "Get sales analytics"
```

#### Step 1: Analytics Query Resolution
**File**: `src/graphql/schema.rs`
```rust
async fn sales_analytics(&self, ctx: &Context<'_>) -> Result<SalesAnalytics, Error> {
    let df_ctx = ctx.data_unchecked::<Arc<DataFusionContext>>();
    
    // Execute multiple queries for analytics
    let total_sales_query = "SELECT SUM(o_totalprice) as total_sales FROM orders";
    let total_orders_query = "SELECT COUNT(*) as total_orders FROM orders";
    let avg_order_query = "SELECT AVG(o_totalprice) as avg_order_value FROM orders";
    
    // Execute queries and extract results...
    
    Ok(SalesAnalytics {
        total_sales: total_sales,
        total_orders: total_orders,
        avg_order_value: avg_order_value,
        top_customers: top_customers,
        sales_by_region: sales_by_region,
        monthly_trends: monthly_trends,
    })
}
```

#### Step 2: Complex Query Execution
**File**: `src/graphql/schema.rs`
```rust
// Example: Top customers query
let top_customers_query = r#"
    SELECT c_custkey, c_name, c_address, c_nationkey, c_phone, 
           CAST(c_acctbal AS DOUBLE) as c_acctbal, c_mktsegment, c_comment 
    FROM customer 
    ORDER BY c_acctbal DESC 
    LIMIT 5
"#;

let batches = df_ctx.execute_query(&top_customers_query).await?;
// Process results and create CustomerSales structs...
```

### 4. **AI Insights Generation Flow**

```
User Request: "Generate insights from customer data"
```

#### Step 1: Insights Query Resolution
**File**: `src/graphql/schema.rs`
```rust
async fn insights(
    &self,
    ctx: &Context<'_>,
    #[graphql(desc = "Natural language query")] input: String,
) -> Result<String, Error> {
    let agent_client = ctx.data_unchecked::<AgentClient>();
    
    // Get customer data for analysis
    let customers = self.customers(ctx, Some(100), Some(0)).await?;
    
    // Generate insights using AI
    let insights = agent_client.generate_insights(customers.data).await?;
    
    Ok(insights)
}
```

#### Step 2: AI Analysis
**File**: `src/agents/client.rs`
```rust
pub async fn generate_insights(&self, customers: Vec<Customer>) -> Result<String, Error> {
    if customers.is_empty() {
        return Ok("No data available for analysis.".to_string());
    }

    let summary = self.summarize_customers(&customers);
    let prompt = format!(
        "Analyze this customer data and provide business insights: {}",
        summary
    );
    
    self.call_ollama(&prompt).await
}
```

## Data Flow Architecture

### 1. **Request Processing Pipeline**

```
HTTP Request → Actix-Web → GraphQL Schema → Resolver → Agent/DataFusion → Response
```

### 2. **Data Processing Pipeline**

```
Raw Data (CSV/Parquet) → DataFusion Context → SQL Query → RecordBatch → GraphQL Type → JSON Response
```

### 3. **AI Processing Pipeline**

```
Natural Language → Agent Client → Ollama API → SQL/Insights → GraphQL Response
```

## Key Design Patterns

### 1. **Dependency Injection**
- GraphQL context provides access to DataFusion and Agent clients
- Services are injected through Actix-Web's data system

### 2. **Async/Await Pattern**
- All I/O operations are asynchronous
- Non-blocking request processing
- Efficient resource utilization

### 3. **Error Handling**
- Centralized error types
- Graceful degradation for external service failures
- Comprehensive error reporting

### 4. **Configuration Management**
- Environment-based configuration
- Type-safe configuration structs
- Default value handling

## Performance Considerations

### 1. **DataFusion Optimization**
- Columnar data processing
- Query optimization and caching
- Memory-efficient batch processing

### 2. **GraphQL Optimization**
- Field-level resolution
- Query complexity analysis
- Response caching (future)

### 3. **AI Integration**
- Connection pooling for Ollama
- Request batching
- Response caching

## Security Architecture

### 1. **Authentication** (Minimal Implementation)
- JWT-based authentication
- Token validation middleware
- Role-based access control (future)

### 2. **Rate Limiting** (Minimal Implementation)
- Request throttling
- Per-client rate limits
- Burst protection

### 3. **Input Validation**
- GraphQL schema validation
- SQL injection prevention
- Data sanitization

## Monitoring and Observability

### 1. **Logging**
- Structured logging with tracing
- Request/response logging
- Error tracking

### 2. **Metrics** (Future)
- Prometheus metrics
- Performance monitoring
- Health checks

### 3. **Tracing**
- Distributed tracing
- Request correlation
- Performance profiling

## Extension Points

### 1. **Data Sources**
- Database connectors
- Streaming data sources
- API integrations

### 2. **AI Models**
- Multiple LLM providers
- Model switching
- Custom model integration

### 3. **Analytics**
- Custom aggregation functions
- Machine learning integration
- Real-time analytics

## Future Architecture Enhancements

### 1. **Phase 2: Automatic Data Discovery**
- File system scanning
- Schema inference
- Metadata management

### 2. **Phase 3: Advanced Analytics**
- Machine learning pipelines
- Predictive analytics
- Automated insights

### 3. **Scalability**
- Horizontal scaling
- Load balancing
- Distributed processing

This architecture provides a solid foundation for building a powerful, scalable, and intelligent data analytics platform that can evolve with changing requirements and technologies. 