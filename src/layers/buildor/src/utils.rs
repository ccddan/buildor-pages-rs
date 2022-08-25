use crate::models::common::RequiredEnvVarError;
use crate::models::request::PathParameterError;
use aws_sdk_dynamodb::Client as DynamoClient;
use error_stack::Report;
use serde_json::Value;

pub fn load_env_var(
    name: &str,
    default: Option<&str>,
) -> Result<String, Report<RequiredEnvVarError>> {
    match std::env::var(name) {
        Ok(val) => Ok(val),
        Err(err) => {
            println!("Env var \"{}\" does not exist. Error: {}", &name, err);
            match default {
                Some(default_value) => {
                    println!("Using default value: {}", &default_value);
                    Ok(String::from(default_value))
                }
                None => {
                    println!("No default value provided, paniking");
                    Err(Report::new(RequiredEnvVarError::new(name)))
                }
            }
        }
    }
}

#[cfg(test)]
mod load_env_var_tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Missing required env var: Undefined")]
    fn exception_when_undefined_env_var() -> () {
        let _ = load_env_var("Undefined", None).unwrap();
    }

    #[test]
    fn returns_default_value() -> () {
        let default_value = "default_value";
        let value = load_env_var("Undefined", Some(&default_value)).unwrap();
        assert_eq!(value, default_value);
    }

    #[test]
    fn returns_env_var_value() -> () {
        let _ = load_env_var("USER", None).unwrap();
    }
}

pub struct Clients;
impl Clients {
    pub async fn dynamodb() -> DynamoClient {
        let config = aws_config::load_from_env().await;
        DynamoClient::new(&config)
    }

    pub async fn codebuild() -> CodebuildClient {
        let config = aws_config::load_from_env().await;
        CodebuildClient::new(&config)
    }
}

#[cfg(test)]
mod clients_tests {
    use super::*;

    #[tokio::test]
    async fn resturns_dynamodb_client() {
        let _ = Clients::dynamodb().await;
    }

    #[tokio::test]
    async fn resturns_codebuild_client() {
        let _ = Clients::codebuild().await;
    }
}

pub fn get_path_parameter(key: &str, event: &Value) -> Result<String, Report<PathParameterError>> {
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

