use datafusion::arrow::record_batch::RecordBatch;
use datafusion::prelude::*;

pub struct DataFusionContext {
    ctx: SessionContext,
    table_names: Vec<String>,
    data_path: String,
}

impl DataFusionContext {
    pub async fn new(
        data_path: &str,
    ) -> Result<DataFusionContext, datafusion::error::DataFusionError> {
        let ctx = SessionContext::new();
        let mut table_names = Vec::new();

        // Register all TPCH tables
        let tables = [
            "customer", "orders", "lineitem", "part", "supplier", "nation", "region", "partsupp",
        ];

        for table in &tables {
            let table_path = format!("{}/{}.parquet", data_path, table);
            ctx.register_parquet(*table, &table_path, ParquetReadOptions::default())
                .await?;
            table_names.push(table.to_string());
        }

        Ok(Self {
            ctx,
            table_names,
            data_path: data_path.to_string(),
        })
    }

    pub async fn execute_query(
        &self,
        query: &str,
    ) -> Result<Vec<RecordBatch>, datafusion::error::DataFusionError> {
        let df = self.ctx.sql(query).await?;
        df.collect().await
    }

    pub fn get_table_names(&self) -> &Vec<String> {
        &self.table_names
    }

    pub fn get_data_path(&self) -> &str {
        &self.data_path
    }

    // Helper method to get table row count
    pub async fn get_table_count(
        &self,
        table_name: &str,
    ) -> Result<i64, datafusion::error::DataFusionError> {
        let query = format!("SELECT COUNT(*) as count FROM {}", table_name);
        let batches = self.execute_query(&query).await?;

        if let Some(batch) = batches.first() {
            if let Some(count_array) = batch
                .column(0)
                .as_any()
                .downcast_ref::<datafusion::arrow::array::Int64Array>()
            {
                return Ok(count_array.value(0));
            }
        }

        Ok(0)
    }
}
