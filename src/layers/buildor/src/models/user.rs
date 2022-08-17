use std::collections::HashMap;

use aws_sdk_dynamodb::model::AttributeValue;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use super::{request::RequestError, AsDynamoDBAttributeValue};

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
