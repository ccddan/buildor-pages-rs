import { Duration, RemovalPolicy, StackProps } from "aws-cdk-lib";
import * as build from "aws-cdk-lib/aws-codebuild";
import * as s3 from "aws-cdk-lib/aws-s3";
import { Construct } from "constructs";
import config from "../config";
import { OutputStack } from "./utils/output-stack";



export class DeployStack extends OutputStack {
  constructor(scope: Construct, id: string, props: StackProps) {
    super(scope, id, props);

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
    });
  }
}
