# GraphQL DataFusion with AI-Powered Analytics

A Rust-based GraphQL server that combines Apache DataFusion for high-performance data processing with local AI models (Ollama) for intelligent data analysis and insights generation. This project provides a scalable, secure, and feature-rich solution for data exploration with AI-powered business intelligence.

## 🚀 Project Vision

### Phase 1: Foundation (Current)
- **TPCH Dataset Example**: Working implementation with the TPCH benchmark dataset
- **Core Infrastructure**: GraphQL API, DataFusion integration, Ollama AI
- **Basic Analytics**: Sales analytics, customer insights, order analysis
- **AI Integration**: Natural language to SQL translation, business insights generation

### Phase 2: Automatic Data Discovery (Next)
- **Universal Data Connector**: Point to any directory, database, or data source
- **Intelligent Schema Inference**: Automatically discover table structures and relationships
- **Multi-Format Support**: CSV, Parquet, JSON, JSONL, Excel, SQL databases
- **Metadata Store**: Centralized metadata management for discovered datasets

### Phase 3: Advanced Analytics & Intelligence (Future)
- **Automated Insights**: AI-driven pattern recognition and anomaly detection
- **Predictive Analytics**: Machine learning models for forecasting and trends
- **Data Lineage**: Track data origins, transformations, and dependencies
- **Collaborative Analytics**: Multi-user dashboards and shared insights

## ✨ Current Features (Phase 1)

### 📊 Data Processing
- **Apache DataFusion**: High-performance SQL query engine
- **TPCH Dataset**: 300MB+ benchmark dataset with realistic business data
- **Real-time Queries**: Fast response times for complex analytics
- **Memory Optimization**: Efficient data handling and caching

### 🤖 AI Integration
- **Ollama Integration**: Local AI models for privacy and performance
- **Natural Language Queries**: Convert plain English to SQL
- **Business Insights**: AI-generated analysis and recommendations
- **Intelligent Recommendations**: Suggest relevant queries and visualizations

### 🔍 GraphQL API
- **Type-safe Queries**: Self-documenting API with introspection
- **Real-time Analytics**: Live data exploration and analysis
- **Flexible Schema**: Dynamic field mapping and pagination
- **Error Handling**: Comprehensive error reporting and recovery

### 📈 Business Intelligence
- **Sales Analytics**: Revenue analysis, customer segmentation, order trends
- **Customer Insights**: Spending patterns, loyalty analysis, market segments
- **Performance Metrics**: Response times, throughput, resource utilization
- **Data Quality**: Schema validation, data type inference, relationship discovery

## 🛠️ Technology Stack

### Core Technologies
- **Rust**: High-performance, memory-safe systems programming
- **Apache DataFusion**: In-memory query engine with SQL support
- **GraphQL**: Type-safe API with real-time capabilities
- **Actix Web**: High-performance HTTP server framework

### AI & Analytics
- **Ollama**: Local large language model inference
- **Arrow**: Columnar memory format for efficient data processing
- **Tracing**: Distributed tracing and observability
- **Prometheus**: Metrics collection and monitoring

### Data Formats
- **CSV**: Comma-separated values with automatic schema inference
- **Parquet**: Columnar storage for high-performance analytics
- **JSON/JSONL**: Flexible data interchange formats
- **SQL Databases**: PostgreSQL, MySQL, SQLite support (planned)

## 🚀 Quick Start

### Prerequisites

