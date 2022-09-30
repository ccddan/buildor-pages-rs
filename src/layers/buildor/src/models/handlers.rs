use async_trait::async_trait;
use aws_sdk_dynamodb::{model::AttributeValue, Client};
use chrono::Utc;
use error_stack::{Context, Report};
use std::collections::HashMap;
use std::fmt;

use crate::models::common::AsDynamoDBAttributeValue;

#[derive(Debug, Clone)]
pub struct HandlerUpdateExpressions {
    pub attribute_names: HashMap<String, String>,
    pub attribute_values: HashMap<String, AttributeValue>,
    pub update_expression: String,
}

#[derive(Debug)]
pub struct HandlerError {
    pub msg: String,
}

impl HandlerError {
    pub fn new(message: &str) -> Self {
        Self {
            msg: String::from(message),
        }
    }
}

impl fmt::Display for HandlerError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(format!("Handler create error: {}", self.msg).as_str())
    }
}

impl Context for HandlerError {}

pub trait HandlerInit {
    fn new(table_name: String, client: Client) -> Self;
}

#[async_trait]
pub trait HandlerCreate<T, PC, CE> {
    /// T = Main handler type (Project, User, etc.)
    /// PC = Payload Create. Payload to create objects of type T.
    /// CE = Create error
    async fn create(&self, payload: PC) -> Result<T, Report<CE>>;
}

#[async_trait]
pub trait HandlerGet<T, GE> {
    /// T = Main handler type (Project, User, etc.)
    /// GE = Update error
    async fn get(&self, uuid: String) -> Result<Option<T>, Report<GE>>;
}

#[async_trait]
pub trait HandlerList<T, LE> {
    /// T = Main handler type (Project, User, etc.)
    /// LE = List error
    async fn list(&self) -> Result<Vec<T>, Report<LE>>;
}

#[async_trait]
pub trait HandlerUpdate<T, PU: AsDynamoDBAttributeValue, UE> {
    /// T = Main handler type (Project, User, etc.)
    /// PU = Payload update. Payload to update object.
    /// UE = Update error
    async fn update(&self, uuid: String, payload: PU) -> Result<(), Report<UE>>;

    fn get_update_expressions(&self, payload: PU) -> HandlerUpdateExpressions {
        let mut attribute_names: HashMap<String, String> = HashMap::new();
        let mut attribute_values: HashMap<String, AttributeValue> = HashMap::new();
        let mut update_expression = String::from("SET ");

        // Default values
        let timestamp = Utc::now().to_rfc3339().to_string();
        attribute_names.insert("#updated_at".to_string(), "updated_at".to_string());
        attribute_values.insert(":updated_at".to_string(), AttributeValue::S(timestamp));
        update_expression.push_str("#updated_at = :updated_at, ");

        let map = payload.as_hashmap();
        for (k, v) in map.iter() {
            attribute_names.insert(format!("#{k}"), k.to_string());
            attribute_values.insert(format!(":{k}"), v.clone());
            update_expression.push_str(format!("#{prop} = :{prop}, ", prop = k).as_str());
        }
        update_expression.truncate(update_expression.len() - 2); // remove last ", "

        HandlerUpdateExpressions {
            attribute_names,
            attribute_values,
            update_expression,
        }
    }
}

#[async_trait]
pub trait HandlerDelete<T, DE> {
    /// T = Main handler type (Project, User, etc.)
    /// DE = Delete error
    async fn delete(&self, uuid: String) -> Result<bool, Report<DE>>;
}
