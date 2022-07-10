use lambda_runtime::{service_fn, Error as LambdaError, LambdaEvent};
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Serialize, Debug)]
pub struct ResponseHeaders {
    #[serde(rename(serialize = "Content-Type"))]
    pub content_type: String,

    #[serde(rename(serialize = "Access-Control-Allow-Origin"))]
    pub access_control_allow_origin: String,

    #[serde(rename(serialize = "Access-Control-Allow-Credentials"))]
    pub access_control_allow_credentials: String,

    #[serde(rename(serialize = "X-Requested-With"))]
    pub x_requested_with: String,

    #[serde(rename(serialize = "Access-Control-Allow-Headers"))]
    pub access_control_allow_headers: String,

    #[serde(rename(serialize = "Access-Control-Allow-Methods"))]
    pub access_control_allow_methods: String,

    #[serde(rename(serialize = "Access-Control-Expose-Headers"))]
    pub access_control_expose_headers: String,
}

impl ResponseHeaders {
    pub fn default() -> Self {
        ResponseHeaders {
            content_type: "application/json".to_string(),
            access_control_allow_origin: "*".to_string(),
            access_control_allow_credentials: "false".to_string(),
            x_requested_with: "*".to_string(),
            access_control_allow_headers: "Accept,Content-Type,Authorization,X-Amz-Date,X-Api-Key,X-Amz-User-Agent,X-Requested-With,X-Amz-Security-Token".to_string(),
            access_control_allow_methods: "OPTIONS,HEAD,GET,POST,PUT,PATCH,DELETE".to_string(),
            access_control_expose_headers: "Authorization,X-Requested-With".to_string(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Response {
    #[serde(rename(serialize = "statusCode"))]
    pub status_code: u16,
    pub headers: ResponseHeaders,
    pub body: String,
}

impl Response {
    pub fn ok(body: Value) -> Value {
        json!(Response {
            status_code: 200,
            headers: ResponseHeaders::default(),
            body: body.to_string(),
        })
    }
}

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

async fn handler(_: LambdaEvent<Value>) -> Result<Value, LambdaError> {
    println!("Start handler execution");
    Ok(Response::ok(json!({ "message": "Buildor API" })))
}
