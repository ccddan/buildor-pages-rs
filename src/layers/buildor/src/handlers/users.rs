use std::collections::HashMap;

use crate::models::common::{AsDynamoDBAttributeValue, MissingModelPropertyError};
use crate::models::handlers::{HandlerCreate, HandlerError, HandlerList};
use crate::models::user::{User, UserCreatePayload};
use async_trait::async_trait;
use aws_sdk_dynamodb::types::SdkError;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::{error::ScanError, model::AttributeValue};
use error_stack::Report;
use serde_json::{json, Value};
use tokio_stream::StreamExt;

pub struct UsersParser {}
impl UsersParser {
    pub fn parse(
        item: HashMap<String, AttributeValue>,
    ) -> Result<User, Report<MissingModelPropertyError>> {
        let uuid = match item.get("uuid") {
            Some(value) => value.as_s().unwrap().to_string(),
            None => return Err(Report::new(MissingModelPropertyError::new("uuid"))),
        };

        let fname = match item.get("fname") {
            Some(value) => value.as_s().unwrap().to_string(),
            None => return Err(Report::new(MissingModelPropertyError::new("fname"))),
        };

        let lname = match item.get("lname") {
            Some(value) => value.as_s().unwrap().to_string(),
            None => return Err(Report::new(MissingModelPropertyError::new("lname"))),
        };

        Ok(User { uuid, fname, lname })
    }

    pub fn json(
        item: HashMap<String, AttributeValue>,
    ) -> Result<Value, Report<MissingModelPropertyError>> {
        match UsersParser::parse(item) {
            Ok(parsed) => Ok(json!(parsed)),
            Err(error) => Err(error),
        }
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
}

#[async_trait]
impl HandlerCreate<User, UserCreatePayload, HandlerError> for UsersHandler {
    async fn create(&self, payload: UserCreatePayload) -> Result<User, Report<HandlerError>> {
        println!("UserHandler::create - payload: {:?}", payload);
        let user = User::new(payload);

        let tx = self
            .table
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(user.as_hashmap()));

        println!("UserHandler::create - send tx");
        let result = tx.send().await;
        println!("UserHandler::create - tx response: {:?}", result);

        match result {
            Ok(res) => {
                println!("UserHandler::create - new user created: {:?}", res);
                Ok(user)
            }
            Err(error) => {
                println!("UserHandler::create - failed to create user: {:?}", error);
                Err(Report::new(HandlerError::new(&error.to_string())))
            }
        }
    }
}

#[async_trait]
impl HandlerList<User, HandlerError> for UsersHandler {
    async fn list(&self) -> Result<Vec<User>, Report<HandlerError>> {
        let mut data = Vec::new();

        println!("UserHandler::list - preparing query to list users");
        let tx = self
            .table
            .scan()
            .table_name(&self.table_name)
            .into_paginator()
            .items();
        println!("UserHandler::list - send tx");
        let result: Result<Vec<_>, SdkError<ScanError>> = tx.send().collect().await;
        println!("UserHandler::list - tx response: {:?}", result);

        match result {
            Ok(res) => {
                println!("UserHandler::list - parse users");
                for item in res {
                    println!("UsersParser::list - parse record: {:?}", &item);
                    match UsersParser::parse(item) {
                        Ok(parsed) => {
                            println!("UsersHandler::list - user: {:?}", parsed);
                            data.push(parsed);
                        }
                        Err(error) => {
                            println!(
                                "UsersParser::list - parse error (skip from result): {}",
                                error
                            )
                        }
                    };
                }
            }
            Err(err) => {
                println!("UserHandler::list - failed to list users: {}", err);
                return Err(Report::new(HandlerError::new(&err.to_string())));
            }
        }

        Ok(data)
    }
}
