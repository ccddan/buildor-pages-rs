import {
  Architecture,
  AssetCode,
  Function,
  Runtime,
} from "aws-cdk-lib/aws-lambda";
import {Duration, Stack, StackProps} from "aws-cdk-lib";
import {Effect, PolicyStatement} from "aws-cdk-lib/aws-iam";
import {Tables, TablesStack} from "../tables-stack";

import {APIStack} from "./api-stack";
import {Construct} from "constructs";
import {LambdaIntegration} from "aws-cdk-lib/aws-apigateway";
import {StringParameter} from "aws-cdk-lib/aws-ssm";
import config from "../../config";

export class APIProjectDeploymentsStack extends Stack {
  private readonly srcPath = "target/lambda";
  public static readonly pathDeployments = "deployments";
  public static readonly pathDeployment = `${APIProjectDeploymentsStack.pathDeployments}/{deployment}`;

  public readonly post: Function;
  public readonly get: Function;

  constructor(scope: Construct, id: string, props: StackProps) {
    super(scope, id, props);

    // Share dependencies
    const deploymentsTable = TablesStack.getStreamingInstance(
      this,
      Tables.ProjectDeployments
    );
    const projectsTable = TablesStack.getStreamingInstance(
      this,
      Tables.Projects
    );
    const codeBuildProjectName = StringParameter.fromStringParameterName(
      this,
      "CodebuildProjectNameValue",
      config.ssm.codebuild.project.name
    ).stringValue;
    const codeBuildProjectARN = StringParameter.fromStringParameterName(
      this,
      "CodebuildProjectARNValue",
      config.ssm.codebuild.project.arn,
    ).stringValue;

    // Create new project deployment
    this.post = new Function(this, "post", {
      description: "Create new project deployment",
      runtime: Runtime.PROVIDED_AL2,
      code: AssetCode.fromAsset(
        `${this.srcPath}/api-project-deployments-post/bootstrap.zip`
      ),
      handler: "bootstrap",
      environment: {
        RUST_BACKTRACE: "1",
        RUST_LOG: config.api.resources.projectDeployments.post.logging,
        TABLE_NAME: deploymentsTable.tableName,
        TABLE_REGION: props.env!.region!,
        TABLE_NAME_PROJECTS: projectsTable.tableName,
        CODEBUILD_PROJECT_NAME_BUILDING: codeBuildProjectName,
      },
      timeout: Duration.seconds(5),
    });
    deploymentsTable.grantReadWriteData(this.post);
    projectsTable.grantReadWriteData(this.post);
    this.post.grantInvoke(APIStack.principal);
    this.post.addToRolePolicy(
      new PolicyStatement({
        effect: Effect.ALLOW,
        actions: ["codebuild:StartBuild"],
        resources: [codeBuildProjectARN],
      })
    );

    // Get deployment status
    this.get = new Function(this, "get", {
      description: "Get project deployment status",
      runtime: Runtime.PROVIDED_AL2,
      code: AssetCode.fromAsset(
        `${this.srcPath}/api-project-deployments-get/bootstrap.zip`
      ),
      architecture: Architecture.X86_64,
      handler: "bootstrap",
      environment: {
        RUST_BACKTRACE: "1",
        RUST_LOG: config.api.resources.projectDeployments.deployment.get.logging,
        TABLE_NAME: deploymentsTable.tableName,
        TABLE_REGION: props.env!.region!,
      },
      timeout: Duration.seconds(5),
    });
    this.get.grantInvoke(APIStack.principal);
    deploymentsTable.grantReadData(this.get);

    // API Endpoints
    const api = APIStack.getInstance(this);
    const rootResource = APIStack.getRootResource(this, api);

    const deployments = rootResource.addResource(
      APIProjectDeploymentsStack.pathDeployments
    );
    deployments.addMethod("POST", new LambdaIntegration(this.post));

    const deployment = rootResource.resourceForPath(
      APIProjectDeploymentsStack.pathDeployment
    );
    deployment.addMethod("GET", new LambdaIntegration(this.get));
  }
}
