use aws_sdk_codebuild::Client;
use aws_sdk_codebuild::{model::Build, model::StatusType};
use aws_sdk_dynamodb::model::AttributeValue;
use error_stack::Report;
use log::{self, debug, error, info};
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::handlers::projects::ProjectParser;
use crate::models::codebuild::{BuildInfo, BuildObject};
use crate::models::common::MissingModelPropertyError;
use crate::models::handlers::HandlerError;

fn get_build_status(status: Option<&StatusType>) -> Option<String> {
    match status {
        Some(value) => Some(match value {
            StatusType::Failed => String::from("Failed"),
            StatusType::Fault => String::from("Fault"),
            StatusType::InProgress => String::from("InProgress"),
            StatusType::Stopped => String::from("Stopped"),
            StatusType::TimedOut => String::from("TimedOut"),
            StatusType::Succeeded => String::from("Succeeded"),
            _ => String::from("Unknown"),
        }),
        None => None,
    }
}

fn parse_build_info(build: &Build) -> Option<BuildInfo> {
    let uuid = build.id.to_owned().unwrap().split(":").last()?.to_string();
    let build_number = build.build_number;
    let start_time = match build.start_time() {
        Some(value) => Some(value.to_millis().unwrap()),
        None => None,
    };
    let end_time = match build.end_time() {
        Some(value) => Some(value.to_millis().unwrap()),
        None => None,
    };
    let current_phase = match build.current_phase() {
        Some(value) => Some(String::from(value)),
        None => None,
    };
    let build_status = get_build_status(build.build_status());

    Some(BuildInfo {
        uuid,
        build_number,
        start_time,
        end_time,
        current_phase,
        build_status,
    })
}

pub fn get_build_info(build: &BuildObject) -> Option<BuildInfo> {
    match build {
        BuildObject::Build(build) => parse_build_info(build),
        BuildObject::Builds(builds) => match builds {
            None => None,
            Some(builds) => {
                if builds.len() >= 1 {
                    parse_build_info(&builds[0])
                } else {
                    None
                }
            }
        },
        BuildObject::StartBuildOutput(build) => match build.build_value() {
            Some(build) => parse_build_info(build),
            None => None,
        },
    }
}

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

pub struct CodeBuildHandler {
    client: Client,
    project_name: String,
}

impl CodeBuildHandler {
    pub fn new(client: Client, project_name: String) -> Self {
        Self {
            client,
            project_name,
        }
    }

    pub async fn get(&self, id: String) -> Result<Option<BuildInfo>, Report<HandlerError>> {
        info!("CodeBuildHandler::get - id: {}", id);

        let mut ids: Vec<String> = Vec::new();
        ids.push(String::from(format!("{}:{}", self.project_name, id)));
        let tx = self.client.batch_get_builds().set_ids(Some(ids));

        match tx.send().await {
            Ok(result) => {
                debug!("CodeBuildHandler::get - get build response: {:?}", result);
                Ok(get_build_info(&BuildObject::Builds(result.builds)))
            }
            Err(error) => {
                error!("ProjectsHandler::create - failed to get build: {:?}", error);
                Err(Report::new(HandlerError::new(&error.to_string())))
            }
        }
    }
}
