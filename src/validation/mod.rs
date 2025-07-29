use async_graphql::{Context, Error, InputObject, Result};
// use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

#[derive(Debug, Validate, InputObject)]
pub struct QueryInput {
    #[validate(length(min = 1, message = "Query cannot be empty"))]
    pub query: String,

    #[validate(length(min = 1, message = "Agent type cannot be empty"))]
    pub agent_type: Option<String>,

    #[validate(range(min = 1, max = 1000, message = "Limit must be between 1 and 1000"))]
    pub limit: Option<i32>,

    #[validate(range(min = 0, message = "Offset must be non-negative"))]
    pub offset: Option<i32>,
}

#[derive(Debug, Validate, InputObject)]
pub struct FilterInput {
    #[validate(length(min = 1, message = "Field cannot be empty"))]
    pub field: String,

    #[validate(length(min = 1, message = "Value cannot be empty"))]
    pub value: String,

    pub operator: String,
}

#[derive(Debug, Validate, InputObject)]
pub struct AggregationInput {
    #[validate(length(min = 1, message = "Function cannot be empty"))]
    pub function: String,

    #[validate(length(min = 1, message = "Field cannot be empty"))]
    pub field: String,

    #[validate(length(min = 1, message = "Group by field cannot be empty"))]
    pub group_by: Option<Vec<String>>,
}

pub fn validate_query_input(_ctx: &Context<'_>, input: QueryInput) -> Result<QueryInput> {
    input.validate().map_err(|e| {
        let mut errors = HashMap::new();
        for err in e.field_errors() {
            errors.insert(err.0.to_string(), err.1[0].clone());
        }
        Error::new(format!("Validation errors: {:?}", errors))
    })?;

    // Additional SQL injection prevention
    if input.query.contains(";--") || input.query.contains("/*") || input.query.contains("*/") {
        return Err(Error::new("Invalid characters in query"));
    }

    Ok(input)
}

pub fn validate_filter_input(_ctx: &Context<'_>, input: FilterInput) -> Result<FilterInput> {
    // input.validate().map_err(|e| {
    //     let mut errors = HashMap::new();
    //     for err in e.field_errors() {
    //         errors.insert(err.0.to_string(), err.1[0].clone());
    //     }
    //     Error::new(format!("Validation errors: {:?}", errors))
    // })?;

    // Validate operator
    let valid_operators = ["=", "!=", ">", "<", ">=", "<=", "LIKE"];
    if !valid_operators.contains(&input.operator.as_str()) {
        return Err(Error::new(format!(
            "Invalid operator. Must be one of: {}",
            valid_operators.join(", ")
        )));
    }

    Ok(input)
}

pub fn validate_aggregation_input(
    _ctx: &Context<'_>,
    input: AggregationInput,
) -> Result<AggregationInput> {
    input.validate().map_err(|e| {
        let mut errors = HashMap::new();
        for err in e.field_errors() {
            errors.insert(err.0.to_string(), err.1[0].clone());
        }
        Error::new(format!("Validation errors: {:?}", errors))
    })?;

    // Validate aggregation function
    let valid_functions = ["sum", "avg", "count", "min", "max"];
    if !valid_functions.contains(&input.function.to_lowercase().as_str()) {
        return Err(Error::new(format!(
            "Invalid aggregation function. Must be one of: {}",
            valid_functions.join(", ")
        )));
    }

    Ok(input)
}
