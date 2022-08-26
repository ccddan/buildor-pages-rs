use aws_sdk_codebuild::output::StartBuildOutput;
use aws_sdk_dynamodb::model::AttributeValue;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

use super::common::AsDynamoDBAttributeValue;
use crate::utils::get_build_info;

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildInfo {
    pub uuid: String,
    pub build_number: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub current_phase: Option<String>,
    pub build_status: Option<String>,
}

impl BuildInfo {
    pub fn new(build: &StartBuildOutput) -> Self {
        get_build_info(build).unwrap()
    }
}

impl AsDynamoDBAttributeValue for BuildInfo {
    fn as_hashmap(&self) -> HashMap<String, AttributeValue> {
        let mut map: HashMap<String, AttributeValue> = HashMap::new();
        map.insert("uuid".to_string(), AttributeValue::S(self.uuid.to_owned()));
        map.insert(
            "build_number".to_string(),
            AttributeValue::N(format!("{}", self.build_number.unwrap_or(0))),
        );
        map.insert(
            "start_time".to_string(),
            AttributeValue::N(format!("{}", self.start_time.unwrap_or(0))),
        );
        map.insert(
            "end_time".to_string(),
            AttributeValue::N(format!("{}", self.end_time.unwrap_or(0))),
        );
        map.insert(
            "current_phase".to_string(),
            AttributeValue::S(self.current_phase.to_owned().unwrap_or("-".to_string())),
        );
        map.insert(
            "build_status".to_string(),
            AttributeValue::S(self.build_status.to_owned().unwrap_or("-".to_string())),
        );

        map
    }

    fn as_attr(&self) -> AttributeValue {
        AttributeValue::M(self.as_hashmap())
    }
}
