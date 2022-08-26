use aws_sdk_dynamodb::model::AttributeValue;
use error_stack::Report;
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::handlers::projects::ProjectParser;
use crate::models::codebuild::BuildInfo;
use crate::models::common::MissingModelPropertyError;

pub struct BuildInfoParser {}
impl BuildInfoParser {
    pub fn parse(
        item: HashMap<String, AttributeValue>,
    ) -> Result<BuildInfo, Report<MissingModelPropertyError>> {
        let uuid = match item.get("uuid") {
            Some(value) => value.as_s().unwrap().to_string(),
            None => return Err(Report::new(MissingModelPropertyError::new("uuid"))),
        };

        let build_number = match item.get("build_number") {
            Some(value) => match value.as_n().unwrap().parse() {
                Ok(value) => Some(value),
                Err(_) => Some(0),
            },
            None => return Err(Report::new(MissingModelPropertyError::new("build_number"))),
        };

        let start_time = match item.get("start_time") {
            Some(value) => match value.as_n().unwrap().parse() {
                Ok(value) => Some(value),
                Err(_) => Some(0),
            },
            None => return Err(Report::new(MissingModelPropertyError::new("start_time"))),
        };

        let end_time = match item.get("end_time") {
            Some(value) => match value.as_n().unwrap().parse() {
                Ok(value) => Some(value),
                Err(_) => Some(0),
            },
            None => return Err(Report::new(MissingModelPropertyError::new("end_time"))),
        };

        let current_phase = match item.get("current_phase") {
            Some(value) => Some(value.as_s().unwrap().to_string()),
            None => return Err(Report::new(MissingModelPropertyError::new("current_phase"))),
        };

        let build_status = match item.get("build_status") {
            Some(value) => Some(value.as_s().unwrap().to_string()),
            None => return Err(Report::new(MissingModelPropertyError::new("build_status"))),
        };

        Ok(BuildInfo {
            uuid,
            build_number,
            start_time,
            end_time,
            current_phase,
            build_status,
        })
    }

    pub fn json(
        item: HashMap<String, AttributeValue>,
    ) -> Result<Value, Report<MissingModelPropertyError>> {
        match ProjectParser::parse(item) {
            Ok(parsed) => Ok(json!(parsed)),
            Err(err) => Err(err),
        }
    }
}
