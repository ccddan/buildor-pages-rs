use std::collections::HashMap;

use crate::models::user::User;
use aws_sdk_dynamodb::types::SdkError;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::{error::ScanError, model::AttributeValue};
use serde_json::{json, Value};
use tokio_stream::StreamExt;

pub struct UsersParser {}
impl UsersParser {
    pub fn user(item: HashMap<String, AttributeValue>) -> User {
        let uuid = item.get("uuid").unwrap().as_s().unwrap().to_string();
        let fname = item.get("lname").unwrap().as_s().unwrap().to_string();
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

    pub async fn list(self) -> Vec<User> {
        let mut data = Vec::new();

        println!("Preparing to insert new record in db");
        let tx = self
            .table
            .scan()
            .table_name(self.table_name)
            .into_paginator()
            .items();
        println!("Send transaction");
        let result: Result<Vec<_>, SdkError<ScanError>> = tx.send().collect().await;
        println!("Tx response: {:?}", result);

        match result {
            Ok(res) => {
                println!("Parse users records");
                for item in res {
                    let user = UsersParser::user(item);
                    println!("user: {:?}", user);
                    data.push(user);
                }
            }
            Err(err) => {
                println!("Failed to list users: {}", err);
            }
        }

        data
    }
}
