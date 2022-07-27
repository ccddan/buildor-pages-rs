use std::collections::HashMap;

use crate::models::user::{User, UserCreatePayload};
use aws_sdk_dynamodb::types::SdkError;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::{error::ScanError, model::AttributeValue};
use serde_json::{json, Value};
use tokio_stream::StreamExt;

pub struct UsersParser {}
impl UsersParser {
    pub fn user(item: HashMap<String, AttributeValue>) -> User {
        let uuid = item.get("uuid").unwrap().as_s().unwrap().to_string();
        let fname = item.get("fname").unwrap().as_s().unwrap().to_string();
        let lname = item.get("lname").unwrap().as_s().unwrap().to_string();

        User { uuid, fname, lname }
    }

    pub fn json(item: HashMap<String, AttributeValue>) -> Value {
        json!(UsersParser::user(item))
    }
}

pub struct UsersHandler {
    table: Client,
    table_name: String,
}

impl UsersHandler {
    pub fn new(table: Client, table_name: String) -> Self {
        UsersHandler { table, table_name }
    }

    pub async fn create(self, payload: UserCreatePayload) -> Option<User> {
        println!("UserHandler::create - payload: {:?}", payload);
        let user = User::new(payload);

        let tx = self
            .table
            .put_item()
            .table_name(self.table_name)
            .item("uuid", AttributeValue::S(user.uuid.to_string()))
            .item("fname", AttributeValue::S(user.fname.to_string()))
            .item("lname", AttributeValue::S(user.lname.to_string()));

        println!("UserHandler::create - send tx");
        let result = tx.send().await;
        println!("UserHandler::create - tx response: {:?}", result);

        match result {
            Ok(res) => {
                println!("UserHandler::create - new user created: {:?}", res);
                Some(user)
            }
            Err(err) => {
                println!("UserHandler::create - failed to create user: {:?}", err);
                None
            }
        }
    }

    pub async fn list(self) -> Vec<User> {
        let mut data = Vec::new();

        println!("UserHandler::list - preparing query to list users");
        let tx = self
            .table
            .scan()
            .table_name(self.table_name)
            .into_paginator()
            .items();
        println!("UserHandler::list - send tx");
        let result: Result<Vec<_>, SdkError<ScanError>> = tx.send().collect().await;
        println!("UserHandler::list - tx response: {:?}", result);

        match result {
            Ok(res) => {
                println!("UserHandler::list - parse users");
                for item in res {
                    let user = UsersParser::user(item);
                    println!("UserHandler::list - user: {:?}", user);
                    data.push(user);
                }
            }
            Err(err) => {
                println!("UserHandler::list - failed to list users: {}", err);
            }
        }

        data
    }
}
