use aws_sdk_codebuild::{
    model::{Build, EnvironmentVariable, EnvironmentVariableType},
    Client,
};
use aws_sdk_dynamodb::model::AttributeValue;
use chrono::Utc;
use error_stack::Report;
use log::{self, debug, error, info};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::str::FromStr;

use crate::{
    handlers::projects::ProjectParser,
    models::{
        codebuild::{BuildInfo, BuildObject, BuildPhase, BuildPhaseStatus, ProjectDeploymentPhase},
        common::MissingModelPropertyError,
        handlers::HandlerError,
        project::Project,
    },
};

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
    let deployment_phase = match build.project_name() {
        Some(value) => Some(value.to_string()),
        None => None,
    };
    // TODO: change current_phase by build_phase
    let current_phase = match build.current_phase() {
        Some(value) => Some(BuildPhase::from_str(value).unwrap().to_string()),
        None => None,
    };
    // TODO: change build_status by build_phase_status
    let build_status = match build.build_status() {
        Some(value) => Some(
            BuildPhaseStatus::from_str(value.as_str())
                .unwrap()
                .to_string(),
        ),
        None => Some(BuildPhaseStatus::Unknown.to_string()),
    };

    Some(BuildInfo {
        uuid,
        build_number,
        start_time,
        end_time,
        deployment_phase,
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

        let deployment_phase = match item.get("deployment_phase") {
            Some(value) => match ProjectDeploymentPhase::from_str(value.as_s().unwrap()) {
                Ok(parsed) => Some(parsed.to_string()),
                Err(_) => Some(ProjectDeploymentPhase::Unknown.to_string()),
            },
            None => {
                return Err(Report::new(MissingModelPropertyError::new(
                    "deployment_phase",
                )))
            }
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
            deployment_phase,
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

    pub fn deployment_phase(
        value: Option<String>,
        codebuild_building: String,
        codebuild_deployment: String,
    ) -> ProjectDeploymentPhase {
        info!("BuildInfoParser::deployment_phase - value: {:?}", value);
        info!(
            "BuildInfoParser::deployment_phase - codebuild_building: {}",
            codebuild_building
        );
        info!(
            "BuildInfoParser::deployment_phase - codebuild_deployment: {}",
            codebuild_deployment
        );
        match value {
            Some(validated) => match validated {
                building
                    if building == codebuild_building
                        || building == ProjectDeploymentPhase::Building.to_string() =>
                {
                    ProjectDeploymentPhase::Building
                }
                deployment
                    if deployment == codebuild_deployment
                        || deployment == ProjectDeploymentPhase::Deployment.to_string() =>
                {
                    ProjectDeploymentPhase::Deployment
                }
                _ => {
                    info!("BuildInfoParser::deployment_phase - value did not match any valid option, defaults to {}", ProjectDeploymentPhase::Unknown.to_string());
                    ProjectDeploymentPhase::Unknown
                }
            },
            None => {
                info!(
                    "BuildInfoParser::deployment_phase - value not set, defaults to {}",
                    ProjectDeploymentPhase::Unknown.to_string()
                );
                ProjectDeploymentPhase::Unknown
            }
        }
    }
}

pub struct CodeBuildHandler {
    client: Client,
    codebuild_project_name_building: String,
    codebuild_project_name_deployment: String,
}

impl CodeBuildHandler {
    pub fn new(
        client: Client,
        codebuild_project_name_building: String,
        codebuild_project_name_deployment: String,
    ) -> Self {
        Self {
            client,
            codebuild_project_name_building,
            codebuild_project_name_deployment,
        }
    }

    pub async fn create(&self, project: &Project) -> Result<BuildInfo, Report<HandlerError>> {
        info!("CodeBuildHandler::create - project: {:?}", project);
        let timestamp = Utc::now().to_rfc3339().to_string();

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
            .project_name(self.codebuild_project_name_building.to_string())
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
                    Some(mut build_info) => {
                        build_info.deployment_phase = Some(
                            BuildInfoParser::deployment_phase(
                                build_info.deployment_phase,
                                self.codebuild_project_name_building.clone(),
                                self.codebuild_project_name_deployment.clone(),
                            )
                            .to_string(),
                        );
                        info!(
                            "CodeBuildHandler::create - parsed deployment phase: {:?}",
                            build_info
                        );
                        Ok(build_info)
                    }
                    None => {
                        error!("CodeBuildHandler::create - code info parsing failed, but build was created");
                        Err(Report::new(HandlerError::new(
                            "Failed to parse build result into BuildInfo",
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
        ids.push(String::from(format!(
            "{}:{}",
            self.codebuild_project_name_building, id
        )));

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
