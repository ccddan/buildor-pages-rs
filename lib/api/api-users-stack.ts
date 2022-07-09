import {
  Architecture,
  AssetCode,
  Function,
  Runtime,
} from "aws-cdk-lib/aws-lambda";
import { Duration, Stack, StackProps } from "aws-cdk-lib";
import { Tables, TablesStack } from "../tables-stack";

import { Construct } from "constructs";

export class APIUsersStack extends Stack {
  private readonly srcPath = "target/lambda";

  public readonly post: Function;

  constructor(scope: Construct, id: string, props: StackProps) {
    super(scope, id, props);

    // dependencies
    const usersTable = TablesStack.getStreamingInstance(this, Tables.Users);

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
    TablesStack.grantReadWriteIndex(usersTable, this.post);
  }
}
