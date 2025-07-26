use async_graphql::
    {InputObject, SimpleObject, Result};
use serde::{Serialize, Deserialize};
use crate::models::data::Record;

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Insight {
    pub title: String,
    pub description: String,
    pub value: Option<String>,
    pub visualization: Option<Visualization>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Visualization {
    #[graphql(desc = "Type of visualization (e.g., bar, line, pie)")]
    pub kind: String,
    #[graphql(desc = "Data series for the visualization")]
    pub series: Vec<Series>,
    #[graphql(desc = "Visualization configuration options")]
    pub options: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Series {
    pub name: String,
    pub data: Vec<f64>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, InputObject)]
pub struct AgentConfig {
    #[graphql(desc = "Type of agent to use")]
    pub agent_type: String,
    #[graphql(desc = "Optional visualization configuration")]
    pub visualization: Option<VisualizationConfig>,
    #[graphql(desc = "Optional filtering criteria")]
    pub filters: Option<Vec<Filter>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, InputObject)]
pub struct VisualizationConfig {
    #[graphql(desc = "Preferred visualization types")]
    pub preferred_types: Option<Vec<String>>,
    #[graphql(desc = "Data aggregation settings")]
    pub aggregation: Option<Aggregation>,
    #[graphql(desc = "Color scheme preferences")]
    pub colors: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, InputObject)]
pub struct Aggregation {
    #[graphql(desc = "Time period for aggregation")]
    pub time_period: Option<String>,
    #[graphql(desc = "Aggregation function (e.g., sum, avg)")]
    pub function: Option<String>,
    #[graphql(desc = "Grouping columns")]
    pub group_by: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, InputObject)]
pub struct Filter {
    pub field: String,
    pub operator: String,
    pub value: String,
}
