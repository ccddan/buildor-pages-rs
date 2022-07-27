use std::collections::HashMap;

use crate::models::project::{/* Commands, */ Project, ProjectCreatePayload};
use aws_sdk_dynamodb::types::SdkError;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::{error::ScanError, model::AttributeValue};
// use serde_dynamo::{from_item, to_item};
use serde_json::{json, Value};
use tokio_stream::StreamExt;

pub struct ProjectParser {}
impl ProjectParser {
    pub fn project(item: HashMap<String, AttributeValue>) -> Project {
        // let project = from_item(item);
        let uuid = item.get("uuid").unwrap().as_s().unwrap().to_string();
        let name = item.get("name").unwrap().as_s().unwrap().to_string();
        let repository = item.get("repository").unwrap().as_s().unwrap().to_string();
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
            // commands: Commands::defaults(),
            last_published,
        }
    }

    pub fn json(item: HashMap<String, AttributeValue>) -> Value {
        json!(ProjectParser::project(item))
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
        // let item = to_item(project).unwrap();

        let tx = self
            .table
            .put_item()
            .table_name(self.table_name)
            // .set_item(Some(item));
            .item("uuid", AttributeValue::S(project.uuid.to_string()))
            .item("name", AttributeValue::S(project.name.to_string()))
            .item(
                "repository",
                AttributeValue::S(project.repository.to_string()),
            )
            // .item(
            //     "commands",
            //     AttributeValue::M(project.commands.into::<HashMap>()),
            // )
            .item(
                "last_published",
                AttributeValue::S(project.last_published.to_string()),
            );

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
                    let user = ProjectParser::project(item);
                    println!("ProjectsHandler::list - user: {:?}", user);
                    data.push(user);
                }
            }
            Err(err) => {
                println!("ProjectsHandler::list - failed to list projects: {}", err);
            }
        }

        data
    }
}
