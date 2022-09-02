use buildor::handlers::codebuild::CodeBuildHandler;
use buildor::utils::Clients;
use log::{self, info};

#[tokio::main]
async fn main() {
    env_logger::init();

    info!("Initialize CodeBuild client.");
    let codebuild_client = Clients::codebuild().await;
    let codebuild_project_name = String::from("App-Dynamically-Deploy-SPAs");
    info!("Define CodeBuild project name: {}", codebuild_project_name);

    info!("Initialize CodeBuildHnalder instance.");
    let cbh = CodeBuildHandler::new(codebuild_client, codebuild_project_name);
    info!("Codebuild Handler is ready.");

    info!("Fetch BuildInfo from build");
    let result = cbh
        .get("414026fb-4994-45e3-bd97-32a97bf47b8e".to_string())
        .await;
    info!("Handler result: {:?}", result);
}
