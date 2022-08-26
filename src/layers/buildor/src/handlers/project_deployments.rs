use async_trait::async_trait;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use error_stack::Report;
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::handlers::codebuild::BuildInfoParser;
use crate::handlers::projects::ProjectParser;
use crate::models::common::{AsDynamoDBAttributeValue, MissingModelPropertyError};
use crate::models::handlers::{HandlerCreate, HandlerError};
use crate::models::project_deployment::{ProjectDeployment, ProjectDeploymentCreatePayload};

pub struct ProjectDeploymentParser {}
impl ProjectDeploymentParser {
    pub fn parse(
        item: HashMap<String, AttributeValue>,
    ) -> Result<ProjectDeployment, Report<MissingModelPropertyError>> {
        let uuid = match item.get("uuid") {
            Some(value) => value.as_s().unwrap().to_string(),
            None => return Err(Report::new(MissingModelPropertyError::new("uuid"))),
        };

        let project = match item.get("project") {
            Some(value) => match ProjectParser::parse(value.as_m().unwrap().to_owned()) {
                Ok(value) => value,
                Err(error) => {
                    return Err(error.change_context(MissingModelPropertyError::new("project")))
                }
            },
            None => return Err(Report::new(MissingModelPropertyError::new("project"))),
        };

        let build = match item.get("build") {
            Some(value) => match BuildInfoParser::parse(value.as_m().unwrap().to_owned()) {
                Ok(value) => value,
                Err(_) => {
                    return Err(Report::new(MissingModelPropertyError::new("build")));
                }
            },
            None => return Err(Report::new(MissingModelPropertyError::new("build"))),
        };

        let updated_at = match item.get("updated_at") {
            Some(value) => value.as_s().unwrap().to_string(),
            None => return Err(Report::new(MissingModelPropertyError::new("updated_at"))),
        };

        let created_at = match item.get("created_at") {
            Some(value) => value.as_s().unwrap().to_string(),
            None => return Err(Report::new(MissingModelPropertyError::new("created_at"))),
        };

        Ok(ProjectDeployment {
            uuid,
            project,
            build,
            updated_at,
            created_at,
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

pub struct ProjectDeploymentsHandler {
    table: Client,
    table_name: String,
}
impl ProjectDeploymentsHandler {
    pub fn new(client: Client, table_name: String) -> Self {
        Self {
            table: client,
            table_name,
        }
    }
}

#[async_trait]
impl HandlerCreate<ProjectDeployment, ProjectDeploymentCreatePayload, HandlerError>
    for ProjectDeploymentsHandler
{
    async fn create(
        &self,
        payload: ProjectDeploymentCreatePayload,
    ) -> Result<ProjectDeployment, Report<HandlerError>> {
        println!("ProjectDeploymentsHandler::create - payload: {:?}", payload);
        let project_deployment = ProjectDeployment::new(payload.project, payload.build);

        let tx = self
            .table
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(project_deployment.as_hashmap()));

        println!("ProjectDeploymentsHandler::create - send tx");
        let result = tx.send().await;
        println!(
            "ProjectDeploymentsHandler::create - tx response: {:?}",
            result
        );

        match result {
            Ok(res) => {
                println!(
                    "ProjectDeploymentsHandler::create - new user created: {:?}",
                    res
                );
                Ok(project_deployment)
            }
            Err(err) => {
                println!(
                    "ProjectDeploymentsHandler::create - failed to create user: {:?}",
                    err
                );
                Err(Report::new(HandlerError::new(&err.to_string())))
            }
        }
    }
}
