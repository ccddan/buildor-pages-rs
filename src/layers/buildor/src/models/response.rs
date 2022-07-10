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
    pub fn new(body: Value, code: u16) -> Value {
        json!(Response {
            status_code: code,
            headers: ResponseHeaders::default(),
            body: body.to_string(),
        })
    }

    pub fn ok() -> Value {
        json!(Response {
            status_code: 204,
            headers: ResponseHeaders::default(),
            body: String::from(""),
        })
    }
}
