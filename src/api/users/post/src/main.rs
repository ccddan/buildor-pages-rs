use error_stack::{Report, ResultExt};
use lambda_runtime::{service_fn, LambdaEvent};
use serde_json::{json, Value};

use buildor::models::handlers::HandlerCreate;
use buildor::{
    handlers::users::UsersHandler,
    models::{
        common::ExecutionError,
        request::RequestError,
        response::Response,
        user::{UserCreatePayload, UserError},
    },
    utils::{load_env_var, parse_request_body_payload, Clients},
};

#[tokio::main]
async fn main() -> Result<(), Value> {
    env_logger::init();

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
            Err(json!(RequestError::internal()))
        }
    }
}

async fn handler(event: LambdaEvent<Value>) -> Result<Value, Report<ExecutionError>> {
    println!("Start handler execution");

    println!("Load env vars");
    #[allow(non_snake_case)]
    let TABLE_NAME = load_env_var("TABLE_NAME", None).change_context(ExecutionError)?;
    #[allow(non_snake_case)]
    let TABLE_REGION = load_env_var("TABLE_REGION", None).change_context(ExecutionError)?;
    println!("TABLE_NAME: {}", TABLE_NAME);
    println!("TABLE_REGION: {}", TABLE_REGION);

    println!("Parse event and context objects");
    let (event, context) = event.into_parts();
    println!("event: {:?}", event);
    println!("context: {:?}", context);

    // Body Payload
    println!("Parse body payload");
    let body = match parse_request_body_payload::<UserCreatePayload>(&event["body"]) {
        Ok(value) => value,
        Err(err) => return Ok(json!(err)),
    };
    println!("Body: {:?}", body);

    let table = Clients::dynamodb().await;
    let uh = UsersHandler::new(table, TABLE_NAME);

    match uh.create(body).await {
        Ok(user) => Ok(Response::new(json!(user), 200)),
        Err(error) => {
            println!(
                "Failed to create user: {}",
                error.change_context(ExecutionError)
            );
            Ok(Response::new(json!(UserError::creation_failed()), 400))
        }
    }
}
