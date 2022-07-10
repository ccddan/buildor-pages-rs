use aws_sdk_dynamodb::Client as DynamoClient;
use buildor::models::response::Response;
use lambda_runtime::{service_fn, Error as LambdaError, LambdaEvent};
use serde_json::{json, Value};
use std::env;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Value> {
    println!("Creating service fn for handler");
    let func = service_fn(handler);
    println!("Executing handler from runtime");
    let result = lambda_runtime::run(func).await;
    println!("Evaluating handler result");
    match result {
        Ok(res) => {
            println!("Success");
            Ok(res)
        }
        Err(err) => {
            println!("Handler exception: {}", err);
            Err(json!({ "error": format!("Internal error: {}", err) }))
        }
    }
}

async fn handler(event: LambdaEvent<Value>) -> Result<Value, LambdaError> {
    println!("Start handler execution");

    println!("Load env vars");
    #[allow(non_snake_case)]
    let TABLE_NAME = env::var("TABLE_NAME")?;
    #[allow(non_snake_case)]
    let TABLE_REGION: String = env::var("TABLE_REGION")?;
    println!("TABLE_NAME: {}", TABLE_NAME);
    println!("TABLE_REGION: {}", TABLE_REGION);

    println!("Parse event and context objects");
    let (event, context) = event.into_parts();
    println!("event: {:?}", event);
    println!("context: {:?}", context);

    let config = aws_config::load_from_env().await;
    let table = DynamoClient::new(&config);

    println!("Preparing to insert new record in db");
    let tx = table.scan().table_name(TABLE_NAME).into_paginator().items();
    println!("Send transaction");
    let result: Result<Vec<_>, _> = tx.send().collect().await;
    println!("Tx response: {:?}", result);

    // let items = result.unwrap();
    // Ok(())

    match result {
        Ok(res) => {
            println!("Listed records: {:?}", &res);
            let mut data: Vec<Value> = Vec::new();

            for item in &res {
                let uuid = item.get("uuid").unwrap().as_s().clone().unwrap();
                let fname = item.get("lname").unwrap().as_s().clone().unwrap();
                let lname = item.get("lname").unwrap().as_s().clone().unwrap();
                let record = json!({
                    "uuid": uuid,
                    "fname": fname,
                    "lname": lname,
                });
                println!("record: {:?}", json!(record));
                data.push(record);
            }
            Ok(Response::new(json!({ "data": data }), 200))
        }
        Err(err) => {
            println!("Failed to create record: {}", err);
            Ok(Response::new(
                json!({ "error": format!("Failed to create record: {}", err) }),
                400,
            ))
        }
    }
}
