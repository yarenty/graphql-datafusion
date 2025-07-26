use regex::Regex;
use serde_json::from_str;
use crate::agents::types::{Insight, Visualization, Series};
use crate::models::data::Record;

pub fn parse_insights(
    insights_text: String,
    records: &[Record],
    config: &crate::agents::types::AgentConfig,
) -> Result<Vec<Insight>, String> {
    // Parse structured insights if available
    if let Ok(insights) = from_str::<Vec<Insight>>(&insights_text) {
        return Ok(insights);
    }

    // Fallback to parsing plain text
    let mut insights = Vec::new();
    
    // Split insights by newlines
    for line in insights_text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        
        // Try to extract title and description
        if let Some(caps) = Regex::new(r"^(.+?):\s+(.+)$")?.captures(line) {
            let title = caps.get(1).unwrap().as_str().to_string();
            let description = caps.get(2).unwrap().as_str().to_string();
            
            // Try to extract numeric value
            let value = Regex::new(r"\b\d+(?:\.\d+)?\b")?
                .captures(line)
                .map(|caps| caps.get(0).unwrap().as_str().to_string());
            
            insights.push(Insight {
                title,
                description,
                value,
                visualization: None,
                tags: vec![config.agent_type.clone()],
            });
        }
    }
    
    Ok(insights)
}

pub fn apply_filters(
    records: Vec<Record>,
    filters: &[crate::agents::types::Filter],
) -> Vec<Record> {
    records.into_iter()
        .filter(|record| {
            filters.iter().all(|filter| {
                match (record, &filter.operator, &filter.value) {
                    (Record { id, name, value }, "=", val) => 
                        name == val || value.to_string() == val,
                    (Record { id, name, value }, ">", val) => 
                        value > val.parse::<f64>().unwrap_or(0.0),
                    (Record { id, name, value }, "<", val) => 
                        value < val.parse::<f64>().unwrap_or(0.0),
                    _ => false,
                }
            })
        })
        .collect()
}
