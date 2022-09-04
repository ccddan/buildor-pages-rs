use aws_sdk_dynamodb::model::AttributeValue;
use error_stack::Context;
use serde::Serialize as Serializable;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use super::request::RequestError;

/* Required Env Var Error */
#[derive(Debug)]
pub struct RequiredEnvVarError {
    pub name: String,
}
impl RequiredEnvVarError {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}
impl fmt::Display for RequiredEnvVarError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(format!("Missing required env var: {}", self.name).as_str())
    }
}
impl Context for RequiredEnvVarError {}

/* Execution Error */
#[derive(Debug)]
pub struct ExecutionError;
impl fmt::Display for ExecutionError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(format!("Execution error").as_str())
    }
}
impl Context for ExecutionError {}

/* Common Error */
pub struct CommonError;
impl CommonError {
    pub fn generic(error: String) -> RequestError {
        RequestError {
            code: "CME00".to_string(),
            message: "Error".to_string(),
            details: error,
        }
    }

    pub fn schema_compliant(details: String) -> RequestError {
        RequestError {
            code: "CME01".to_string(),
            message: "Schema Compliant Error".to_string(),
            details,
        }
    }

    pub fn item_not_found(details: Option<String>) -> RequestError {
        RequestError {
            code: "CME02".to_string(),
            message: "Not Found Error".to_string(),
            details: match details {
                Some(details) => details,
                None => "Item not found".to_string(),
            },
        }
    }
}

pub trait AsDynamoDBAttributeValue {
    fn as_hashmap(&self) -> HashMap<String, AttributeValue>;
    fn as_attr(&self) -> AttributeValue;
}

/* Missing Model Property Error */
#[derive(Debug)]
pub struct MissingModelPropertyError {
    pub name: String,
}

impl MissingModelPropertyError {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}

impl fmt::Display for MissingModelPropertyError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(format!("Missing model property: {}", self.name).as_str())
    }
}

impl Context for MissingModelPropertyError {}

/* Common Result List Response */
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseGenericList<T: Serializable> {
    pub items: Vec<T>,
    pub count: usize,
}

impl<T: Serializable> ResponseGenericList<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            count: items.len(),
            items,
        }
    }
}
