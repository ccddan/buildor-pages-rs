use std::collections::HashMap;

use aws_sdk_dynamodb::model::AttributeValue;
use chrono::Utc;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use super::common::AsDynamoDBAttributeValue;
use super::request::RequestError;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub uuid: String,
    #[serde(rename(serialize = "firstName"))]
    pub fname: String,
    #[serde(rename(serialize = "lastName"))]
    pub lname: String,
    #[serde(rename(serialize = "updatedAt"))]
    pub updated_at: String,
    #[serde(rename(serialize = "createdAt"))]
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCreatePayload {
    #[serde(rename(serialize = "firstName"))]
    pub fname: String,
    #[serde(rename(serialize = "lastName"))]
    pub lname: String,
}

impl User {
    pub fn new(payload: UserCreatePayload) -> Self {
        let timestamp = Utc::now().to_rfc3339().to_string();
        User {
            uuid: Uuid::new_v4().to_string(),
            fname: payload.fname,
            lname: payload.lname,
            updated_at: timestamp.clone(),
            created_at: timestamp,
        }
    }
}
impl AsDynamoDBAttributeValue for User {
    fn as_hashmap(&self) -> HashMap<String, AttributeValue> {
        let mut map: HashMap<String, AttributeValue> = HashMap::new();
        map.insert("uuid".to_string(), AttributeValue::S(self.uuid.to_owned()));
        map.insert(
            "fname".to_string(),
            AttributeValue::S(self.fname.to_owned()),
        );
        map.insert(
            "lname".to_string(),
            AttributeValue::S(self.lname.to_owned()),
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
