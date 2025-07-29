//! Data structures for GraphQL DataFusion

use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

/// Record structure representing a data row
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Record {
    pub id: i32,
    pub name: String,
    pub value: f64,
}

/// Query input parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryInput {
    pub query: String,
    pub agent_type: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// Query parameters for DataFusion
#[derive(Debug, Clone, Serialize, Deserialize, async_graphql::InputObject)]
pub struct QueryParams {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub filters: Option<Vec<Filter>>,
    pub sort_by: Option<String>,
    pub sort_order: Option<SortOrder>,
}

/// Filter for querying data
#[derive(Debug, Clone, Serialize, Deserialize, async_graphql::InputObject)]
pub struct Filter {
    pub field: String,
    pub operator: FilterOperator,
    pub value: String,
}

/// Filter operators
#[derive(Debug, Clone, Serialize, Deserialize, async_graphql::Enum, Copy, PartialEq, Eq)]
pub enum FilterOperator {
    Eq,
    Ne,
    Gt,
    Lt,
    Gte,
    Lte,
    Like,
    In,
}

impl std::fmt::Display for FilterOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterOperator::Eq => write!(f, "="),
            FilterOperator::Ne => write!(f, "!="),
            FilterOperator::Gt => write!(f, ">"),
            FilterOperator::Lt => write!(f, "<"),
            FilterOperator::Gte => write!(f, ">="),
            FilterOperator::Lte => write!(f, "<="),
            FilterOperator::Like => write!(f, "LIKE"),
            FilterOperator::In => write!(f, "IN"),
        }
    }
}

/// Sort order
#[derive(Debug, Clone, Serialize, Deserialize, async_graphql::Enum, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Asc,
    Desc,
}

impl std::fmt::Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortOrder::Asc => write!(f, "ASC"),
            SortOrder::Desc => write!(f, "DESC"),
        }
    }
}

/// Query result with metadata
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct QueryResult {
    pub records: Vec<Record>,
    pub total_count: i64,
    pub has_more: bool,
    pub query_time_ms: u64,
}

impl Default for QueryParams {
    fn default() -> Self {
        Self {
            limit: Some(100),
            offset: Some(0),
            filters: None,
            sort_by: None,
            sort_order: None,
        }
    }
}