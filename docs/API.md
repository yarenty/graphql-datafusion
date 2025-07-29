# GraphQL DataFusion API Documentation

## 🚀 Overview

The **GraphQL DataFusion API** provides a powerful interface for querying and analyzing data with AI-powered insights. The API combines the performance of Apache DataFusion with the flexibility of GraphQL and the intelligence of local AI models via Ollama.

### Key Features

- **📊 Automatic Data Discovery**: Automatically discovers and loads data from files and directories
- **⚡ High Performance**: Apache DataFusion for fast SQL execution
- **🤖 AI Integration**: Ollama-powered natural language queries and insights
- **🔍 GraphQL Interface**: Type-safe, self-documenting API
- **📈 Business Analytics**: Built-in analytics and insights generation
- **🌐 Real-time**: Live data exploration and analysis

## 📊 Data Discovery

The API automatically discovers and loads data from configured directories:

### Supported Formats
- **CSV**: Comma-separated values
- **Parquet**: Columnar storage format
- **JSON**: JavaScript Object Notation
- **JSONL**: JSON Lines format

### Automatic Schema Inference
- **Column Types**: Automatically detects data types (string, integer, float, boolean, date)
- **Table Names**: Uses file names as table names
- **Relationships**: Discovers foreign key relationships between tables
- **Metadata**: Extracts table statistics and sample data

## 🔧 GraphQL Schema

### Core Query Types

```graphql
type Query {
  # Get available tables in the dataset
  tables: [String!]!
  
  # Get row count for a specific table
  table_count(tableName: String!): Int!
  
  # Get table schema information
  table_schema(tableName: String!): TableSchema!
  
  # Query data with pagination
  records(tableName: String!, limit: Int, offset: Int): [Record!]!
  
  # Comprehensive analytics
  analytics(tableName: String!): Analytics!
  
  # AI-powered natural language query translation
  naturalLanguageQuery(input: String!): String!
  
  # AI-generated business insights
  insights(input: String!): String!
  
  # Check AI agent status
  agentStatus: String!
}
```

### Data Types

#### Record
```graphql
type Record {
  # Dynamic fields based on discovered schema
  [fieldName: String!]: String!
}
```

#### Table Schema
```graphql
type TableSchema {
  tableName: String!
  columns: [Column!]!
  rowCount: Int!
  sampleData: [Record!]!
}

type Column {
  name: String!
  dataType: String!
  nullable: Boolean!
  description: String
}
```

#### Analytics
```graphql
type Analytics {
  totalRecords: Int!
  columnStats: [ColumnStats!]!
  relationships: [Relationship!]!
  insights: String!
}

type ColumnStats {
  columnName: String!
  dataType: String!
  uniqueValues: Int!
  nullCount: Int!
  minValue: String
  maxValue: String
  avgValue: String
}

type Relationship {
  sourceTable: String!
  sourceColumn: String!
  targetTable: String!
  targetColumn: String!
  relationshipType: String!
}
```

## 🎯 Query Examples

### Basic Data Queries

#### Get Available Tables
```graphql
query {
  tables
}
```

**Response:**
```json
{
  "data": {
    "tables": ["customers", "orders", "products", "suppliers"]
  }
}
```

#### Get Table Schema
```graphql
query {
  table_schema(tableName: "customers") {
    tableName
    rowCount
    columns {
      name
      dataType
      nullable
    }
    sampleData {
      id
      name
      email
    }
  }
}
```

#### Get Data Records
```graphql
query {
  records(tableName: "customers", limit: 3) {
    id
    name
    email
    created_at
  }
}
```

### Analytics Queries

#### Table Analytics
```graphql
query {
  analytics(tableName: "customers") {
    totalRecords
    columnStats {
      columnName
      dataType
      uniqueValues
      nullCount
    }
    relationships {
      sourceTable
      targetTable
      relationshipType
    }
  }
}
```

### AI-Powered Queries

#### Natural Language Query Translation
```graphql
query {
  naturalLanguageQuery(input: "show me top customers by spending")
}
```

**Response:**
```json
{
  "data": {
    "naturalLanguageQuery": "SELECT c.name, SUM(o.total_amount) as total_spent FROM customers c JOIN orders o ON c.id = o.customer_id GROUP BY c.id, c.name ORDER BY total_spent DESC LIMIT 10"
  }
}
```

#### AI-Generated Insights
```graphql
query {
  insights(input: "analyze customer spending patterns and identify trends")
}
```

