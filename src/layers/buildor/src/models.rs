use aws_sdk_dynamodb::model::AttributeValue;
use std::collections::HashMap;

pub mod common;
pub mod project;
pub mod request;
pub mod response;
pub mod user;

pub trait AsDynamoDBAttributeValue {
    fn as_hashmap(&self) -> HashMap<String, AttributeValue>;
    fn as_attr(&self) -> AttributeValue;
}
