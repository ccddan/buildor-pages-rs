use buildor::handlers::users::UsersHandler;
use buildor::models::response::Response;
use buildor::utils::get_table_client;
use lambda_runtime::{service_fn, LambdaEvent};
use serde_json::{json, Value};

use error_stack::{Context, Report, ResultExt};
use std::fmt;

#[derive(Debug)]
struct MissingRequiredEnvVarError;
impl fmt::Display for MissingRequiredEnvVarError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(format!("Missing required env var").as_str())
    }
}
impl Context for MissingRequiredEnvVarError {}
// impl Error for MissingRequiredEnvVarError {}

#[derive(Debug)]
struct ExecutionError;
impl fmt::Display for ExecutionError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(format!("Execution error").as_str())
    }
}
impl Context for ExecutionError {}

fn load_env_var(name: &str) -> Result<String, Report<MissingRequiredEnvVarError>> {
    let value = std::env::var(name).or_else(|_| {
        Err(Report::new(MissingRequiredEnvVarError))
        // .change_context(MissingRequiredEnvVarError)
        // .attach_printable("{name}"))
    })?;

    Ok(value)
}

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
            Err(json!({ "error": format!("Internal server error") }))
        }
    }
}

async fn handler(event: LambdaEvent<Value>) -> Result<Value, Report<ExecutionError>> {
    println!("Start handler execution");

    println!("Load env vars");
    #[allow(non_snake_case)]
    let TABLE_NAME = load_env_var("TABLE_NAME").unwrap();
    #[allow(non_snake_case)]
    let TABLE_REGION = load_env_var("TABLE_REGION").unwrap();
    println!("TABLE_NAME: {}", TABLE_NAME);
    println!("TABLE_REGION: {}", TABLE_REGION);

    #[allow(non_snake_case)]
    let UNDEFINED_VAR = load_env_var("UNDEFINED_VAR").change_context(ExecutionError)?;
    println!("UNDEFINED_VAR: {}", UNDEFINED_VAR);

    println!("Parse event and context objects");
    let (event, context) = event.into_parts();
    println!("event: {:?}", event);
    println!("context: {:?}", context);

    let table = get_table_client().await;
    let uh = UsersHandler::new(table, TABLE_NAME);
    let users = uh.list().await;

    Ok(Response::new(json!({ "data": users }), 200))
}
