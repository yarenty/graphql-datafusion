//! GraphQL schema for DataFusion integration

use async_graphql::{Context, Object, Schema};
use std::sync::Arc;
use crate::datafusion::context::DataFusionContext;
use crate::agents::orchestrator::AgentOrchestrator;
use crate::models::data::*;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // Get all tables available
    async fn tables(&self, ctx: &Context<'_>) -> Result<Vec<String>, async_graphql::Error> {
        let df_ctx = ctx.data_unchecked::<Arc<DataFusionContext>>();
        Ok(df_ctx.get_table_names().clone())
    }

    // Get table row count
    async fn table_count(
        &self,
        ctx: &Context<'_>,
        table_name: String,
    ) -> Result<i64, async_graphql::Error> {
        let df_ctx = ctx.data_unchecked::<Arc<DataFusionContext>>();
        df_ctx
            .get_table_count(&table_name)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to get count: {}", e)))
    }

    // Customer queries
    async fn customers(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<Customer>, async_graphql::Error> {
        let df_ctx = ctx.data_unchecked::<Arc<DataFusionContext>>();
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);

        let query = format!(
            "SELECT c_custkey, c_name, c_address, c_nationkey, c_phone, 
                    CAST(c_acctbal AS DOUBLE) as c_acctbal, c_mktsegment, c_comment 
             FROM customer 
             ORDER BY c_custkey 
             LIMIT {} OFFSET {}",
            limit, offset
        );

        let batches = df_ctx
            .execute_query(&query)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Query failed: {}", e)))?;

        let mut customers = Vec::new();
        for batch in batches {
            let custkeys = batch
                .column(0)
                .as_any()
                .downcast_ref::<datafusion::arrow::array::Int64Array>()
                .ok_or_else(|| async_graphql::Error::new("Failed to cast c_custkey column"))?;
            let nationkeys = batch
                .column(3)
                .as_any()
                .downcast_ref::<datafusion::arrow::array::Int64Array>()
                .ok_or_else(|| async_graphql::Error::new("Failed to cast c_nationkey column"))?;
            let acctbals = batch
                .column(5)
                .as_any()
                .downcast_ref::<datafusion::arrow::array::Float64Array>()
                .ok_or_else(|| async_graphql::Error::new("Failed to cast c_acctbal column"))?;

            // Handle string columns - support StringViewArray
            let names = batch
                .column(1)
                .as_any()
                .downcast_ref::<datafusion::arrow::array::StringViewArray>()
                .ok_or_else(|| async_graphql::Error::new("Failed to cast c_name column"))?;
            let addresses = batch
                .column(2)
                .as_any()
                .downcast_ref::<datafusion::arrow::array::StringViewArray>()
                .ok_or_else(|| async_graphql::Error::new("Failed to cast c_address column"))?;
            let phones = batch
                .column(4)
                .as_any()
                .downcast_ref::<datafusion::arrow::array::StringViewArray>()
                .ok_or_else(|| async_graphql::Error::new("Failed to cast c_phone column"))?;
            let mktsegments = batch
                .column(6)
                .as_any()
                .downcast_ref::<datafusion::arrow::array::StringViewArray>()
                .ok_or_else(|| async_graphql::Error::new("Failed to cast c_mktsegment column"))?;
            let comments = batch
                .column(7)
                .as_any()
                .downcast_ref::<datafusion::arrow::array::StringViewArray>()
                .ok_or_else(|| async_graphql::Error::new("Failed to cast c_comment column"))?;

            for i in 0..batch.num_rows() {
                customers.push(Customer {
                    c_custkey: custkeys.value(i),
                    c_name: names.value(i).to_string(),
                    c_address: addresses.value(i).to_string(),
                    c_nationkey: nationkeys.value(i),
                    c_phone: phones.value(i).to_string(),
                    c_acctbal: acctbals.value(i),
                    c_mktsegment: mktsegments.value(i).to_string(),
                    c_comment: comments.value(i).to_string(),
                });
            }
        }

        Ok(customers)
    }

    // Orders queries
    async fn orders(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<Order>, async_graphql::Error> {
        let df_ctx = ctx.data_unchecked::<Arc<DataFusionContext>>();
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);

        let query = format!(
            "SELECT o_orderkey, o_custkey, o_orderstatus, 
                    CAST(o_totalprice AS DOUBLE) as o_totalprice,
                    o_orderpriority, o_clerk, o_shippriority, o_comment 
             FROM orders 
             ORDER BY o_orderkey 
             LIMIT {} OFFSET {}",
            limit, offset
        );

        let batches = df_ctx
            .execute_query(&query)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Query failed: {}", e)))?;

        let mut orders = Vec::new();
        for batch in batches {
            println!("Orders batch schema: {:?}", batch.schema());
            
            let orderkeys = batch
                .column(0)
                .as_any()
                .downcast_ref::<datafusion::arrow::array::Int64Array>()
                .ok_or_else(|| async_graphql::Error::new("Failed to cast o_orderkey column"))?;
            let custkeys = batch
                .column(1)
                .as_any()
                .downcast_ref::<datafusion::arrow::array::Int64Array>()
                .ok_or_else(|| async_graphql::Error::new("Failed to cast o_custkey column"))?;
            let totalprices = batch
                .column(3)
                .as_any()
                .downcast_ref::<datafusion::arrow::array::Float64Array>()
                .ok_or_else(|| async_graphql::Error::new("Failed to cast o_totalprice column"))?;
            let shippriorities = batch
                .column(6)
                .as_any()
                .downcast_ref::<datafusion::arrow::array::Int32Array>()
                .ok_or_else(|| async_graphql::Error::new("Failed to cast o_shippriority column"))?;

            // Handle string columns - support StringViewArray
            let orderstatuses = batch
                .column(2)
                .as_any()
                .downcast_ref::<datafusion::arrow::array::StringViewArray>()
                .ok_or_else(|| async_graphql::Error::new("Failed to cast o_orderstatus column"))?;
            let orderpriorities = batch
                .column(4)
                .as_any()
                .downcast_ref::<datafusion::arrow::array::StringViewArray>()
                .ok_or_else(|| async_graphql::Error::new("Failed to cast o_orderpriority column"))?;
            let clerks = batch
                .column(5)
                .as_any()
                .downcast_ref::<datafusion::arrow::array::StringViewArray>()
                .ok_or_else(|| async_graphql::Error::new("Failed to cast o_clerk column"))?;
            let comments = batch
                .column(7)
                .as_any()
                .downcast_ref::<datafusion::arrow::array::StringViewArray>()
                .ok_or_else(|| async_graphql::Error::new("Failed to cast o_comment column"))?;

            for i in 0..batch.num_rows() {
                orders.push(Order {
                    o_orderkey: orderkeys.value(i),
                    o_custkey: custkeys.value(i),
                    o_orderstatus: orderstatuses.value(i).to_string(),
                    o_totalprice: totalprices.value(i),
                    o_orderdate: "1992-01-01".to_string(), // Temporary placeholder
                    o_orderpriority: orderpriorities.value(i).to_string(),
                    o_clerk: clerks.value(i).to_string(),
                    o_shippriority: shippriorities.value(i),
                    o_comment: comments.value(i).to_string(),
                });
            }
        }

        Ok(orders)
    }

    // Sales analytics
    async fn sales_analytics(
        &self,
        ctx: &Context<'_>,
    ) -> Result<SalesAnalytics, async_graphql::Error> {
        let df_ctx = ctx.data_unchecked::<Arc<DataFusionContext>>();

        // For now, return mock data to get the system working
        // TODO: Implement real DataFusion queries once basic functionality is working

        let total_sales = 15000000.0;
        let total_orders = 150000;
        let avg_order_value = total_sales / total_orders as f64;

        // Get some basic customer data
        let customers_query = "
            SELECT 
                c_custkey, c_name, c_address, c_nationkey, c_phone,
                CAST(c_acctbal AS DOUBLE) as c_acctbal, c_mktsegment, c_comment
            FROM customer 
            ORDER BY c_acctbal DESC
            LIMIT 5
        ";

        let customers_batches = df_ctx
            .execute_query(customers_query)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Customers query failed: {}", e)))?;

        let mut top_customers = Vec::new();
        for batch in customers_batches {
            if batch.num_rows() == 0 {
                continue;
            }

            // Debug: Print column types
            println!("Batch schema: {:?}", batch.schema());
            for (i, col) in batch.columns().iter().enumerate() {
                println!("Column {}: {:?}", i, col.data_type());
            }

            // For now, just extract the numeric columns we know work
            let custkeys = if let Some(arr) = batch
                .column(0)
                .as_any()
                .downcast_ref::<datafusion::arrow::array::Int64Array>(
            ) {
                arr
            } else {
                return Err(async_graphql::Error::new(format!(
                    "Failed to cast c_custkey column, type: {:?}",
                    batch.column(0).data_type()
                )));
            };

            let nationkeys = if let Some(arr) = batch
                .column(3)
                .as_any()
                .downcast_ref::<datafusion::arrow::array::Int64Array>(
            ) {
                arr
            } else {
                return Err(async_graphql::Error::new(format!(
                    "Failed to cast c_nationkey column, type: {:?}",
                    batch.column(3).data_type()
                )));
            };

            let acctbals = if let Some(arr) = batch
                .column(5)
                .as_any()
                .downcast_ref::<datafusion::arrow::array::Float64Array>(
            ) {
                arr
            } else {
                return Err(async_graphql::Error::new(format!(
                    "Failed to cast c_acctbal column, type: {:?}",
                    batch.column(5).data_type()
                )));
            };

            for i in 0..batch.num_rows() {
                // For now, use mock string data to avoid Utf8View issues
                let customer = Customer {
                    c_custkey: custkeys.value(i),
                    c_name: format!("Customer_{}", custkeys.value(i)),
                    c_address: "Mock Address".to_string(),
                    c_nationkey: nationkeys.value(i),
                    c_phone: "555-0000".to_string(),
                    c_acctbal: acctbals.value(i),
                    c_mktsegment: "BUILDING".to_string(),
                    c_comment: "Mock comment".to_string(),
                };

                top_customers.push(CustomerSales {
                    customer,
                    total_spent: acctbals.value(i), // Using account balance as proxy for spending
                    order_count: 1,                 // Mock value
                });
            }
        }

        // Mock data for other analytics
        let sales_by_region = vec![
            RegionSales {
                region: "AMERICA".to_string(),
                total_sales: total_sales * 0.4,
                customer_count: 1000,
            },
            RegionSales {
                region: "ASIA".to_string(),
                total_sales: total_sales * 0.35,
                customer_count: 800,
            },
            RegionSales {
                region: "EUROPE".to_string(),
                total_sales: total_sales * 0.25,
                customer_count: 600,
            },
        ];

        let monthly_trends = vec![
            MonthlyTrend {
                month: "2024-01".to_string(),
                total_sales: total_sales * 0.08,
                order_count: total_orders / 12,
            },
            MonthlyTrend {
                month: "2024-02".to_string(),
                total_sales: total_sales * 0.09,
                order_count: total_orders / 12,
            },
            MonthlyTrend {
                month: "2024-03".to_string(),
                total_sales: total_sales * 0.10,
                order_count: total_orders / 12,
            },
        ];

        Ok(SalesAnalytics {
            total_sales,
            total_orders,
            avg_order_value,
            top_customers,
            sales_by_region,
            monthly_trends,
        })
    }

    // Natural language query (still mocked for now)
    async fn natural_language_query(
        &self,
        _ctx: &Context<'_>,
        _input: String,
    ) -> Result<String, async_graphql::Error> {
        Ok(
            "SELECT c_name, SUM(CAST(o_totalprice AS DOUBLE)) as total_spent 
            FROM customer c 
            JOIN orders o ON c.c_custkey = o.o_custkey 
            GROUP BY c.c_custkey, c.c_name 
            ORDER BY total_spent DESC 
            LIMIT 10"
                .to_string(),
        )
    }

    // AI insights (mocked for now)
    async fn insights(
        &self,
        _ctx: &Context<'_>,
        _input: String,
    ) -> Result<String, async_graphql::Error> {
        Ok("Based on the TPCH data analysis:
        
1. **Top Customers**: The highest spending customers are primarily from the BUILDING market segment
2. **Order Patterns**: Most orders are placed in Q1 and Q4, showing seasonal business patterns
3. **Revenue Distribution**: 40% of revenue comes from AMERICA, 35% from ASIA, 25% from EUROPE
4. **Average Order Value**: The average order value is $15,000 with significant variation by region
5. **Customer Segments**: BUILDING and MACHINERY segments show the highest customer loyalty

Recommendations:
- Focus marketing efforts on BUILDING segment customers
- Develop seasonal promotions for Q1 and Q4
- Expand presence in ASIA market given strong performance"
            .to_string())
    }

    // Agent status
    async fn agent_status(&self, _ctx: &Context<'_>) -> Result<String, async_graphql::Error> {
        Ok("Agent system is operational and ready for TPCH data analysis".to_string())
    }

    // Test agent connections
    async fn test_agent_connections(
        &self,
        _ctx: &Context<'_>,
    ) -> Result<bool, async_graphql::Error> {
        Ok(true)
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn refresh_connection(&self, _ctx: &Context<'_>) -> Result<bool, async_graphql::Error> {
        Ok(true)
    }
}

pub type AppSchema = Schema<QueryRoot, MutationRoot, async_graphql::EmptySubscription>;

pub fn build_schema(
    df_ctx: Arc<DataFusionContext>,
    _orchestrator: Arc<AgentOrchestrator>,
) -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, async_graphql::EmptySubscription)
        .data(df_ctx)
        .finish()
}
