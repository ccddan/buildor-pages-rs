use aws_sdk_dynamodb::model::AttributeValue;
use chrono::Utc;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

use super::common::AsDynamoDBAttributeValue;
use super::request::RequestError;
use super::{codebuild::BuildInfo, project::Project};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectDeployment {
    pub uuid: String,
    pub project: Project,
    pub build: BuildInfo,
    pub updated_at: String,
    pub created_at: String,
}

impl ProjectDeployment {
    pub fn new(project: Project, build: BuildInfo) -> Self {
        let timestamp = Utc::now().to_string();
        Self {
            uuid: build.uuid.clone(),
            project,
            build,
            updated_at: String::from(&timestamp),
            created_at: timestamp,
        }
    }
}

impl AsDynamoDBAttributeValue for ProjectDeployment {
    fn as_hashmap(&self) -> HashMap<String, AttributeValue> {
        let mut map: HashMap<String, AttributeValue> = HashMap::new();
        map.insert("uuid".to_string(), AttributeValue::S(self.uuid.to_owned()));
        map.insert("project".to_string(), self.project.as_attr());
        map.insert("build".to_string(), self.build.as_attr());
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectDeploymentCreatePayload {
    pub project: Project,
    pub build: BuildInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectDeploymentCreatePayloadRequest {
    pub project_uuid: String,
}

pub struct ProjectDeploymentError;
impl ProjectDeploymentError {
    pub fn creation_failed() -> RequestError {
        RequestError {
            code: "PDE00".to_string(),
            message: "Create Project Deployment Error".to_string(),
            details: "Record creation failed but build has been triggered (probably)".to_string(),
        }
    }
}
