use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
