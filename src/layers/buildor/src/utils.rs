use crate::models::common::RequiredEnvVarError;
use aws_sdk_dynamodb::Client;
use error_stack::Report;

pub async fn get_table_client() -> Client {
    let config = aws_config::load_from_env().await;
    Client::new(&config)
}

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
