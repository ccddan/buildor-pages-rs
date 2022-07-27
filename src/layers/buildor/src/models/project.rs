use serde::{Deserialize, Serialize};
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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub uuid: String,
    pub name: String,
    pub repository: String,
    // pub commands: Commands,
    pub last_published: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectCreatePayload {
    pub name: String,
    pub repository: String,
    pub commands: Option<Commands>,
}

impl Project {
    pub fn new(payload: ProjectCreatePayload) -> Self {
        Project {
            uuid: Uuid::new_v4().to_string(),
            name: payload.name,
            repository: payload.repository,
            // commands: match payload.commands {
            //     Some(value) => value,
            //     None => Commands::defaults(),
            // },
            last_published: "-".to_string(),
        }
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