**Response:**
```json
{
  "data": {
    "insights": "Based on the data analysis:\n\n1. **Top Customers**: The highest spending customers show consistent purchasing patterns\n2. **Seasonal Trends**: Most orders are placed in Q1 and Q4, showing seasonal business patterns\n3. **Revenue Distribution**: Clear patterns in revenue distribution across different segments\n4. **Customer Segments**: Different segments show varying levels of customer loyalty\n5. **Growth Opportunities**: Identified potential areas for business expansion\n\nRecommendations:\n- Focus marketing efforts on high-value customer segments\n- Develop seasonal promotions for peak ordering periods\n- Expand presence in underperforming segments"
  }
}
```

## 🔍 Advanced Query Patterns

### Dynamic Field Selection
```graphql
query {
  records(tableName: "customers", limit: 5) {
    # Fields are dynamically available based on discovered schema
    id
    name
    email
    # Additional fields will be available based on actual data
  }
}
```

### Cross-Table Analysis
```graphql
query {
  # Get relationships between tables
  analytics(tableName: "orders") {
    relationships {
      sourceTable
      targetTable
      relationshipType
    }
  }
}
```

### Data Quality Analysis
```graphql
query {
  analytics(tableName: "customers") {
    columnStats {
      columnName
      nullCount
      uniqueValues
      dataType
    }
  }
}
```

## 🚀 Performance Characteristics

### Response Times
- **Schema discovery**: 100-500ms per table
- **Basic queries**: 10-100ms
- **Analytics queries**: 500ms-5s
- **AI queries**: 1-5s

### Data Volume
- **Supported file sizes**: Up to 10GB per file
- **Total dataset size**: Limited by available memory
- **Concurrent queries**: Up to 100 concurrent users

### Memory Usage
- **Schema caching**: Automatic caching of discovered schemas
- **Query optimization**: Automatic query optimization
- **Resource management**: Automatic cleanup of temporary data

## 🔧 Error Handling

### GraphQL Errors
```json
{
  "errors": [
    {
      "message": "Table 'unknown_table' not found",
      "locations": [
        {
          "line": 2,
          "column": 3
        }
      ],
      "path": ["records"]
    }
  ],
  "data": null
}
```

### Common Error Types
- **Table not found**: Requested table doesn't exist
- **Schema inference errors**: Unable to determine data types
- **File access errors**: Permission or file format issues
- **AI service errors**: Ollama connection issues
- **Memory errors**: Dataset too large for available memory

### Error Recovery
- **Automatic retry**: For transient errors
- **Fallback responses**: For AI service failures
- **Graceful degradation**: Partial results when possible
- **Schema validation**: Automatic validation of discovered schemas

## 🛠️ Integration Examples

### JavaScript/TypeScript
```javascript
const query = `
  query {
    tables
  }
`;

const response = await fetch('http://localhost:8080/graphql', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
  },
  body: JSON.stringify({ query })
});

const data = await response.json();
console.log('Available tables:', data.data.tables);
```

### Python
```python
import requests
import json

query = """
query {
  table_schema(tableName: "customers") {
    tableName
    rowCount
    columns {
      name
      dataType
    }
  }
}
"""

response = requests.post(
    'http://localhost:8080/graphql',
    json={'query': query}
)

data = response.json()
schema = data['data']['table_schema']
print(f"Table {schema['tableName']} has {schema['rowCount']} rows")
```

### cURL
```bash
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "{ tables }"
  }'
```

## 📚 Best Practices

### Data Organization
1. **Use descriptive file names** - They become table names
2. **Organize by data type** - Group related files in directories
3. **Use consistent formats** - Standardize on CSV or Parquet
4. **Include headers** - CSV files should have column headers

### Query Optimization
1. **Use pagination** for large datasets
2. **Limit fields** to only what you need
3. **Cache results** for repeated queries
4. **Use analytics queries** for aggregated data

### AI Integration
1. **Be specific** in natural language queries
2. **Use insights** for business analysis
3. **Check agent status** before AI queries
4. **Handle timeouts** for long-running AI operations

### Error Handling
1. **Always check** for GraphQL errors
2. **Implement retry logic** for transient failures
3. **Provide fallbacks** for AI service outages
4. **Log errors** for debugging

## 🔗 Related Documentation

- [Configuration Guide](CONFIGURATION.md) - Server configuration options
- [Deployment Guide](DEPLOYMENT.md) - Production deployment instructions
- [Troubleshooting Guide](TROUBLESHOOTING.md) - Common issues and solutions
- [Examples](../examples/README.md) - Complete example applications
