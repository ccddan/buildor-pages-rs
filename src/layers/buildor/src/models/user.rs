use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::request::RequestError;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub uuid: String,
    pub fname: String,
    pub lname: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCreatePayload {
    pub fname: String,
    pub lname: String,
}

impl User {
    pub fn new(payload: UserCreatePayload) -> Self {
        User {
            uuid: Uuid::new_v4().to_string(),
            fname: payload.fname,
            lname: payload.lname,
        }
    }
}

pub struct UserError;
impl UserError {
    pub fn creation_failed() -> RequestError {
        RequestError {
            code: "USE00".to_string(),
            message: "Create User Error".to_string(),
            details: "User creation failed, try again".to_string(),
        }
    }
}
