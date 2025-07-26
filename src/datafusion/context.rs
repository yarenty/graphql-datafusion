use datafusion::prelude::*;
use std::sync::Arc;

pub struct DataFusionContext {
    ctx: SessionContext,
}

impl DataFusionContext {
    pub async fn new() -> Result {
        let ctx = SessionContext::new();
        // Register the Parquet data source - I use TPCH data for this example
        ctx.register_parquet("sample", "/opt/data/tpch/nation.parquet", ParquetReadOptions::default()).await?;
        Ok(Self { ctx })
    }

    pub async fn execute_query(&self, query: &str) -> Result, datafusion::error::DataFusionError> {
    let df = self.ctx.sql(query).await?;
    df.collect().await
    }
}