- **Rust 1.80.0+**: [Install Rust](https://rustup.rs/)
- **Ollama**: [Install Ollama](https://ollama.ai/)
- **TPCH Data**: Available in `/opt/data/tpch/` (or configure your own)

### Installation

1. **Clone the repository**:
```bash
git clone <repository-url>
cd graphql-datafusion
```

2. **Install dependencies**:
```bash
cargo build --release
```

3. **Start Ollama and pull a model**:
```bash
# Start Ollama service
ollama serve

# Pull a model (in another terminal)
ollama pull llama2
```

4. **Configure environment**:
```bash
# Create .env file
cat > .env << EOF
DATA_PATH=/opt/data/tpch
OLLAMA_BASE_URL=http://localhost:11434
OLLAMA_MODEL=llama2
SERVER_PORT=8080
RUST_LOG=info
EOF
```

5. **Run the server**:
```bash
cargo run --release
```

6. **Access the API**:
- **GraphQL Playground**: http://localhost:8080/playground
- **Health Check**: http://localhost:8080/health
- **API Endpoint**: http://localhost:8080/graphql

## 📊 Example Queries

### Basic Data Exploration
```graphql
# Get available tables
query {
  tables
}

# Get customer data
query {
  customers(limit: 5) {
    c_custkey
    c_name
    c_acctbal
    c_mktsegment
  }
}

# Get sales analytics
query {
  salesAnalytics {
    totalSales
    totalOrders
    avgOrderValue
    topCustomers {
      customer {
        c_name
        c_mktsegment
      }
      totalSpent
      orderCount
    }
  }
}
```

### AI-Powered Analysis
```graphql
# Natural language to SQL
query {
  naturalLanguageQuery(input: "show me top customers by spending")
}

# AI-generated insights
query {
  insights(input: "analyze customer spending patterns and market segments")
}
```

## 🔧 Configuration

### Environment Variables
```bash
# Data Configuration
DATA_PATH=/path/to/data/directory
AUTO_DISCOVERY=true
SUPPORTED_FORMATS=csv,parquet,json,jsonl

# AI Configuration
OLLAMA_BASE_URL=http://localhost:11434
OLLAMA_MODEL=llama2
OLLAMA_TIMEOUT=30

# Server Configuration
SERVER_PORT=8080
HOST=0.0.0.0
WORKERS=4

# Performance Configuration
DATAFUSION_MEMORY_LIMIT=1073741824
DATAFUSION_BATCH_SIZE=8192
CACHE_ENABLED=true
```

### Configuration File
```toml
# config.toml
[data]
path = "/path/to/data"
auto_discovery = true
supported_formats = ["csv", "parquet", "json"]

[ai]
base_url = "http://localhost:11434"
model = "llama2"
timeout = 30

[server]
port = 8080
host = "0.0.0.0"
workers = 4
```

## 🐳 Docker Deployment

### Quick Start with Docker
```bash
# Build and run with Docker Compose
docker-compose up -d

# Or build manually
docker build -t graphql-datafusion .
docker run -p 8080:8080 -v /path/to/data:/data graphql-datafusion
```

### Docker Compose
```yaml
version: '3.8'
services:
  graphql-datafusion:
    build: .
    ports:
      - "8080:8080"
    volumes:
      - ./data:/data
    environment:
      - DATA_PATH=/data
      - OLLAMA_BASE_URL=http://ollama:11434
    depends_on:
      - ollama

  ollama:
    image: ollama/ollama:latest
    ports:
      - "11434:11434"
    volumes:
      - ollama_data:/root/.ollama
```

## 📈 Performance

### Benchmarks
- **Query Response**: 10-100ms for basic queries
- **Analytics Queries**: 500ms-2s for complex aggregations
- **AI Queries**: 1-5s for natural language processing
- **Concurrent Users**: Up to 100 concurrent users
- **Data Volume**: Supports datasets up to 10GB per file

### Optimization
- **Memory Management**: Automatic memory limits and cleanup
- **Query Optimization**: DataFusion's built-in optimizations
- **Caching**: Intelligent caching of query results
- **Batch Processing**: Efficient handling of large datasets

## 🔒 Security

### Built-in Security
- **Input Validation**: Comprehensive parameter validation
- **SQL Injection Prevention**: Safe query construction
- **Rate Limiting**: Configurable request limits
- **CORS Support**: Cross-origin resource sharing
- **Error Handling**: Secure error messages

### Production Security
- **HTTPS Support**: SSL/TLS encryption
- **Authentication**: JWT-based authentication (optional)
- **Authorization**: Role-based access control
- **Audit Logging**: Comprehensive activity logging

## 📚 Documentation

### API Documentation
- [Complete API Reference](docs/API.md) - GraphQL schema and queries
- [Configuration Guide](docs/CONFIGURATION.md) - Setup and configuration
- [Deployment Guide](docs/DEPLOYMENT.md) - Production deployment
- [Troubleshooting Guide](docs/TROUBLESHOOTING.md) - Common issues and solutions

### Examples
- [Basic Queries](../examples/src/basic_queries.rs) - Data exploration examples
- [AI Integration](../examples/src/ai_integration.rs) - AI-powered analytics
- [Advanced Analytics](../examples/src/advanced_analytics.rs) - Complex analytics

## 🔮 Roadmap

### Phase 2: Automatic Data Discovery (Q2 2024)
- [ ] **Universal Data Connector**
  - Directory scanning and file discovery
  - Database connection management
  - API endpoint integration
  - Real-time data source monitoring

- [ ] **Intelligent Schema Inference**
  - Automatic data type detection
  - Relationship discovery between tables
  - Schema evolution tracking
  - Data quality assessment

- [ ] **Metadata Store**
  - Centralized metadata management
  - Data lineage tracking
  - Schema versioning
  - Usage analytics

### Phase 3: Advanced Analytics & Intelligence (Q3 2024)
- [ ] **Automated Insights**
  - Pattern recognition algorithms
  - Anomaly detection
  - Trend analysis
  - Predictive modeling

- [ ] **Collaborative Analytics**
  - Multi-user dashboards
  - Shared insights and reports
  - Comment and annotation system
  - Export and sharing capabilities

- [ ] **Advanced AI Features**
  - Custom model training
  - Domain-specific insights
  - Natural language generation
  - Automated report creation

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup
```bash
# Clone and setup
git clone <repository-url>
cd graphql-datafusion

# Install development dependencies
cargo install cargo-watch
cargo install cargo-audit

# Run tests
cargo test

# Run with hot reload
cargo watch -x run
```

### Code Quality
- **Rust**: Follow Rust coding standards
- **Testing**: Comprehensive unit and integration tests
- **Documentation**: Clear code documentation
- **Security**: Regular security audits

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Apache DataFusion**: High-performance query engine
- **Ollama**: Local AI model inference
- **Actix Web**: High-performance web framework
- **TPCH**: Benchmark dataset for testing

## 📞 Support

- **Documentation**: [docs/](docs/) directory
- **Issues**: [GitHub Issues](https://github.com/your-repo/issues)
- **Discussions**: [GitHub Discussions](https://github.com/your-repo/discussions)
- **Email**: support@your-domain.com

---

**Ready to explore your data with AI?** Start with our [Quick Start Guide](#quick-start) or dive into the [API Documentation](docs/API.md).
