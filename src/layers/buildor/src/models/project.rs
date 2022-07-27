use aws_sdk_dynamodb::model::AttributeValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::request::RequestError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Commands {
    pub pre_build: Vec<String>,
    pub build: Vec<String>,
}
impl Commands {
    pub fn defaults() -> Self {
        Self {
            pre_build: vec!["npm install".to_string()],
            build: vec!["npm run build".to_string()],
        }
    }

    pub fn as_hashmap(&self) -> HashMap<String, AttributeValue> {
        let mut map: HashMap<String, AttributeValue> = HashMap::new();
        map.insert(
            "pre_build".to_string(),
            AttributeValue::L(
                self.pre_build
                    .iter()
                    .map(|command| AttributeValue::S(command.to_owned()))
                    .collect(),
            ),
        );
        map.insert(
            "build".to_string(),
            AttributeValue::L(
                self.build
                    .iter()
                    .map(|command| AttributeValue::S(command.to_owned()))
                    .collect(),
            ),
        );

        map
    }

    pub fn as_attr(&self) -> AttributeValue {
        AttributeValue::M(self.as_hashmap())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub uuid: String,
    pub name: String,
    pub repository: String,
    pub commands: Commands,
    pub output_folder: String,
    pub last_published: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectCreatePayload {
    pub name: String,
    pub repository: String,
    pub commands: Option<Commands>,
    pub output_folder: Option<String>,
}

impl Project {
    pub fn new(payload: ProjectCreatePayload) -> Self {
        Project {
            uuid: Uuid::new_v4().to_string(),
            name: payload.name,
            repository: payload.repository,
            commands: match payload.commands {
                Some(value) => value,
                None => Commands::defaults(),
            },
            output_folder: match payload.output_folder {
                Some(value) => value,
                None => "dist".to_string(),
            },
            last_published: "-".to_string(),
        }
    }

    pub fn as_hashmap(&self) -> HashMap<String, AttributeValue> {
        let mut map: HashMap<String, AttributeValue> = HashMap::new();
        map.insert("uuid".to_string(), AttributeValue::S(self.uuid.to_owned()));
        map.insert("name".to_string(), AttributeValue::S(self.name.to_owned()));
        map.insert(
            "repository".to_string(),
            AttributeValue::S(self.repository.to_owned()),
        );
        map.insert("commands".to_string(), self.commands.as_attr());
        map.insert(
            "output_folder".to_string(),
            AttributeValue::S(self.output_folder.to_owned()),
        );
        map.insert(
            "last_published".to_string(),
            AttributeValue::S(self.last_published.to_owned()),
        );

        map
    }

    pub fn as_attr(&self) -> AttributeValue {
        AttributeValue::M(self.as_hashmap())
    }
}

pub struct ProjectError;
impl ProjectError {
    pub fn creation_failed() -> RequestError {
        RequestError {
            code: "PJE00".to_string(),
            message: "Create Project Error".to_string(),
            details: "Project creation failed, try again".to_string(),
        }
    }
}