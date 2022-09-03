use error_stack::{Report, ResultExt};
use lambda_runtime::{service_fn, LambdaEvent};
use log::{self, error, info};
use serde_json::{json, Value};

use buildor::{
    handlers::{
        codebuild::CodeBuildHandler, project_deployments::ProjectDeploymentsHandler,
        projects::ProjectsHandler,
    },
    models::{
        common::{CommonError, ExecutionError},
        handlers::{HandlerCreate, HandlerGet},
        project_deployment::{
            ProjectDeploymentCreatePayload, ProjectDeploymentCreatePayloadRequest,
            ProjectDeploymentError,
        },
        request::RequestError,
        response::Response,
    },
    utils::{load_env_var, parse_request_body_payload, Clients},
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

    // ENVIRONMENT VARIABLES
    info!("Load env vars");
    #[allow(non_snake_case)]
    let TABLE_NAME = load_env_var("TABLE_NAME", None).change_context(ExecutionError)?;
    info!("TABLE_NAME: {}", TABLE_NAME);

    #[allow(non_snake_case)]
    let TABLE_NAME_PROJECTS =
        load_env_var("TABLE_NAME_PROJECTS", None).change_context(ExecutionError)?;
    info!("TABLE_NAME_PROJECTS: {}", TABLE_NAME_PROJECTS);

    #[allow(non_snake_case)]
    let TABLE_REGION = load_env_var("TABLE_REGION", None).change_context(ExecutionError)?;
    info!("TABLE_REGION: {}", TABLE_REGION);

    #[allow(non_snake_case)]
    let CODEBUILD_PROJECT_NAME =
        load_env_var("CODEBUILD_PROJECT_NAME", None).change_context(ExecutionError)?;
    info!("CODEBUILD_PROJECT_NAME: {}", CODEBUILD_PROJECT_NAME);

    info!("Parse event and context objects");
    let (event, context) = event.into_parts();
    info!("Event: {:?}", event);
    info!("Context: {:?}", context);

    // Body Payload
    info!("Parse body payload");
    let body =
        match parse_request_body_payload::<ProjectDeploymentCreatePayloadRequest>(&event["body"]) {
            Ok(value) => value,
            Err(err) => return Ok(json!(err)),
        };
    info!("Body: {:?}", body);

    let ph = ProjectsHandler::new(Clients::dynamodb().await, TABLE_NAME_PROJECTS);

    info!("Fetch project");
    let project = match ph.get(body.project_uuid).await {
        Ok(value) => match value {
            Some(project) => project,
            None => {
                return Ok(Response::new(
                    json!(CommonError::item_not_found(Some(
                        "Project not found".to_string()
                    ))),
                    404,
                ))
            }
        },
        Err(error) => {
            error!("Failed to get project: {}", error);
            return Ok(Response::new(json!(CommonError::item_not_found(None)), 404));
        }
    };
    info!("Project: {:?}", project);

    // CodeBuild Vars
    let cbh = CodeBuildHandler::new(Clients::codebuild().await, CODEBUILD_PROJECT_NAME);
    let pdh = ProjectDeploymentsHandler::new(Clients::dynamodb().await, TABLE_NAME);

    info!("Execute new codebuild build");
    match cbh.create(&project).await {
        Ok(build) => {
            info!("Build info: {:?}", build);
            info!("Create project deployment record");
            return match pdh
                .create(ProjectDeploymentCreatePayload { project, build })
                .await
            {
                Ok(value) => Ok(Response::new(json!(value), 201)),
                Err(error) => {
                    error!(
                        "Failed to create project deployment record: {}",
                        error.change_context(ExecutionError)
                    );
                    Ok(Response::new(
                        json!(ProjectDeploymentError::creation_failed()),
                        400,
                    ))
                }
            };
        }
        Err(err) => {
            error!("Error: {}", err);
            Ok(Response::new(json!({ "error": format!("{}", err) }), 200))
        }
    }
}
