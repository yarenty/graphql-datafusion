use crate::datafusion::context::DataFusionContext;
use datafusion::prelude::*;

#[tokio::test]
async fn test_datafusion_query() {
    let ctx = DataFusionContext::new();
    let df = ctx.ctx.sql("SELECT 1").await.unwrap();
    let results = df.collect().await.unwrap();
    assert_eq!(results.len(), 1);
}
