use lambda_runtime::{service_fn, Error as LambdaError, LambdaEvent};
use log::{self, error, info};
use serde_json::{json, Value};

use buildor::models::{request::RequestError, response::Response};

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

async fn handler(_: LambdaEvent<Value>) -> Result<Value, LambdaError> {
    info!("Start handler execution");
    Ok(Response::new(json!({ "message": "Buildor API" }), 200))
}
