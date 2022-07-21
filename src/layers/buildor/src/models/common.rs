use error_stack::Context;
use std::fmt;

#[derive(Debug)]
pub struct RequiredEnvVarError {
    pub name: String,
}
impl RequiredEnvVarError {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}
impl fmt::Display for RequiredEnvVarError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(format!("Missing required env var: {}", self.name).as_str())
    }
}
impl Context for RequiredEnvVarError {}

#[derive(Debug)]
pub struct ExecutionError;
impl fmt::Display for ExecutionError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(format!("Execution error").as_str())
    }
}
impl Context for ExecutionError {}
