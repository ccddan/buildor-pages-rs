use aws_sdk_dynamodb::model::AttributeValue;
use chrono::Utc;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::commands::Commands;
use super::common::AsDynamoDBAttributeValue;
use super::request::RequestError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub uuid: String,
    pub name: String,
    pub repository: String,
    pub commands: Commands,
    #[serde(rename(serialize = "outputFolder"))]
    pub output_folder: String,
    #[serde(rename(serialize = "lastPublished"))]
    pub last_published: String,
    #[serde(rename(serialize = "updatedAt"))]
    pub updated_at: String,
    #[serde(rename(serialize = "createdAt"))]
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectCreatePayload {
    pub name: String,
    pub repository: String,
    pub commands: Option<Commands>,
    #[serde(rename(serialize = "outputFolder"))]
    pub output_folder: Option<String>,
}
impl ProjectCreatePayload {
    pub fn default(name: String, repository: String) -> Self {
        Self {
            name,
            repository,
            commands: None,
            output_folder: None,
        }
    }
}

impl Project {
    pub fn new(payload: ProjectCreatePayload) -> Self {
        let timestamp = Utc::now().to_rfc3339().to_string();
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
            updated_at: timestamp.clone(),
            created_at: timestamp,
        }
    }
}

impl AsDynamoDBAttributeValue for Project {
    fn as_hashmap(&self) -> HashMap<String, AttributeValue> {
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
        map.insert(
            "updated_at".to_string(),
            AttributeValue::S(self.updated_at.to_owned()),
        );
        map.insert(
            "created_at".to_string(),
            AttributeValue::S(self.created_at.to_owned()),
        );

        map
    }

    fn as_attr(&self) -> AttributeValue {
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
