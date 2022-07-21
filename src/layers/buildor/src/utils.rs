use crate::models::common::RequiredEnvVarError;
use aws_sdk_dynamodb::Client;
use error_stack::Report;

pub async fn get_table_client() -> Client {
    let config = aws_config::load_from_env().await;
    Client::new(&config)
}

pub fn load_env_var(name: &str) -> Result<String, Report<RequiredEnvVarError>> {
    let value =
        std::env::var(name).or_else(|_| Err(Report::new(RequiredEnvVarError::new(name))))?;

    Ok(value)
}
