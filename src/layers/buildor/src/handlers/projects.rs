use async_trait::async_trait;
use aws_sdk_dynamodb::types::SdkError;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::{error::ScanError, model::AttributeValue};
use error_stack::Report;
use serde_json::{json, Value};
use std::collections::HashMap;
use tokio_stream::StreamExt;

use crate::handlers::commands::CommandsParser;
use crate::models::common::{AsDynamoDBAttributeValue, MissingModelPropertyError};
use crate::models::handlers::{HandlerCreate, HandlerError, HandlerList};
use crate::models::project::{Project, ProjectCreatePayload};

pub struct ProjectParser {}
impl ProjectParser {
    pub fn parse(
        item: HashMap<String, AttributeValue>,
    ) -> Result<Project, Report<MissingModelPropertyError>> {
        let uuid = match item.get("uuid") {
            Some(value) => value.as_s().unwrap().to_string(),
            None => return Err(Report::new(MissingModelPropertyError::new("uuid"))),
        };

        let name = match item.get("name") {
            Some(value) => value.as_s().unwrap().to_string(),
            None => return Err(Report::new(MissingModelPropertyError::new("name"))),
        };

        let repository = match item.get("repository") {
            Some(value) => value.as_s().unwrap().to_string(),
            None => return Err(Report::new(MissingModelPropertyError::new("repository"))),
        };

        let commands = match item.get("commands") {
            Some(value) => match CommandsParser::parse(value.as_m().unwrap().to_owned()) {
                Ok(value) => value,
                Err(error) => {
                    return Err(error.change_context(MissingModelPropertyError::new("commands")))
                }
            },
            None => return Err(Report::new(MissingModelPropertyError::new("commands"))),
        };

        let output_folder = match item.get("output_folder") {
            Some(value) => value.as_s().unwrap().to_string(),
            None => return Err(Report::new(MissingModelPropertyError::new("output_folder"))),
        };

        let last_published = match item.get("last_published") {
            Some(value) => value.as_s().unwrap().to_string(),
            None => {
                return Err(Report::new(MissingModelPropertyError::new(
                    "last_published",
                )))
            }
        };

        Ok(Project {
            uuid,
            name,
            repository,
            commands,
            output_folder,
            last_published,
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

#[cfg(test)]
mod project_parser_tests {
    use super::*;
    use crate::models::commands::Commands;

    // Validate required properties
    #[test]
    fn fails_on_missing_uuid() {
        let input: HashMap<String, AttributeValue> = HashMap::new();
        match ProjectParser::parse(input) {
            Err(error) => assert_eq!(error.to_string(), "Missing model property: uuid"),
            _ => assert_eq!("", "Should have panicked but it did not"),
        }
    }

    #[test]
    fn fails_on_missing_name() {
        let mut input: HashMap<String, AttributeValue> = HashMap::new();
        input.insert(
            "uuid".to_string(),
            AttributeValue::S("uuid-value".to_string()),
        );
        match ProjectParser::parse(input) {
            Err(error) => assert_eq!(error.to_string(), "Missing model property: name"),
            _ => assert_eq!("", "Should have panicked but it did not"),
        }
    }

    #[test]
    fn fails_on_missing_repository() {
        let mut input: HashMap<String, AttributeValue> = HashMap::new();
        input.insert(
            "uuid".to_string(),
            AttributeValue::S("uuid-value".to_string()),
        );
        input.insert(
            "name".to_string(),
            AttributeValue::S("name-value".to_string()),
        );

        match ProjectParser::parse(input) {
            Err(error) => assert_eq!(error.to_string(), "Missing model property: repository"),
            _ => assert_eq!("", "Should have panicked but it did not"),
        }
    }

    #[test]
    fn fails_on_missing_commands() {
        let mut input: HashMap<String, AttributeValue> = HashMap::new();
        input.insert(
            "uuid".to_string(),
            AttributeValue::S("uuid-value".to_string()),
        );
        input.insert(
            "name".to_string(),
            AttributeValue::S("name-value".to_string()),
        );
        input.insert(
            "repository".to_string(),
            AttributeValue::S("repository-value".to_string()),
        );

        match ProjectParser::parse(input) {
            Err(error) => assert_eq!(error.to_string(), "Missing model property: commands"),
            _ => assert_eq!("", "Should have panicked but it did not"),
        }
    }

    #[test]
    fn fails_on_missing_output_folder() {
        let mut input: HashMap<String, AttributeValue> = HashMap::new();
        input.insert(
            "uuid".to_string(),
            AttributeValue::S("uuid-value".to_string()),
        );
        input.insert(
            "name".to_string(),
            AttributeValue::S("name-value".to_string()),
        );
        input.insert(
            "repository".to_string(),
            AttributeValue::S("repository-value".to_string()),
        );
        input.insert(
            "commands".to_string(),
            AttributeValue::M(Commands::defaults().as_hashmap()),
        );

        match ProjectParser::parse(input) {
            Err(error) => assert_eq!(error.to_string(), "Missing model property: output_folder"),
            _ => assert_eq!("", "Should have panicked but it did not"),
        }
    }

    #[test]
    fn fails_on_missing_last_published() {
        let mut input: HashMap<String, AttributeValue> = HashMap::new();
        input.insert(
            "uuid".to_string(),
            AttributeValue::S("uuid-value".to_string()),
        );
        input.insert(
            "name".to_string(),
            AttributeValue::S("name-value".to_string()),
        );
        input.insert(
            "repository".to_string(),
            AttributeValue::S("repository-value".to_string()),
        );
        input.insert(
            "commands".to_string(),
            AttributeValue::M(Commands::defaults().as_hashmap()),
        );
        input.insert(
            "output_folder".to_string(),
            AttributeValue::S("output-folder-value".to_string()),
        );

        match ProjectParser::parse(input) {
            Err(error) => assert_eq!(error.to_string(), "Missing model property: last_published"),
            _ => assert_eq!("", "Should have panicked but it did not"),
        }
    }
}

pub struct ProjectsHandler {
    table: Client,
    table_name: String,
}
impl ProjectsHandler {
    pub fn new(client: Client, table_name: String) -> Self {
        Self {
            table: client,
            table_name,
        }
    }
}

#[async_trait]
impl HandlerCreate<Project, ProjectCreatePayload, HandlerError> for ProjectsHandler {
    async fn create(&self, payload: ProjectCreatePayload) -> Result<Project, Report<HandlerError>> {
        println!("ProjectsHandler::create - payload: {:?}", payload);
        let project = Project::new(payload);

        let tx = self
            .table
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(project.as_hashmap()));

        println!("ProjectsHandler::create - send tx");
        let result = tx.send().await;
        println!("ProjectsHandler::create - tx response: {:?}", result);

        match result {
            Ok(res) => {
                println!("ProjectsHandler::create - new user created: {:?}", res);
                Ok(project)
            }
            Err(err) => {
                println!("ProjectsHandler::create - failed to create user: {:?}", err);
                Err(Report::new(HandlerError::new(&err.to_string())))
            }
        }
    }
}
#[async_trait]
impl HandlerList<Project, HandlerError> for ProjectsHandler {
    async fn list(&self) -> Result<Vec<Project>, Report<HandlerError>> {
        let mut data: Vec<Project> = Vec::new();

        println!("ProjectsHandler::list - preparing query to list projects");
        let tx = self
            .table
            .scan()
            .table_name(&self.table_name)
            .into_paginator()
            .items();
        println!("ProjectsHandler::list - send tx");
        let result: Result<Vec<_>, SdkError<ScanError>> = tx.send().collect().await;
        println!("ProjectsHandler::list - tx response: {:?}", result);

        match result {
            Ok(res) => {
                println!("ProjectsHandler::list - parse projects");
                for item in res {
                    println!("ProjectParser::list - parse record: {:?}", &item);
                    match ProjectParser::parse(item) {
                        Ok(parsed) => {
                            println!("ProjectsHandler::list - project: {:?}", parsed);
                            data.push(parsed);
                        }
                        Err(error) => {
                            println!(
                                "ProjectParser::list - parse error (skip from result): {}",
                                error
                            )
                        }
                    };
                }
            }
            Err(err) => {
                println!("ProjectsHandler::list - failed to list projects: {}", err);
                return Err(Report::new(HandlerError::new(&err.to_string())));
            }
        };

        Ok(data)
    }
}
