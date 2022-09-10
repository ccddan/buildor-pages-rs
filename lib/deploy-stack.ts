import {
  Duration,
  RemovalPolicy,
  StackProps,
  aws_logs as logs,
  aws_lambda as lambdas,
  aws_codebuild as build,
  aws_s3 as s3,
  aws_events_targets as targets,
} from "aws-cdk-lib";
import {Construct} from "constructs";
import config from "../config";
import {OutputStack} from "./utils/output-stack";
import {TablesStack, Tables} from "./tables-stack";

export class DeployStack extends OutputStack {

  private readonly srcPath = "target/lambda";

  constructor(scope: Construct, id: string, props: StackProps) {
    super(scope, id, props);

    // dependencies
    const projectDeployments = TablesStack.getStreamingInstance(this, Tables.ProjectDeployments);

    const artifactsBucket = new s3.Bucket(this, "deploy-spas-artifacts", {
      bucketName: config.app.name("-deploy-spas-artifacts").toLowerCase(),
      autoDeleteObjects: true,
      blockPublicAccess: s3.BlockPublicAccess.BLOCK_ALL,
      lifecycleRules: [
        {
          id: "auto-delete",
          enabled: true,
          expiration: Duration.days(1),
          abortIncompleteMultipartUploadAfter: Duration.days(1),
        },
      ],
      removalPolicy: RemovalPolicy.DESTROY,
    });
    let buildEventsProcessingFn = new lambdas.Function(this, "build-events-processing", {
      description: "Process codebuild execution status",
      runtime: lambdas.Runtime.PROVIDED_AL2,
      code: lambdas.AssetCode.fromAsset(`${this.srcPath}/codebuild-events-processing/bootstrap.zip`),
      architecture: lambdas.Architecture.X86_64,
      handler: "bootstrap",
      environment: {
        RUST_BACKTRACE: "1",
        RUST_LOG: config.codebuild.events.processing.logging,
        TABLE_NAME: projectDeployments.tableName,
        TABLE_REGION: props.env!.region!,
      },
      timeout: Duration.seconds(5),
    });
    projectDeployments.grantReadWriteData(buildEventsProcessingFn);

    const deploy = new build.Project(this, "deploy", {
      projectName: config.app.name("-Dynamically-Deploy-SPAs"),
      environment: {
        buildImage: build.LinuxBuildImage.STANDARD_5_0,
      },
      buildSpec: build.BuildSpec.fromObject({
        version: "0.2",
        phases: {
          install: {
            commands: [
              "echo Download project",
              "git clone $REPO_URL $PROJECT_NAME",
            ],
          },
          pre_build: {
            commands: [
              "echo Install project dependencies",
              "cd $PROJECT_NAME",
              "npm install",
            ],
          },
          build: {
            commands: [
              "echo Build project",
              "npm run release",
              "echo Move build output to artifacts location",
              "mv out dist",
            ],
          },
          post_build: {
            commands: ["echo Build has completed and artifacts were moved"],
          },
        },
        artifacts: {
          "discard-paths": "no",
          files: ["dist"],
        },
      }),
      artifacts: build.Artifacts.s3({
        bucket: artifactsBucket,
        includeBuildId: true,
      }),
      logging: {
        cloudWatch: {
          logGroup: new logs.LogGroup(this, config.app.name("project-deployment-logs")),
        },
      },
    });
    deploy.onEvent(config.app.name("codebuild-events"), {
      description: "Send codebuild events to processing lambda",
      target: new targets.LambdaFunction(buildEventsProcessingFn, {
        retryAttempts: 3,
      }),
      eventPattern: {
        detail: {
          "completed-phase": [
            "SUBMITTED",
            "PROVISIONING",
            "DOWNLOAD_SOURCE",
            "INSTALL",
            "PRE_BUILD",
            "BUILD",
            "POST_BUILD",
            "UPLOAD_ARTIFACTS",
            "FINALIZING"
          ],
          "completed-phase-status": [
            "TIMED_OUT",
            "STOPPED",
            "FAILED",
            "SUCCEEDED",
            "FAULT",
            "CLIENT_ERROR"
          ],
        },
      }
    });

    this.outputSSM(config.app.name("CodebuildProjectNameSSM"), config.ssm.codebuild.project.name, deploy.projectName);
    this.outputSSM(config.app.name("CodebuildProjectARNSSM"), config.ssm.codebuild.project.arn, deploy.projectArn);
  }
}
