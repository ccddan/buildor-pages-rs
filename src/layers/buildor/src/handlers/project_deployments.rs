use async_trait::async_trait;
use aws_sdk_dynamodb::{
    model::{AttributeValue, ReturnConsumedCapacity, ReturnItemCollectionMetrics, ReturnValue},
    Client,
};
use error_stack::Report;
use log::{self, error, info};
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::{
    handlers::{codebuild::BuildInfoParser, projects::ProjectParser},
    models::{
        common::{AsDynamoDBAttributeValue, MissingModelPropertyError},
        handlers::{HandlerCreate, HandlerError, HandlerGet, HandlerUpdate},
        project_deployment::{
            ProjectDeployment, ProjectDeploymentCreatePayload, ProjectDeploymentUpdatePayload,
        },
    },
};

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
        info!("ProjectDeploymentsHandler::create - payload: {:?}", payload);
        let project_deployment = ProjectDeployment::new(payload.project, payload.build);

        let tx = self
            .table
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(project_deployment.as_hashmap()));

        info!("ProjectDeploymentsHandler::create - send tx");
        let result = tx.send().await;
        info!(
            "ProjectDeploymentsHandler::create - tx response: {:?}",
            result
        );

        match result {
            Ok(res) => {
                info!(
                    "ProjectDeploymentsHandler::create - new project deployment created: {:?}",
                    res
                );
                Ok(project_deployment)
            }
            Err(err) => {
                error!(
                    "ProjectDeploymentsHandler::create - failed to create project deployment: {:?}",
                    err
                );
                Err(Report::new(HandlerError::new(&err.to_string())))
            }
        }
    }
}

#[async_trait]
impl HandlerGet<ProjectDeployment, HandlerError> for ProjectDeploymentsHandler {
    async fn get(&self, uuid: String) -> Result<Option<ProjectDeployment>, Report<HandlerError>> {
        info!("ProjectDeploymentHandler::get - uuid: {}", uuid);

        let tx = self
            .table
            .get_item()
            .table_name(&self.table_name)
            .key("uuid".to_string(), AttributeValue::S(uuid));

        info!("ProjectDeploymentHandler::get - send tx");
        let result = tx.send().await;
        info!("ProjectDeploymentHandler::get - tx response: {:?}", result);

        match result {
            Ok(res) => {
                info!("ProjectDeploymentHandler::get - record: {:?}", res);
                match res.item {
                    Some(value) => match ProjectDeploymentParser::parse(value) {
                        Ok(deployment) => Ok(Some(deployment)),
                        Err(error) => {
                            error!("ProjectDeploymentHandler::get - failed to parse project deployment: {}", error);
                            Ok(None)
                        }
                    },
                    None => {
                        info!("ProjectDeploymentHandler::get - no project deployment found with given uuid");
                        Ok(None)
                    }
                }
            }
            Err(error) => {
                error!(
                    "ProjectDeploymentHandler::get - failed to get project deployment: {:?}",
                    error
                );
                Err(Report::new(HandlerError::new(&error.to_string())))
            }
        }
    }
}

#[async_trait]
impl HandlerUpdate<bool, ProjectDeploymentUpdatePayload, HandlerError>
    for ProjectDeploymentsHandler
{
    async fn update(
        &self,
        uuid: String,
        payload: ProjectDeploymentUpdatePayload,
    ) -> Result<(), Report<HandlerError>> {
        info!("ProjectDeploymentsHandler::update - uuid: {}", uuid);
        info!(
            "ProjectDeploymentsHandler::update - payload: {:#?}",
            payload
        );
        let expressions = self.get_update_expressions(payload);

        let tx = self
            .table
            .update_item()
            .table_name(&self.table_name)
            .return_values(ReturnValue::UpdatedOld)
            .return_consumed_capacity(ReturnConsumedCapacity::Total)
            .return_item_collection_metrics(ReturnItemCollectionMetrics::Size)
            .key("uuid".to_string(), AttributeValue::S(uuid))
            .set_expression_attribute_names(Some(expressions.attribute_names))
            .set_expression_attribute_values(Some(expressions.attribute_values))
            .update_expression(expressions.update_expression);

        info!("ProjectDeploymentsHandler::update - send tx");
        let result = tx.send().await;
        info!(
            "ProjectDeploymentsHandler::update - tx response: {:#?}",
            result
        );

        match result {
            Ok(res) => {
                info!(
                    "ProjectDeploymentsHandler::update - project deployment updated: {:?}",
                    res
                );
                Ok(())
            }
            Err(err) => {
                error!(
                    "ProjectDeploymentsHandler::update - failed to update project deployment: {:?}",
                    err
                );
                Err(Report::new(HandlerError::new(&err.to_string())))
            }
        }
    }
}
