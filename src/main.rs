// use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client as DynamoClient;
use lambda_runtime::{service_fn, Error as LambdaError, LambdaEvent};
use serde::Deserialize;
use serde_json::{json, Value};
use std::env;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    println!("Creating service fn for handler");
    let func = service_fn(handler);
    println!("Executing handler from runtime");
    lambda_runtime::run(func).await?;
    Ok(())
}

#[derive(Deserialize, Debug)]
struct EventPayload {
    pub fname: String,
    pub lname: String,
}

async fn handler(event: LambdaEvent<EventPayload>) -> Result<Value, LambdaError> {
    println!("Start handler execution");

    println!("Load env vars");
    #[allow(non_snake_case)]
    let TABLE_NAME = env::var("TABLE_NAME").unwrap();
    #[allow(non_snake_case)]
    let TABLE_REGION: String = env::var("TABLE_REGION").unwrap();
    println!("TABLE_NAME: {}", TABLE_NAME);
    println!("TABLE_REGION: {}", TABLE_REGION);

    println!("Parse event and context objects");
    let (event, _context) = event.into_parts();
    println!("event: {:?}", event);

    // let region_provider = RegionProviderChain::default_provider().or_else("us-west-2");
    let config = aws_config::load_from_env().await;
    let table = DynamoClient::new(&config);

    println!("Extracting/Creating values to store in DB");
    let uuid = Uuid::new_v4().to_string();
    let fname = String::from(event.fname);
    let lname = String::from(event.lname);
    println!("uuid = {}", uuid);
    println!("fname = {}", fname);
    println!("lname = {}", lname);

    println!("Preparing to insert new record in db");
    let tx = table
        .put_item()
        .table_name(TABLE_NAME)
        .item("uuid", AttributeValue::S(uuid))
        .item("fname", AttributeValue::S(fname.to_string()))
        .item("lname", AttributeValue::S(lname.to_string()));

    println!("Send transaction");
    let result = tx.send().await;
    println!("Tx response: {:?}", result);

    match result {
        Ok(res) => {
            println!("New record created: {:?}", res);
            Ok(json!({
                "message": format!("{} {}, your request has been processed", fname, lname)
            }))
        }
        Err(err) => {
            println!("Failed to create record: {}", err);
            Ok(json!({
                "error": format!("Failed to create record: {}", err)
            }))
        }
    }
}
