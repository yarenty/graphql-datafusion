use async_graphql::{Object, SchemaBuilder, Result, Enum};
use datafusion::arrow::datatypes::{Schema as ArrowSchema, DataType};
use std::collections::HashMap;
use std::sync::Arc;

pub struct SchemaInference {
    datafusion_ctx: Arc<SessionContext>,
    schema_cache: HashMap<String, Arc<ArrowSchema>>,
}

impl SchemaInference {
    pub fn new(datafusion_ctx: Arc<SessionContext>) -> Self {
        Self {
            datafusion_ctx,
            schema_cache: HashMap::new(),
        }
    }

    pub async fn infer_schema(&mut self, table_name: &str) -> Result<Arc<ArrowSchema>> {
        if let Some(schema) = self.schema_cache.get(table_name) {
            return Ok(schema.clone());
        }

        // Get table schema from DataFusion
        let schema = self.datafusion_ctx
            .table(table_name)
            .await?
            .schema()
            .clone();

        self.schema_cache.insert(table_name.to_string(), Arc::new(schema.clone()));
        Ok(Arc::new(schema))
    }

    pub fn generate_graphql_type(&self, table_name: &str, schema: &ArrowSchema) -> String {
        format!(
            """
#[derive(SimpleObject)]
pub struct {} {{
    {}{}
}}
""",
            table_name.to_camel_case(),
            self.generate_fields(schema),
            if schema.fields().len() > 0 { "\n" } else { "" }
        )
    }

    fn generate_fields(&self, schema: &ArrowSchema) -> String {
        schema
            .fields()
            .iter()
            .map(|field| self.generate_field(field))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn generate_field(&self, field: &datafusion::arrow::datatypes::Field) -> String {
        let field_name = field.name();
        let rust_type = self.arrow_type_to_rust_type(field.data_type());
        format!("    pub {}: {},", field_name, rust_type)
    }

    fn arrow_type_to_rust_type(&self, data_type: &DataType) -> String {
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
            DataType::Struct(fields) => {
                let struct_name = format!("{}Struct", fields[0].name());
                format!("Box<{}>", struct_name)
            }
            _ => "serde_json::Value".to_string(), // Fallback for unsupported types
        }
    }

    pub fn generate_graphql_schema(&self, table_name: &str, schema: &ArrowSchema) -> String {
        format!(
            """
#[derive(SimpleObject)]
pub struct {} {{
    {}{}
}}

#[derive(SimpleObject)]
pub struct {}Query {{
    {}(ctx: &Context<'_>) -> Result<Vec<{}>> {{
        let df_ctx = ctx.data_unchecked::<Arc<SessionContext>>();
        let df = df_ctx.sql(format!("SELECT * FROM {}"));
        let batches = df.collect().await?;
        let records = batches.into_iter().flat_map(|batch| {{
            let record = {}::from_arrow_batch(batch);
            Some(record)
        }}).collect();
        Ok(records)
    }}
}}
""",
            table_name.to_camel_case(),
            self.generate_fields(schema),
            if schema.fields().len() > 0 { "\n" } else { "" },
            table_name.to_camel_case(),
            table_name.to_camel_case(),
            table_name.to_camel_case(),
            table_name.to_camel_case(),
            table_name.to_camel_case()
        )
    }
}

// Extension trait for converting Arrow batches to Rust structs
pub trait FromArrowBatch {
    fn from_arrow_batch(batch: RecordBatch) -> Self;
}

// Helper trait for converting Arrow types to Rust types
pub trait ArrowTypeToRust {
    fn to_rust_type(&self) -> String;
}
