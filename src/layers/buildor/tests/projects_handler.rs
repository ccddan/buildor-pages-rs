use buildor::handlers::projects::ProjectsHandler;
use buildor::utils::{load_env_var, Clients};

#[tokio::test]
async fn handler_init() {
    let _handler = ProjectsHandler::new(
        Clients::dynamodb().await,
        load_env_var("TABLE_NAME_PROJECTS", Some("Undefined")).unwrap(),
    );
}
