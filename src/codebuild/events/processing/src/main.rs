use error_stack::{Report, ResultExt};
use lambda_runtime::{service_fn, LambdaEvent};
use log::{self, error, info};
use serde_json::{json, Value};
use std::str::FromStr;

use buildor::{
    models::{
        codebuild::{BuildPhase, BuildPhaseStatus},
        common::ExecutionError,
        request::RequestError,
        response::Response,
    },
    utils::load_env_var,
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
    #[allow(non_snake_case)]
    let TABLE_NAME_PROJECTS =
        load_env_var("TABLE_NAME_PROJECTS", None).change_context(ExecutionError)?;
    info!("TABLE_NAME_PROJECTS: {}", TABLE_NAME_PROJECTS);

    info!("Parse event and context objects");
    let (event, context) = event.into_parts();
    info!("event: {}", event);
    info!("context: {:?}", context);

    // Get Build Details
    let details = match event.get("detail") {
        Some(details) => details.to_owned(),
        None => todo!(),
    };
    info!("Build details: {}", details);

    // Get Build ID
    let uuid = match details.get("build-id") {
        Some(value) => {
            info!("Build raw id: {}", value);
            match value.as_str() {
                Some(value_str) => {
                    let values: Vec<&str> = value_str.split(":").into_iter().collect();
                    String::from(values[values.len() - 1])
                }
                None => todo!(),
            }
        }
        None => todo!(),
    };
    info!("Build uuid: {}", uuid);

    // Get Phase Information
    let completed_phase = match details.get("completed-phase") {
        Some(value) => match value.as_str() {
            Some(phase) => match BuildPhase::from_str(phase) {
                Ok(parsed_value) => parsed_value,
                Err(_) => todo!(),
            },
            None => todo!(),
        },
        None => todo!(),
    };
    info!("Build completed phase: {}", completed_phase);
    let completed_phase_status = match details.get("completed-phase-status") {
        Some(value) => match value.as_str() {
            Some(status) => match BuildPhaseStatus::from_str(status) {
                Ok(parsed_status) => parsed_status,
                Err(_) => todo!(),
            },
            None => todo!(),
        },
        None => todo!(),
    };
    info!("Completed phase status: {}", completed_phase_status);

    Ok(Response::new(json!({ "data": "static output"}), 200))
}
