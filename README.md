# GraphQL over DataFusion with Agentic AI

A modern GraphQL server built on top of Apache DataFusion with Agentic AI integration for natural language queries and insights.

## Features

- GraphQL API over DataFusion
- Natural language query support via Agentic AI
- Multi-agent orchestration
- Real-time insights generation
- Prometheus metrics
- JWT authentication

## Setup

1. Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone the repository:
```bash
git clone [repository-url]
cd graphql-datafusion
```

3. Install dependencies:
```bash
cargo build
```

4. Set environment variables:
```bash
export AGENT_API_URL="https://api.x.ai/grok"
export AGENT_API_KEY="your-api-key"
export RUST_LOG=info
export RUST_TRACING=info
```

5. Run the server:
```bash
cargo run
```

## Usage

Access the GraphQL playground at `http://localhost:8000/playground`:

```graphql
query {
  insights(input: "Show records with value > 100 and summarize trends") {
    insights
  }
}
```

## Testing

Run unit tests:
```bash
cargo test
```

Run integration tests:
```bash
cargo test --test integration
```

## Development

The project follows a modular structure:

```
src/
├── graphql/          # GraphQL schema and resolvers
├── datafusion/       # DataFusion integration
├── models/          # Data models
├── agents/          # Agentic AI integration
├── tracing/          # Tracing
```

## Dependencies

- tracing = "0.1"              # Tracing
- tracing-subscriber = "0.3"  # Tracing subscriber
- tracing-appender = "0.2"    # File appender
- env_logger = "0.11"         # For compatibility with DataFusion

## License

MIT
