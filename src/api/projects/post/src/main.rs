use error_stack::{Report, ResultExt};
use lambda_runtime::{service_fn, LambdaEvent};
use log::{self, error, info};
use serde_json::{json, Value};

use buildor::{
    handlers::projects::ProjectsHandler,
    models::{
        common::ExecutionError,
        handlers::HandlerCreate,
        project::{ProjectCreatePayload, ProjectError},
        request::{Request, RequestError},
        response::Response,
    },
    utils::{load_env_var, Clients},
};

#[tokio::main]
async fn main() -> Result<(), Value> {
    env_logger::init();

    info!("Creating service fn for handler");
    let func = service_fn(handler);
    info!("Executing handler from runtime");
    let result = lambda_runtime::run(func).await;
    info!("Evaluating handler result");
    match result {
        Ok(res) => {
            info!("Success");
            Ok(res)
        }
        Err(err) => {
            error!("Handler exception: {}", err);
            Err(json!(RequestError::internal()))
        }
    }
}

async fn handler(event: LambdaEvent<Value>) -> Result<Value, Report<ExecutionError>> {
    info!("Start handler execution");

    info!("Load env vars");
    #[allow(non_snake_case)]
    let TABLE_NAME = load_env_var("TABLE_NAME", None).change_context(ExecutionError)?;
    info!("TABLE_NAME: {}", TABLE_NAME);
    #[allow(non_snake_case)]
    let TABLE_REGION = load_env_var("TABLE_REGION", None).change_context(ExecutionError)?;
    info!("TABLE_REGION: {}", TABLE_REGION);

    info!("Parse event and context objects");
    let (event, context) = event.into_parts();
    info!("event: {:?}", event);
    info!("context: {:?}", context);

    info!("Parse body payload");
    let body = match Request::body::<ProjectCreatePayload>(&event["body"]) {
        Ok(value) => value,
        Err(error) => return Ok(Response::new(error, 400)),
    };
    info!("Body: {:?}", body);

    let table = Clients::dynamodb().await;
    let ph = ProjectsHandler::new(table, TABLE_NAME);

    match ph.create(body).await {
        Ok(project) => Ok(Response::new(project, 200)),
        Err(error) => {
            error!(
                "Failed to create project: {}",
                error.change_context(ExecutionError)
            );
            Ok(Response::new(ProjectError::creation_failed(), 400))
        }
    }
}
