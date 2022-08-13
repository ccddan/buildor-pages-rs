use buildor::{
    handlers::projects::ProjectsHandler,
    models::{
        common::{CommonError, ExecutionError},
        project::{ProjectCreatePayload, ProjectError},
        request::RequestError,
        response::Response,
    },
    utils::{get_table_client, load_env_var},
};
use error_stack::{Report, ResultExt};
use lambda_runtime::{service_fn, LambdaEvent};
use serde_json::{json, Value};
use std::borrow::Cow;

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

    println!("Parse body payload");
    let body: ProjectCreatePayload;
    let b = event["body"].to_owned();
    let foo: Cow<'_, str> = Cow::from(b.as_str().unwrap());

    match serde_json::from_str::<ProjectCreatePayload>(&foo) {
        Ok(valid) => body = valid,
        Err(err) => {
            println!("Body payload not compliant: {}", err);
            return Ok(Response::new(
                json!(CommonError::schema_compliant(
                    format!("Body payload not compliant: {}", err).to_string()
                )),
                400,
            ));
        }
    }

    let table = get_table_client().await;
    let ph = ProjectsHandler::new(table, TABLE_NAME);
    let user = ph.create(body).await;

    match user {
        Some(user) => Ok(Response::new(json!(user), 200)),
        None => Ok(Response::new(json!(ProjectError::creation_failed()), 400)),
    }
}
