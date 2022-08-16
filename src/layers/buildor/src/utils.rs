use crate::models::common::RequiredEnvVarError;
use aws_sdk_codebuild::Client as CodebuildClient;
use aws_sdk_dynamodb::Client as DynamoClient;
use error_stack::Report;

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
