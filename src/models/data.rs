//! Data structures for GraphQL DataFusion

use async_graphql::{Enum, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

// TPCH Data Models
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Customer {
    #[graphql(name = "c_custkey")]
    pub c_custkey: i64,
    #[graphql(name = "c_name")]
    pub c_name: String,
    #[graphql(name = "c_address")]
    pub c_address: String,
    #[graphql(name = "c_nationkey")]
    pub c_nationkey: i64,
    #[graphql(name = "c_phone")]
    pub c_phone: String,
    #[graphql(name = "c_acctbal")]
    pub c_acctbal: f64,
    #[graphql(name = "c_mktsegment")]
    pub c_mktsegment: String,
    #[graphql(name = "c_comment")]
    pub c_comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Order {
    #[graphql(name = "o_orderkey")]
    pub o_orderkey: i64,
    #[graphql(name = "o_custkey")]
    pub o_custkey: i64,
    #[graphql(name = "o_orderstatus")]
    pub o_orderstatus: String,
    #[graphql(name = "o_totalprice")]
    pub o_totalprice: f64,
    #[graphql(name = "o_orderdate")]
    pub o_orderdate: String, // Date as string for GraphQL compatibility
    #[graphql(name = "o_orderpriority")]
    pub o_orderpriority: String,
    #[graphql(name = "o_clerk")]
    pub o_clerk: String,
    #[graphql(name = "o_shippriority")]
    pub o_shippriority: i32,
    #[graphql(name = "o_comment")]
    pub o_comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct LineItem {
    #[graphql(name = "l_orderkey")]
    pub l_orderkey: i64,
    #[graphql(name = "l_partkey")]
    pub l_partkey: i64,
    #[graphql(name = "l_suppkey")]
    pub l_suppkey: i64,
    #[graphql(name = "l_linenumber")]
    pub l_linenumber: i32,
    #[graphql(name = "l_quantity")]
    pub l_quantity: f64,
    #[graphql(name = "l_extendedprice")]
    pub l_extendedprice: f64,
    #[graphql(name = "l_discount")]
    pub l_discount: f64,
    #[graphql(name = "l_tax")]
    pub l_tax: f64,
    #[graphql(name = "l_returnflag")]
    pub l_returnflag: String,
    #[graphql(name = "l_linestatus")]
    pub l_linestatus: String,
    #[graphql(name = "l_shipdate")]
    pub l_shipdate: String,
    #[graphql(name = "l_commitdate")]
    pub l_commitdate: String,
    #[graphql(name = "l_receiptdate")]
    pub l_receiptdate: String,
    #[graphql(name = "l_shipinstruct")]
    pub l_shipinstruct: String,
    #[graphql(name = "l_shipmode")]
    pub l_shipmode: String,
    #[graphql(name = "l_comment")]
    pub l_comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Part {
    #[graphql(name = "p_partkey")]
    pub p_partkey: i64,
    #[graphql(name = "p_name")]
    pub p_name: String,
    #[graphql(name = "p_mfgr")]
    pub p_mfgr: String,
    #[graphql(name = "p_brand")]
    pub p_brand: String,
    #[graphql(name = "p_type")]
    pub p_type: String,
    #[graphql(name = "p_size")]
    pub p_size: i32,
    #[graphql(name = "p_container")]
    pub p_container: String,
    #[graphql(name = "p_retailprice")]
    pub p_retailprice: f64,
    #[graphql(name = "p_comment")]
    pub p_comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Supplier {
    #[graphql(name = "s_suppkey")]
    pub s_suppkey: i64,
    #[graphql(name = "s_name")]
    pub s_name: String,
    #[graphql(name = "s_address")]
    pub s_address: String,
    #[graphql(name = "s_nationkey")]
    pub s_nationkey: i32,
    #[graphql(name = "s_phone")]
    pub s_phone: String,
    #[graphql(name = "s_acctbal")]
    pub s_acctbal: f64,
    #[graphql(name = "s_comment")]
    pub s_comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Nation {
    #[graphql(name = "n_nationkey")]
    pub n_nationkey: i64,
    #[graphql(name = "n_name")]
    pub n_name: String,
    #[graphql(name = "n_regionkey")]
    pub n_regionkey: i64,
    #[graphql(name = "n_comment")]
    pub n_comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Region {
    #[graphql(name = "r_regionkey")]
    pub r_regionkey: i64,
    #[graphql(name = "r_name")]
    pub r_name: String,
    #[graphql(name = "r_comment")]
    pub r_comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct PartSupp {
    #[graphql(name = "ps_partkey")]
    pub ps_partkey: i64,
    #[graphql(name = "ps_suppkey")]
    pub ps_suppkey: i64,
    #[graphql(name = "ps_availqty")]
    pub ps_availqty: i32,
    #[graphql(name = "ps_supplycost")]
    pub ps_supplycost: f64,
    #[graphql(name = "ps_comment")]
    pub ps_comment: String,
}

// Query Input Types
#[derive(Debug, Clone, Serialize, Deserialize, InputObject)]
pub struct QueryInput {
    pub table: String,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, InputObject)]
pub struct QueryParams {
    pub filters: Option<Vec<Filter>>,
    pub sort_by: Option<String>,
    pub sort_order: Option<SortOrder>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, InputObject)]
pub struct Filter {
    pub column: String,
    pub operator: FilterOperator,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Enum, Copy, PartialEq, Eq)]
pub enum FilterOperator {
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
    Like,
    In,
}

#[derive(Debug, Clone, Serialize, Deserialize, Enum, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Asc,
    Desc,
}

// Query Results
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct CustomerQueryResult {
    pub data: Vec<Customer>,
    pub total_count: i64,
    pub has_more: bool,
    pub page_info: PageInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct OrderQueryResult {
    pub data: Vec<Order>,
    pub total_count: i64,
    pub has_more: bool,
    pub page_info: PageInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct PageInfo {
    pub current_page: i32,
    pub page_size: i32,
    pub total_pages: i32,
}

// Analytics Results
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct SalesAnalytics {
    pub total_sales: f64,
    pub total_orders: i64,
    pub avg_order_value: f64,
    pub top_customers: Vec<CustomerSales>,
    pub sales_by_region: Vec<RegionSales>,
    pub monthly_trends: Vec<MonthlyTrend>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct CustomerSales {
    pub customer: Customer,
    pub total_spent: f64,
    pub order_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct RegionSales {
    pub region: String,
    pub total_sales: f64,
    pub customer_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct MonthlyTrend {
    pub month: String,
    pub total_sales: f64,
    pub order_count: i64,
}

// Implement Display for enums
impl std::fmt::Display for FilterOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterOperator::Eq => write!(f, "="),
            FilterOperator::Ne => write!(f, "!="),
            FilterOperator::Gt => write!(f, ">"),
            FilterOperator::Gte => write!(f, ">="),
            FilterOperator::Lt => write!(f, "<"),
            FilterOperator::Lte => write!(f, "<="),
            FilterOperator::Like => write!(f, "LIKE"),
            FilterOperator::In => write!(f, "IN"),
        }
    }
}

impl std::fmt::Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortOrder::Asc => write!(f, "ASC"),
            SortOrder::Desc => write!(f, "DESC"),
        }
    }
}
