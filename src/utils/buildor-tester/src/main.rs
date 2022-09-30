use buildor::{
    handlers::{codebuild::CodeBuildHandler, project_deployments::ProjectDeploymentsHandler},
    models::{
        codebuild::{BuildInfo, BuildPhase, BuildPhaseStatus, ProjectDeploymentPhase},
        commands::Commands,
        handlers::{HandlerCreate, HandlerUpdate},
        project::{Project, ProjectCreatePayload},
        project_deployment::{ProjectDeploymentCreatePayload, ProjectDeploymentUpdatePayload},
    },
    utils::Clients,
};
use log::{self, info};

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
    let project = Project::new(project_create_payload.clone());

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
    let mut build_updated = build_info.clone();
    build_updated.build_number = Some(3);
    build_updated.start_time = Some(3);
    build_updated.end_time = Some(3);
    build_updated.deployment_phase = Some(ProjectDeploymentPhase::Deployment.to_string());
    build_updated.current_phase = Some(BuildPhase::Finalizing.to_string());

    // =========================== CODEBUILD ===========================
    info!("====================== CodeBuild ======================");
    info!("Create New Build");
    let result = cbh.create(&project.clone()).await;
    info!("New Build: {:?}", result);

    info!("Get Existing Build");
    let result = cbh
        // .get("414026fb-4994-45e3-bd97-32a97bf47b8e".to_string())
        .get(result.unwrap().uuid)
        .await;
    info!("Build: {:?}", result);

    // =========================== PROJECT DEPLOYMENTS ===========================
    info!("====================== Project Deployments ======================");
    let build = build_info.clone();
    let project_deployment_create_payload = ProjectDeploymentCreatePayload {
        project: project.clone(),
        build,
    };

    info!("Create New Project Deployment");
    let result = pdh.create(project_deployment_create_payload).await;
    info!("New Project Deployment Result: {:?}", result);

    info!("Update Project Deployment");
    let result = pdh
        .update(
            result.unwrap().uuid,
            ProjectDeploymentUpdatePayload {
                project: Some(project.clone()),
                build: Some(build_updated.clone()),
            },
        )
        .await;
    info!("Project Deployment Updated Result: {:?}", result);
}
