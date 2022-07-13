use buildor::{
    handlers::users::UsersHandler,
    models::{response::Response, user::UserCreatePayload},
    utils::get_table_client,
};
use lambda_runtime::{service_fn, Error as LambdaError, LambdaEvent};
use serde_json::{json, Value};
use std::{borrow::Cow, env};

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

    println!("Parse body payload");
    let body: UserCreatePayload;
    let b = event["body"].to_owned();
    let foo: Cow<'_, str> = Cow::from(b.as_str().unwrap());

    match serde_json::from_str::<UserCreatePayload>(&foo) {
        Ok(valid) => body = valid,
        Err(err) => {
            println!("Body payload not compliant: {}", err);
            return Ok(json!({ "error": format!("{}", err) }));
        }
    }

    let table = get_table_client().await;
    let uh = UsersHandler::new(table, TABLE_NAME);
    let user = uh.create(body).await;

    match user {
        Some(user) => Ok(Response::new(json!(user), 200)),
        None => Ok(Response::new(
            json!({ "error": format!("Failed to create user, try again.") }),
            400,
        )),
    }
}
