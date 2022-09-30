use buildor::{
    handlers::codebuild::CodeBuildHandler,
    models::{
        commands::Commands,
        project::{Project, ProjectCreatePayload},
    },
    utils::Clients,
};
use log::{self, error, info};

const CODEBUILD_PROJECT_NAME_BUILDING: &str = "App-Dynamically-Deploy-SPAs";
const CODEBUILD_PROJECT_NAME_DEPLOYMENT: &str = "CODEBUILD_PROJECT_NAME_DEPLOYMENT"; // TODO: replace with deployment codebuild project name

#[tokio::main]
async fn main() {
    env_logger::init();

    info!("Initialize Handlers");
    let cbh = CodeBuildHandler::new(
        Clients::codebuild().await,
        CODEBUILD_PROJECT_NAME_BUILDING.clone().to_string(),
        CODEBUILD_PROJECT_NAME_DEPLOYMENT.clone().to_string(),
    );

    // CodeBuild - New Build (Project Deployment)
    info!("Create new build");
    let project_create_payload = ProjectCreatePayload {
        name: "buildspace-solana-pay".to_string(),
        repository: "https://github.com/ccddan/buildspace-solana-pay.git".to_string(),
        commands: Some(Commands::new(
            Some(vec!["npm install".to_string()]),
            Some(vec!["npm run release".to_string()]),
        )),
        output_folder: Some("out".to_string()),
    };
    let project = Project::new(project_create_payload);
    let build = cbh.create(&project).await;
    info!("Newly created build: {:?}", build);

    info!("Fetch BuildInfo from build");
    let result = cbh
        .get("414026fb-4994-45e3-bd97-32a97bf47b8e".to_string())
        .await;
    info!("Handler result: {:?}", result);
}
