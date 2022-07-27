use std::collections::HashMap;

use crate::models::project::{Commands, Project, ProjectCreatePayload};
use crate::models::AsDynamoDBAttributeValue;
use aws_sdk_dynamodb::types::SdkError;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::{error::ScanError, model::AttributeValue};
use serde_json::{json, Value};
use tokio_stream::StreamExt;

pub struct CommandsParser;
impl CommandsParser {
    pub fn parse(item: HashMap<String, AttributeValue>) -> Commands {
        let pre_build = item
            .get("pre_build")
            .unwrap()
            .as_l()
            .unwrap()
            .iter()
            .map(|command| command.as_s().unwrap().to_string())
            .collect();
        let build = item
            .get("pre_build")
            .unwrap()
            .as_l()
            .unwrap()
            .iter()
            .map(|command| command.as_s().unwrap().to_string())
            .collect();
        Commands { pre_build, build }
    }
}

pub struct ProjectParser {}
impl ProjectParser {
    pub fn parse(item: HashMap<String, AttributeValue>) -> Project {
        let uuid = item.get("uuid").unwrap().as_s().unwrap().to_string();
        let name = item.get("name").unwrap().as_s().unwrap().to_string();
        let repository = item.get("repository").unwrap().as_s().unwrap().to_string();
        let commands =
            CommandsParser::parse(item.get("commands").unwrap().as_m().unwrap().to_owned());
        let output_folder = item
            .get("output_folder")
            .unwrap()
            .as_s()
            .unwrap()
            .to_string();
        let last_published = item
            .get("last_published")
            .unwrap()
            .as_s()
            .unwrap()
            .to_string();

        Project {
            uuid,
            name,
            repository,
            commands,
            output_folder,
            last_published,
        }
    }

    pub fn json(item: HashMap<String, AttributeValue>) -> Value {
        json!(ProjectParser::parse(item))
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
                    let project = ProjectParser::parse(item);
                    println!("ProjectsHandler::list - project: {:?}", project);
                    data.push(project);
                }
            }
            Err(err) => {
                println!("ProjectsHandler::list - failed to list projects: {}", err);
            }
        }

        data
    }
}
