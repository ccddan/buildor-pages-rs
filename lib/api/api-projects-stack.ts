import {
  Architecture,
  AssetCode,
  Function,
  Runtime,
} from "aws-cdk-lib/aws-lambda";
import { Duration, Stack, StackProps } from "aws-cdk-lib";
import { Effect, PolicyStatement } from "aws-cdk-lib/aws-iam";

import { Construct } from "constructs";

export class APIProjectsStack extends Stack {
  private readonly srcPath = "src/codebuild";
  public static readonly pathProjects = "projects";

  public readonly deploy: Function;

  constructor(scope: Construct, id: string, props: StackProps) {
    super(scope, id, props);

    // Deploy project
    this.deploy = new Function(this, "deploy", {
      description: "Deploy SPA project",
      runtime: Runtime.NODEJS_16_X,
      code: AssetCode.fromAsset(`${this.srcPath}/start`),
      architecture: Architecture.X86_64,
      handler: "codebuild-start.handler",
      timeout: Duration.seconds(5),
    });
    this.deploy.addToRolePolicy(
      new PolicyStatement({
        effect: Effect.ALLOW,
        actions: ["codebuild:StartBuild"],
        resources: [
          "arn:aws:codebuild:us-west-2:995360066764:project/App-Dynamically-Deploy-SPAs",
        ], // TODO: fetch from SSM
      })
    );
  }
}
