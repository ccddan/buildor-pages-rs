use error_stack::{Report, ResultExt};
use lambda_runtime::{service_fn, LambdaEvent};
use log::{self, error, info};
use serde_json::{json, Value};

use buildor::{
    handlers::project_deployments::ProjectDeploymentsHandler,
    models::{
        common::{CommonError, ExecutionError},
        handlers::HandlerGet,
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

    let deployment_uuid = match Request::path_parameter("deployment", &event) {
        Ok(uuid) => uuid,
        Err(error) => {
            error!("Path parameter error: {}", error.to_string());
            return Ok(Response::new(
                RequestError::path_parameter("deployment".to_string()),
                400,
            ));
        }
    };
    info!("uuid: {}", deployment_uuid);

    let pdh = ProjectDeploymentsHandler::new(Clients::dynamodb().await, TABLE_NAME);
    info!("Fetch project deployment object");
    match pdh.get(deployment_uuid).await {
        Ok(value) => match value {
            Some(deployment) => {
                info!("Deployment: {:?}", deployment);
                Ok(Response::new(deployment, 200))
            }
            None => {
                info!("Deployment not found");
                Ok(Response::new(
                    CommonError::item_not_found(Some("Project deployment not found".to_string())),
                    404,
                ))
            }
        },
        Err(error) => {
            error!("Failed to retrieve object from db: {}", error);
            Err(error.change_context(ExecutionError))
        }
    }
}
