import { Duration, Stack, StackProps } from "aws-cdk-lib";
import { LambdaIntegration } from "aws-cdk-lib/aws-apigateway";
import {
    Architecture,
    AssetCode,
    Function,
    Runtime
} from "aws-cdk-lib/aws-lambda";
import { Construct } from "constructs";
import { Tables, TablesStack } from "../tables-stack";
import { APIStack } from "./api-stack";


export class APIUsersStack extends Stack {
  private readonly srcPath = "target/lambda";
  public static readonly pathUsers = "users";

  public readonly post: Function;
  public readonly list: Function;

  constructor(scope: Construct, id: string, props: StackProps) {
    super(scope, id, props);

    // dependencies
    const usersTable = TablesStack.getStreamingInstance(this, Tables.Users);

    // Create new users
    this.post = new Function(this, "post", {
      description: "Create new users",
      runtime: Runtime.PROVIDED_AL2,
      code: AssetCode.fromAsset(`${this.srcPath}/api-users-post/bootstrap.zip`),
      architecture: Architecture.X86_64,
      handler: "bootstrap",
      environment: {
        RUST_BACKTRACE: "1",
        TABLE_NAME: usersTable.tableName,
        TABLE_REGION: props.env!.region!,
      },
      timeout: Duration.seconds(5),
    });
    usersTable.grantWriteData(this.post);
    this.post.grantInvoke(APIStack.principal);

    // List users
    this.list = new Function(this, "list", {
      description: "List users",
      runtime: Runtime.PROVIDED_AL2,
      code: AssetCode.fromAsset(`${this.srcPath}/api-users-list/bootstrap.zip`),
      architecture: Architecture.X86_64,
      handler: "bootstrap",
      environment: {
        RUST_BACKTRACE: "1",
        TABLE_NAME: usersTable.tableName,
        TABLE_REGION: props.env!.region!,
      },
      timeout: Duration.seconds(5),
    });
    usersTable.grantReadData(this.list);
    this.list.grantInvoke(APIStack.principal);

    // API Endpoints
    const api = APIStack.getInstance(this);
    const rootResource = APIStack.getRootResource(this, api);

    const users = rootResource.addResource(APIUsersStack.pathUsers);
    users.addMethod("POST", new LambdaIntegration(this.post));
    users.addMethod("GET", new LambdaIntegration(this.list));
  }
}
