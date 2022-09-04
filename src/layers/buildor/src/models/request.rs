use error_stack::Context;
use error_stack::Report;
use serde_derive::Serialize;
use serde_json::Value;
use std::fmt;

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
