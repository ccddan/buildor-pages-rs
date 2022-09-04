import {Duration, Stack, StackProps} from "aws-cdk-lib";
import {LambdaIntegration} from "aws-cdk-lib/aws-apigateway";
import {
  Architecture,
  AssetCode,
  Function,
  Runtime
} from "aws-cdk-lib/aws-lambda";
import {Construct} from "constructs";
import {Tables, TablesStack} from "../tables-stack";
import {APIStack} from "./api-stack";
import config from "../../config";


export class APIProjectsStack extends Stack {
  private readonly srcPath = "target/lambda";
  public static readonly pathProjects = "projects";
  public static readonly pathProject = `${APIProjectsStack.pathProjects}/{project}`;

  public readonly post: Function;
  public readonly list: Function;

  constructor(scope: Construct, id: string, props: StackProps) {
    super(scope, id, props);

    // Share dependencies
    const projectsTable = TablesStack.getStreamingInstance(
      this,
      Tables.Projects
    );

    // Create new project
    this.post = new Function(this, "post", {
      description: "Create new project",
      runtime: Runtime.PROVIDED_AL2,
      code: AssetCode.fromAsset(
        `${this.srcPath}/api-projects-post/bootstrap.zip`
      ),
      handler: "bootstrap",
      environment: {
        RUST_BACKTRACE: "1",
        RUST_LOG: config.api.resources.projects.post.logging,
        TABLE_NAME: projectsTable.tableName,
        TABLE_REGION: props.env!.region!,
      },
      timeout: Duration.seconds(5),
    });
    projectsTable.grantWriteData(this.post);
    this.post.grantInvoke(APIStack.principal);

    // List Projects
    this.list = new Function(this, "list", {
      description: "List projects",
      runtime: Runtime.PROVIDED_AL2,
      code: AssetCode.fromAsset(
        `${this.srcPath}/api-projects-list/bootstrap.zip`
      ),
      architecture: Architecture.X86_64,
      handler: "bootstrap",
      environment: {
        RUST_BACKTRACE: "1",
        RUST_LOG: config.api.resources.projects.list.logging,
        TABLE_NAME: projectsTable.tableName,
        TABLE_REGION: props.env!.region!,
      },
      timeout: Duration.seconds(5),
    });
    projectsTable.grantReadData(this.list);
    this.list.grantInvoke(APIStack.principal);

    // API Endpoints
    const api = APIStack.getInstance(this);
    const rootResource = APIStack.getRootResource(this, api);

    const projects = rootResource.addResource(APIProjectsStack.pathProjects);
    projects.addMethod("POST", new LambdaIntegration(this.post));
    projects.addMethod("GET", new LambdaIntegration(this.list));
  }
}
