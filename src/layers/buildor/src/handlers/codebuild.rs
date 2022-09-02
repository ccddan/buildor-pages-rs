use aws_sdk_codebuild::{
    model::{Build, EnvironmentVariable, EnvironmentVariableType, StatusType},
    Client,
};
use aws_sdk_dynamodb::model::AttributeValue;
use chrono::Utc;
use error_stack::Report;
use log::{self, debug, error, info};
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::handlers::projects::ProjectParser;
use crate::models::codebuild::{BuildInfo, BuildObject};
use crate::models::common::MissingModelPropertyError;
use crate::models::handlers::HandlerError;
use crate::models::project::Project;

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

    pub async fn create(
        &self,
        project: &Project,
    ) -> Result<Option<BuildInfo>, Report<HandlerError>> {
        info!("CodeBuildHandler::create - project: {:?}", project);
        let timestamp = Utc::now().to_string();

        debug!("CodeBuildHandler::create - create pre-build commands");
        let mut pre_build_commands = Vec::from_iter(project.commands.pre_build.iter());
        let command_cd_into_project = "cd $PROJECT_NAME".to_string();
        let command_pre_build_title = "####### Install Project Dependencies #######".to_string();
        pre_build_commands.insert(0, &command_cd_into_project);
        pre_build_commands.insert(0, &command_pre_build_title);

        debug!("CodeBuildHandler::create - parse pre-build commands as string");
        let pre_build_commands_str = pre_build_commands
            .iter()
            .map(|s| format!("\"{}\"", s.to_string()))
            .collect::<Vec<String>>()
            .join(",");

        debug!("CodeBuildHandler::create - create build commands");
        let mut build_commands = Vec::from_iter(project.commands.build.iter());
        let command_build_title = "echo Build project".to_string();
        let command_move_build_output = format!("mv {} ../dist", project.output_folder);
        build_commands.insert(0, &command_build_title);
        build_commands.push(&command_move_build_output);

        debug!("CodeBuildHandler::create - parse build commands as string");
        let build_commands_str = build_commands
            .iter()
            .map(|s| format!("\"{}\"", s.to_string()))
            .collect::<Vec<String>>()
            .join(",");

        let artifacts_output_name = format!("{}-dist-{}.zip", project.name, timestamp);
        debug!("CodeBuildHandler::create - parse buildspec");
        let build_spec = format!(
            r###"
            {{
              "version": "0.2",
              "env": {{
                "variables": {{
                  "MY_ENV_VAR": "value"
                }}
              }},
              "phases": {{
                "install": {{
                  "commands": [
                    "echo Download project",
                    "node -v",
                    "git clone $REPO_URL $PROJECT_NAME"
                  ]
                }},
                "pre_build": {{
                  "commands": [{pre_build_commands_str}]
                }},
                "build": {{
                  "commands": [{build_commands_str}]
                }},
                "post_build": {{
                  "commands": ["echo Build has completed and artifacts were moved"]
                }}
              }},
              "artifacts": {{
                "discard-paths": "no",
                "files": ["dist/**/*"],
                "name": "{artifacts_output_name}"
              }}
            }}
            "###
        );

        debug!("CodeBuildHandler::create - tx preparation");
        let tx = self
            .client
            .start_build()
            .project_name(self.project_name.to_string())
            .environment_variables_override(
                EnvironmentVariable::builder()
                    .set_name(Some("PROJECT_NAME".to_string()))
                    .set_value(Some(project.name.to_string()))
                    .set_type(Some(EnvironmentVariableType::Plaintext))
                    .build(),
            )
            .environment_variables_override(
                EnvironmentVariable::builder()
                    .set_name(Some("REPO_URL".to_string()))
                    .set_value(Some(project.repository.to_string()))
                    .set_type(Some(EnvironmentVariableType::Plaintext))
                    .build(),
            )
            .buildspec_override(build_spec);

        match tx.send().await {
            Ok(result) => {
                debug!("CodeBuildHandler::create - tx result: {:?}", result);
                debug!("CodeBuildHandler::create - parse build info");
                match get_build_info(&BuildObject::StartBuildOutput(result)) {
                    Some(build_info) => Ok(Some(build_info)),
                    None => {
                        debug!("CodeBuildHandler::create - code info parsing failed");
                        Err(Report::new(HandlerError::new(
                            "Failed to parse build result into BuildIngo",
                        )))
                    }
                }
            }
            Err(error) => {
                error!(
                    "CodeBuildHandler::create - failed to create build: {:?}",
                    error
                );
                Err(Report::new(HandlerError::new(&error.to_string())))
            }
        }
    }

    pub async fn get(&self, id: String) -> Result<Option<BuildInfo>, Report<HandlerError>> {
        info!("CodeBuildHandler::get - id: {}", id);

        debug!("CodeBuildHandler::get - build ids parameter");
        let mut ids: Vec<String> = Vec::new();
        ids.push(String::from(format!("{}:{}", self.project_name, id)));

        debug!("CodeBuildHandler::get - tx preparation");
        let tx = self.client.batch_get_builds().set_ids(Some(ids));

        match tx.send().await {
            Ok(result) => {
                debug!("CodeBuildHandler::get - tx result: {:?}", result);
                Ok(get_build_info(&BuildObject::Builds(result.builds)))
            }
            Err(error) => {
                error!("CodeBuildHandler::get - failed to get build: {:?}", error);
                Err(Report::new(HandlerError::new(&error.to_string())))
            }
        }
    }
}
