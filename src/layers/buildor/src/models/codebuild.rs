use aws_sdk_codebuild::{model::Build, output::StartBuildOutput};
use aws_sdk_dynamodb::model::AttributeValue;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

use super::common::AsDynamoDBAttributeValue;

pub enum BuildObject {
    Build(Build),
    Builds(Option<Vec<Build>>),
    StartBuildOutput(StartBuildOutput),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildInfo {
    pub uuid: String,
    #[serde(rename(serialize = "buildNumber"))]
    pub build_number: Option<i64>,
    #[serde(rename(serialize = "startTime"))]
    pub start_time: Option<i64>,
    #[serde(rename(serialize = "endTime"))]
    pub end_time: Option<i64>,
    #[serde(rename(serialize = "currentPhase"))]
    pub current_phase: Option<String>,
    #[serde(rename(serialize = "buildStatus"))]
    pub build_status: Option<String>,
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
