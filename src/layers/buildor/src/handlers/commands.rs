use std::collections::HashMap;

use crate::models::commands::Commands;
use aws_sdk_dynamodb::model::AttributeValue;
use error_stack::{Context, Report};
use std::fmt;

#[derive(Debug)]
pub struct MissingRequiredCommandError {
    pub name: String,
}
impl MissingRequiredCommandError {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}
impl fmt::Display for MissingRequiredCommandError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(format!("Missing required command: {}", self.name).as_str())
    }
}
impl Context for MissingRequiredCommandError {}

pub struct CommandsParser;
impl CommandsParser {
    pub fn parse(
        item: HashMap<String, AttributeValue>,
    ) -> Result<Commands, Report<MissingRequiredCommandError>> {
        let pre_build = match item.get("pre_build") {
            None => return Err(Report::new(MissingRequiredCommandError::new("pre_build"))),
            Some(value) => value,
        };
        let pre_build = pre_build
            .as_l()
            .unwrap()
            .iter()
            .map(|command| command.as_s().unwrap().to_string())
            .collect::<Vec<String>>();

        let build = match item.get("build") {
            None => return Err(Report::new(MissingRequiredCommandError::new("build"))),
            Some(value) => value,
        };
        let build = build
            .as_l()
            .unwrap()
            .iter()
            .map(|command| command.as_s().unwrap().to_string())
            .collect::<Vec<String>>();
        Ok(Commands::new(Some(pre_build), Some(build)))
    }
}
