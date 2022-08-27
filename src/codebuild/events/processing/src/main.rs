use buildor::{
    models::{common::ExecutionError, request::RequestError, response::Response},
    utils::load_env_var,
};
use error_stack::{Report, ResultExt};
use lambda_runtime::{service_fn, LambdaEvent};
use serde_json::{json, Value};

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

    Ok(Response::new(json!({ "data": "statics output"}), 200))
}
