//! Data structures for GraphQL DataFusion

use async_graphql::{InputObject, SimpleObject, Enum};
use serde::{Deserialize, Serialize};

// TPCH Data Models
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Customer {
    pub c_custkey: i64,
    pub c_name: String,
    pub c_address: String,
    pub c_nationkey: i64,
    pub c_phone: String,
    pub c_acctbal: f64,
    pub c_mktsegment: String,
    pub c_comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Order {
    pub o_orderkey: i64,
    pub o_custkey: i64,
    pub o_orderstatus: String,
    pub o_totalprice: f64,
    pub o_orderdate: String, // Date as string for GraphQL compatibility
    pub o_orderpriority: String,
    pub o_clerk: String,
    pub o_shippriority: i32,
    pub o_comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct LineItem {
    pub l_orderkey: i64,
    pub l_partkey: i64,
    pub l_suppkey: i64,
    pub l_linenumber: i32,
    pub l_quantity: f64,
    pub l_extendedprice: f64,
    pub l_discount: f64,
    pub l_tax: f64,
    pub l_returnflag: String,
    pub l_linestatus: String,
    pub l_shipdate: String,
    pub l_commitdate: String,
    pub l_receiptdate: String,
    pub l_shipinstruct: String,
    pub l_shipmode: String,
    pub l_comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Part {
    pub p_partkey: i64,
    pub p_name: String,
    pub p_mfgr: String,
    pub p_brand: String,
    pub p_type: String,
    pub p_size: i32,
    pub p_container: String,
    pub p_retailprice: f64,
    pub p_comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Supplier {
    pub s_suppkey: i64,
    pub s_name: String,
    pub s_address: String,
    pub s_nationkey: i32,
    pub s_phone: String,
    pub s_acctbal: f64,
    pub s_comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Nation {
    pub n_nationkey: i64,
    pub n_name: String,
    pub n_regionkey: i64,
    pub n_comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Region {
    pub r_regionkey: i64,
    pub r_name: String,
    pub r_comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct PartSupp {
    pub ps_partkey: i64,
    pub ps_suppkey: i64,
    pub ps_availqty: i32,
    pub ps_supplycost: f64,
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