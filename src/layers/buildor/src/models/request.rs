use error_stack::Context;
use serde_derive::Serialize;
use std::fmt;

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
