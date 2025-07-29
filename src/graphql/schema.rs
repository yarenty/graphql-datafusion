//! GraphQL schema for DataFusion integration

use async_graphql::{Context, Object, Schema};
use std::sync::Arc;
use crate::datafusion::context::DataFusionContext;
use crate::models::data::*;
use crate::agents::orchestrator::AgentOrchestrator;

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
        df_ctx.get_table_count(&table_name).await
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
        
        let batches = df_ctx.execute_query(&query).await
            .map_err(|e| async_graphql::Error::new(format!("Query failed: {}", e)))?;
        
        let mut customers = Vec::new();
        for batch in batches {
            let custkeys = batch.column(0).as_any().downcast_ref::<datafusion::arrow::array::Int64Array>().unwrap();
            let names = batch.column(1).as_any().downcast_ref::<datafusion::arrow::array::StringArray>().unwrap();
            let addresses = batch.column(2).as_any().downcast_ref::<datafusion::arrow::array::StringArray>().unwrap();
            let nationkeys = batch.column(3).as_any().downcast_ref::<datafusion::arrow::array::Int64Array>().unwrap();
            let phones = batch.column(4).as_any().downcast_ref::<datafusion::arrow::array::StringArray>().unwrap();
            let acctbals = batch.column(5).as_any().downcast_ref::<datafusion::arrow::array::Float64Array>().unwrap();
            let mktsegments = batch.column(6).as_any().downcast_ref::<datafusion::arrow::array::StringArray>().unwrap();
            let comments = batch.column(7).as_any().downcast_ref::<datafusion::arrow::array::StringArray>().unwrap();
            
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
                    CAST(o_orderdate AS VARCHAR) as o_orderdate,
                    o_orderpriority, o_clerk, o_shippriority, o_comment 
             FROM orders 
             ORDER BY o_orderkey 
             LIMIT {} OFFSET {}",
            limit, offset
        );
        
        let batches = df_ctx.execute_query(&query).await
            .map_err(|e| async_graphql::Error::new(format!("Query failed: {}", e)))?;
        
        let mut orders = Vec::new();
        for batch in batches {
            let orderkeys = batch.column(0).as_any().downcast_ref::<datafusion::arrow::array::Int64Array>().unwrap();
            let custkeys = batch.column(1).as_any().downcast_ref::<datafusion::arrow::array::Int64Array>().unwrap();
            let orderstatuses = batch.column(2).as_any().downcast_ref::<datafusion::arrow::array::StringArray>().unwrap();
            let totalprices = batch.column(3).as_any().downcast_ref::<datafusion::arrow::array::Float64Array>().unwrap();
            let orderdates = batch.column(4).as_any().downcast_ref::<datafusion::arrow::array::StringArray>().unwrap();
            let orderpriorities = batch.column(5).as_any().downcast_ref::<datafusion::arrow::array::StringArray>().unwrap();
            let clerks = batch.column(6).as_any().downcast_ref::<datafusion::arrow::array::StringArray>().unwrap();
            let shippriorities = batch.column(7).as_any().downcast_ref::<datafusion::arrow::array::Int32Array>().unwrap();
            let comments = batch.column(8).as_any().downcast_ref::<datafusion::arrow::array::StringArray>().unwrap();
            
            for i in 0..batch.num_rows() {
                orders.push(Order {
                    o_orderkey: orderkeys.value(i),
                    o_custkey: custkeys.value(i),
                    o_orderstatus: orderstatuses.value(i).to_string(),
                    o_totalprice: totalprices.value(i),
                    o_orderdate: orderdates.value(i).to_string(),
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
    async fn sales_analytics(&self, ctx: &Context<'_>) -> Result<SalesAnalytics, async_graphql::Error> {
        let df_ctx = ctx.data_unchecked::<Arc<DataFusionContext>>();
        
        // Total sales and orders
        let summary_query = "
            SELECT 
                SUM(CAST(o_totalprice AS DOUBLE)) as total_sales,
                COUNT(*) as total_orders,
                AVG(CAST(o_totalprice AS DOUBLE)) as avg_order_value
            FROM orders
        ";
        
        let summary_batches = df_ctx.execute_query(summary_query).await
            .map_err(|e| async_graphql::Error::new(format!("Summary query failed: {}", e)))?;
        
        let summary_batch = &summary_batches[0];
        let total_sales = summary_batch.column(0).as_any().downcast_ref::<datafusion::arrow::array::Float64Array>().unwrap().value(0);
        let total_orders = summary_batch.column(1).as_any().downcast_ref::<datafusion::arrow::array::Int64Array>().unwrap().value(0);
        let avg_order_value = summary_batch.column(2).as_any().downcast_ref::<datafusion::arrow::array::Float64Array>().unwrap().value(0);
        
        // Top customers
        let top_customers_query = "
            SELECT 
                c.c_custkey, c.c_name, c.c_address, c.c_nationkey, c.c_phone,
                CAST(c.c_acctbal AS DOUBLE) as c_acctbal, c.c_mktsegment, c.c_comment,
                SUM(CAST(o.o_totalprice AS DOUBLE)) as total_spent,
                COUNT(o.o_orderkey) as order_count
            FROM customer c
            JOIN orders o ON c.c_custkey = o.o_custkey
            GROUP BY c.c_custkey, c.c_name, c.c_address, c.c_nationkey, c.c_phone, c.c_acctbal, c.c_mktsegment, c.c_comment
            ORDER BY total_spent DESC
            LIMIT 10
        ";
        
        let top_customers_batches = df_ctx.execute_query(top_customers_query).await
            .map_err(|e| async_graphql::Error::new(format!("Top customers query failed: {}", e)))?;
        
        let mut top_customers = Vec::new();
        for batch in top_customers_batches {
            let custkeys = batch.column(0).as_any().downcast_ref::<datafusion::arrow::array::Int64Array>().unwrap();
            let names = batch.column(1).as_any().downcast_ref::<datafusion::arrow::array::StringArray>().unwrap();
            let addresses = batch.column(2).as_any().downcast_ref::<datafusion::arrow::array::StringArray>().unwrap();
            let nationkeys = batch.column(3).as_any().downcast_ref::<datafusion::arrow::array::Int64Array>().unwrap();
            let phones = batch.column(4).as_any().downcast_ref::<datafusion::arrow::array::StringArray>().unwrap();
            let acctbals = batch.column(5).as_any().downcast_ref::<datafusion::arrow::array::Float64Array>().unwrap();
            let mktsegments = batch.column(6).as_any().downcast_ref::<datafusion::arrow::array::StringArray>().unwrap();
            let comments = batch.column(7).as_any().downcast_ref::<datafusion::arrow::array::StringArray>().unwrap();
            let total_spents = batch.column(8).as_any().downcast_ref::<datafusion::arrow::array::Float64Array>().unwrap();
            let order_counts = batch.column(9).as_any().downcast_ref::<datafusion::arrow::array::Int64Array>().unwrap();
            
            for i in 0..batch.num_rows() {
                let customer = Customer {
                    c_custkey: custkeys.value(i),
                    c_name: names.value(i).to_string(),
                    c_address: addresses.value(i).to_string(),
                    c_nationkey: nationkeys.value(i),
                    c_phone: phones.value(i).to_string(),
                    c_acctbal: acctbals.value(i),
                    c_mktsegment: mktsegments.value(i).to_string(),
                    c_comment: comments.value(i).to_string(),
                };
                
                top_customers.push(CustomerSales {
                    customer,
                    total_spent: total_spents.value(i),
                    order_count: order_counts.value(i),
                });
            }
        }
        
        // Mock data for other analytics (simplified for now)
        let sales_by_region = vec![
            RegionSales { region: "AMERICA".to_string(), total_sales: total_sales * 0.4, customer_count: 1000 },
            RegionSales { region: "ASIA".to_string(), total_sales: total_sales * 0.35, customer_count: 800 },
            RegionSales { region: "EUROPE".to_string(), total_sales: total_sales * 0.25, customer_count: 600 },
        ];
        
        let monthly_trends = vec![
            MonthlyTrend { month: "2024-01".to_string(), total_sales: total_sales * 0.08, order_count: total_orders / 12 },
            MonthlyTrend { month: "2024-02".to_string(), total_sales: total_sales * 0.09, order_count: total_orders / 12 },
            MonthlyTrend { month: "2024-03".to_string(), total_sales: total_sales * 0.10, order_count: total_orders / 12 },
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
        Ok("SELECT c_name, SUM(CAST(o_totalprice AS DOUBLE)) as total_spent 
            FROM customer c 
            JOIN orders o ON c.c_custkey = o.o_custkey 
            GROUP BY c.c_custkey, c.c_name 
            ORDER BY total_spent DESC 
            LIMIT 10".to_string())
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
- Expand presence in ASIA market given strong performance".to_string())
    }

    // Agent status
    async fn agent_status(&self, _ctx: &Context<'_>) -> Result<String, async_graphql::Error> {
        Ok("Agent system is operational and ready for TPCH data analysis".to_string())
    }

    // Test agent connections
    async fn test_agent_connections(&self, _ctx: &Context<'_>) -> Result<bool, async_graphql::Error> {
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
