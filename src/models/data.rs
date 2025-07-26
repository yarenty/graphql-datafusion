use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResult {
    pub data: Vec<serde_json::Value>,
}


#[derive(Serialize, Deserialize)]
pub struct Record {
    pub id: i32,
    pub name: String,
    pub value: f64,
}