use buildor::{
    handlers::{codebuild::CodeBuildHandler, project_deployments::ProjectDeploymentsHandler},
    models::{
        codebuild::{BuildInfo, BuildPhase, BuildPhaseStatus, ProjectDeploymentPhase},
        commands::Commands,
        handlers::{
            HandlerCreate, HandlerDelete, HandlerError, HandlerGet, HandlerList, HandlerUpdate,
        },
        project::{Project, ProjectCreatePayload},
        project_deployment::{ProjectDeployment, ProjectDeploymentCreatePayload},
    },
    utils::Clients,
};
use log::{self, error, info};

const CODEBUILD_PROJECT_NAME_BUILDING: &str = "App-Dynamically-Deploy-SPAs";
const CODEBUILD_PROJECT_NAME_DEPLOYMENT: &str = "CODEBUILD_PROJECT_NAME_DEPLOYMENT"; // TODO: replace with deployment codebuild project name
const TABLE_NAME_PROJECT_DEPLOYMENTS: &str =
    "AppTablesStack-ProjectDeploymentsB59EBD6B-1B28D1F3R8GYR";

#[tokio::main]
async fn main() {
    env_logger::init();

    // =========================== HANDLERS ===========================
    info!("Initialize Handlers");
    let pdh = ProjectDeploymentsHandler::new(
        Clients::dynamodb().await,
        TABLE_NAME_PROJECT_DEPLOYMENTS.clone().to_string(),
    );
    let cbh = CodeBuildHandler::new(
        Clients::codebuild().await,
        CODEBUILD_PROJECT_NAME_BUILDING.clone().to_string(),
        CODEBUILD_PROJECT_NAME_DEPLOYMENT.clone().to_string(),
    );

    // =========================== PAYLOADS ===========================
    // Project
    let project_create_payload = ProjectCreatePayload {
        name: "buildspace-solana-pay".to_string(),
        repository: "https://github.com/ccddan/buildspace-solana-pay.git".to_string(),
        commands: Some(Commands::new(
            Some(vec!["npm install".to_string()]),
            Some(vec!["npm run release".to_string()]),
        )),
        output_folder: Some("out".to_string()),
    };
    // Build
    let build_info = BuildInfo {
        uuid: String::from("build-id"),
        build_number: Some(1),
        start_time: Some(1),
        end_time: Some(1),
        deployment_phase: Some(ProjectDeploymentPhase::Building.to_string()),
        current_phase: Some(BuildPhase::Queued.to_string()),
        build_status: Some(BuildPhaseStatus::InProgress.to_string()),
    };

    // =========================== CODEBUILD ===========================
    info!("====================== CodeBuild ======================");
    info!("Create New Build");
    let project = Project::new(project_create_payload.clone());
    let result = cbh.create(&project).await;
    info!("New Build: {:?}", result);

    info!("Get Existing Build");
    let result = cbh
        .get("414026fb-4994-45e3-bd97-32a97bf47b8e".to_string())
        .await;
    info!("Build: {:?}", result);

    // =========================== PROJECT DEPLOYMENTS ===========================
    info!("====================== Project Deployments ======================");
    let project = Project::new(project_create_payload.clone());
    let build = build_info.clone();
    let project_deployment_create_payload = ProjectDeploymentCreatePayload { project, build };

    info!("Create New Project Deployment");
    let result = pdh.create(project_deployment_create_payload).await;
    info!("New Project Deployment: {:?}", result);
}
