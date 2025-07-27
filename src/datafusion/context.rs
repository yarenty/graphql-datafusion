use datafusion::prelude::*;
use std::sync::Arc;
use datafusion::arrow::record_batch::RecordBatch;

pub struct DataFusionContext {
    ctx: SessionContext,
    table_name: String,
    path: String

}

impl DataFusionContext {
    pub async fn new(path: &str, table_name: &str) -> Result<DataFusionContext, datafusion::error::DataFusionError> {
        let ctx = SessionContext::new();
        if path.endsWith(".csv") {
            ctx.register_csv(
                table_name,
                path,
                CsvReadOptions::default()
            ).await?;
        } else if path.endsWith(".parquet") {
            ctx.register_parquet(
                table_name,
                path,
                ParquetReadOptions::default()
            ).await?;
        }
               Ok(Self { ctx,
               table_name: table_name.to_string(),
               path: path.to_string()
               })
    }

    pub async fn execute_query(&self, query: &str) -> Result<Vec<RecordBatch>, datafusion::error::DataFusionError> {
    let df = self.ctx.sql(query).await?;
    df.collect().await
    }

    pub fn get_table_name(&self) -> &str {
        &self.table_name
    }

}