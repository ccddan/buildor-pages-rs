use aws_sdk_dynamodb::types::SdkError;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::{error::ScanError, model::AttributeValue};
use error_stack::{Context, Report};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fmt;
use tokio_stream::StreamExt;

use crate::handlers::commands::CommandsParser;
use crate::models::common::AsDynamoDBAttributeValue;
use crate::models::project::{Project, ProjectCreatePayload};

#[derive(Debug)]
pub struct MissingProjectPropertyError {
    pub name: String,
}
impl MissingProjectPropertyError {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}
impl fmt::Display for MissingProjectPropertyError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(format!("Missing project property: {}", self.name).as_str())
    }
}
impl Context for MissingProjectPropertyError {}

pub struct ProjectParser {}
impl ProjectParser {
    pub fn parse(
        item: HashMap<String, AttributeValue>,
    ) -> Result<Project, Report<MissingProjectPropertyError>> {
        let uuid = match item.get("uuid") {
            Some(value) => value.as_s().unwrap().to_string(),

            None => return Err(Report::new(MissingProjectPropertyError::new("uuid"))),
        };

        let name = match item.get("name") {
            Some(value) => value.as_s().unwrap().to_string(),
            None => return Err(Report::new(MissingProjectPropertyError::new("name"))),
        };

        let repository = match item.get("repository") {
            Some(value) => value.as_s().unwrap().to_string(),
            None => return Err(Report::new(MissingProjectPropertyError::new("repository"))),
        };

        let commands = match item.get("commands") {
            Some(value) => match CommandsParser::parse(value.as_m().unwrap().to_owned()) {
                Ok(value) => value,
                Err(error) => {
                    return Err(error.change_context(MissingProjectPropertyError::new("commands")))
                }
            },
            None => return Err(Report::new(MissingProjectPropertyError::new("commands"))),
        };

        let output_folder = match item.get("output_folder") {
            Some(value) => value.as_s().unwrap().to_string(),
            None => {
                return Err(Report::new(MissingProjectPropertyError::new(
                    "output_folder",
                )))
            }
        };

        let last_published = match item.get("last_published") {
            Some(value) => value.as_s().unwrap().to_string(),
            None => {
                return Err(Report::new(MissingProjectPropertyError::new(
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
    ) -> Result<Value, Report<MissingProjectPropertyError>> {
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
            Err(error) => assert_eq!(error.to_string(), "Missing project property: uuid"),
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
            Err(error) => assert_eq!(error.to_string(), "Missing project property: name"),
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
            Err(error) => assert_eq!(error.to_string(), "Missing project property: repository"),
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
            Err(error) => assert_eq!(error.to_string(), "Missing project property: commands"),
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
            Err(error) => assert_eq!(error.to_string(), "Missing project property: output_folder"),
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
            Err(error) => assert_eq!(
                error.to_string(),
                "Missing project property: last_published"
            ),
            _ => assert_eq!("", "Should have panicked but it did not"),
        }
    }
}

pub struct ProjectsHandler {
    table: Client,
    table_name: String,
}

impl ProjectsHandler {
    pub fn new(table: Client, table_name: String) -> Self {
        ProjectsHandler { table, table_name }
    }

    pub async fn create(self, payload: ProjectCreatePayload) -> Option<Project> {
        println!("ProjectsHandler::create - payload: {:?}", payload);
        let project = Project::new(payload);

        let tx = self
            .table
            .put_item()
            .table_name(self.table_name)
            .set_item(Some(project.as_hashmap()));

        println!("ProjectsHandler::create - send tx");
        let result = tx.send().await;
        println!("ProjectsHandler::create - tx response: {:?}", result);

        match result {
            Ok(res) => {
                println!("ProjectsHandler::create - new user created: {:?}", res);
                Some(project)
            }
            Err(err) => {
                println!("ProjectsHandler::create - failed to create user: {:?}", err);
                None
            }
        }
    }

    pub async fn list(self) -> Vec<Project> {
        let mut data = Vec::new();

        println!("ProjectsHandler::list - preparing query to list projects");
        let tx = self
            .table
            .scan()
            .table_name(self.table_name)
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
                            println!("ProjectParser::list - parse error: {}", error);
                        }
                    };
                }
            }
            Err(err) => {
                println!("ProjectsHandler::list - failed to list projects: {}", err);
            }
        }

        data
    }
}
