use error_stack::Context;
use error_stack::Report;
use serde::Deserialize as Deserializable;
use serde_derive::Serialize;
use serde_json::Value;
use std::fmt;

use crate::models::common::CommonError;

pub struct Request;
impl Request {
    pub fn path_parameter(key: &str, event: &Value) -> Result<String, Report<PathParameterError>> {
        match event.get("pathParameters") {
            None => Err(Report::new(PathParameterError::new(
                "No path parameters found",
            ))),
            Some(params) => match params.get(key) {
                None => Err(Report::new(PathParameterError::new(
                    format!("Path parameter \"{}\" not found", key).as_str(),
                ))),
                Some(value) => Ok(value.to_string()),
            },
        }
    }

    pub fn body<'a, T: Deserializable<'a>>(body: &'a Value) -> Result<T, RequestError> {
        let body_str: &'a str = body.as_str().unwrap();
        match serde_json::from_str::<T>(&body_str) {
            Ok(valid) => Ok(valid),
            Err(err) => {
                println!("Body payload not compliant: {}", err);
                Err(CommonError::schema_compliant(
                    format!("Body payload not compliant: {}", err).to_string(),
                ))
            }
        }
    }
}

/* Request Error */
#[derive(Serialize, Debug)]
pub struct RequestError {
    pub code: String,
    pub message: String,
    pub details: String,
}
impl RequestError {
    pub fn new(code: String, message: String, details: String) -> Self {
        Self {
            code,
            message,
            details,
        }
    }

    pub fn internal() -> Self {
        Self {
            code: "ISE00".to_string(),
            message: "Internal Server Error".to_string(),
            details: "Something wrong happened, try again later".to_string(),
        }
    }

    pub fn path_parameter(param: String) -> Self {
        Self {
            code: "GRE100".to_string(),
            message: "Request Error".to_string(),
            details: format!("Path parameter error {}", param),
        }
    }
}

/* Path Parameter Error */
#[derive(Debug)]
pub struct PathParameterError {
    pub error: String,
}
impl PathParameterError {
    pub fn new(error: &str) -> Self {
        Self {
            error: String::from(error),
        }
    }
}
impl fmt::Display for PathParameterError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(format!("Path parameter error: {}", self.error).as_str())
    }
}
impl Context for PathParameterError {}
