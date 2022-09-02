use buildor::{
    handlers::codebuild::CodeBuildHandler,
    models::{
        commands::Commands,
        project::{Project, ProjectCreatePayload},
    },
    utils::Clients,
};

#[tokio::main]
async fn main() {
    env_logger::init();

    println!("Initialize CodeBuild client.");
    let codebuild_client = Clients::codebuild().await;
    let codebuild_project_name = String::from("App-Dynamically-Deploy-SPAs");
    println!("Define CodeBuild project name: {}", codebuild_project_name);

    println!("Initialize CodeBuildHnalder instance.");
    let cbh = CodeBuildHandler::new(codebuild_client, codebuild_project_name);
    println!("Codebuild Handler is ready.");

    println!("Create new build");
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
    println!("Newly created build: {:?}", build);

    println!("Fetch BuildInfo from build");
    let result = cbh
        .get("414026fb-4994-45e3-bd97-32a97bf47b8e".to_string())
        .await;
    println!("Handler result: {:?}", result);
}
