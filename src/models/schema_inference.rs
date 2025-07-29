//! Schema inference for DataFusion tables

use datafusion::arrow::datatypes::{DataType, Schema as ArrowSchema};
use std::collections::HashMap;
use std::sync::Arc;

/// Schema inference for DataFusion tables
pub struct SchemaInference {
    schema_cache: HashMap<String, Arc<ArrowSchema>>,
}

impl SchemaInference {
    /// Create a new schema inference instance
    pub fn new() -> Self {
        Self {
            schema_cache: HashMap::new(),
        }
    }

    /// Cache a schema for a table
    pub fn cache_schema(&mut self, table_name: &str, schema: ArrowSchema) {
        self.schema_cache
            .insert(table_name.to_string(), Arc::new(schema));
    }

    /// Get a cached schema
    pub fn get_cached_schema(&self, table_name: &str) -> Option<Arc<ArrowSchema>> {
        self.schema_cache.get(table_name).cloned()
    }

    /// Convert Arrow data type to Rust type string
    pub fn arrow_type_to_rust_type(data_type: &DataType) -> String {
        match data_type {
            DataType::Int8 | DataType::Int16 | DataType::Int32 => "i32".to_string(),
            DataType::Int64 => "i64".to_string(),
            DataType::UInt8 | DataType::UInt16 | DataType::UInt32 => "u32".to_string(),
            DataType::UInt64 => "u64".to_string(),
            DataType::Float32 => "f32".to_string(),
            DataType::Float64 => "f64".to_string(),
            DataType::Utf8 => "String".to_string(),
            DataType::Boolean => "bool".to_string(),
            DataType::Timestamp(_, _) => "chrono::DateTime<chrono::Utc>".to_string(),
            DataType::Date32 | DataType::Date64 => "chrono::NaiveDate".to_string(),
            DataType::Time32(_) | DataType::Time64(_) => "chrono::NaiveTime".to_string(),
            DataType::Binary => "Vec<u8>".to_string(),
            DataType::LargeBinary => "Vec<u8>".to_string(),
            DataType::List(_) => "Vec<serde_json::Value>".to_string(),
            _ => "serde_json::Value".to_string(), // Fallback for unsupported types
        }
    }

    /// Convert snake_case to CamelCase
    pub fn to_camel_case(s: &str) -> String {
        let mut result = String::new();
        let mut capitalize = true;

        for c in s.chars() {
            if c == '_' {
                capitalize = true;
            } else if capitalize {
                result.push(c.to_ascii_uppercase());
                capitalize = false;
            } else {
                result.push(c);
            }
        }

        result
    }
}
