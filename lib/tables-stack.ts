import { StackProps } from "aws-cdk-lib";
import {
    AttributeType,
    BillingMode,
    ITable,
    StreamViewType,
    Table
} from "aws-cdk-lib/aws-dynamodb";
import { Effect, PolicyStatement } from "aws-cdk-lib/aws-iam";
import { Function } from "aws-cdk-lib/aws-lambda";
import { StringParameter } from "aws-cdk-lib/aws-ssm";
import { Construct } from "constructs";
import config from "../config";
import { OutputStack } from "./utils/output-stack";


export enum Tables {
  Users = "Users",
  Projects = "Projects",
  ProjectDeployments = "ProjectDeployments",
}

export class TablesStack extends OutputStack {
  constructor(scope: Construct, id: string, props: StackProps) {
    super(scope, id, props);

    const users = new Table(this, Tables.Users, {
      partitionKey: { name: "uuid", type: AttributeType.STRING },
      billingMode: BillingMode.PAY_PER_REQUEST,
      stream: StreamViewType.NEW_AND_OLD_IMAGES,
    });

    this.outputSSM(
      config.app.name(`${Tables.Users}SSM`),
      config.ssm.tables.users.tableArn,
      users.tableArn
    );
    this.outputSSM(
      config.app.name(`${Tables.Users}StreamSSM`),
      config.ssm.tables.users.streamArn,
      users.tableStreamArn!
    );

    const projects = new Table(this, Tables.Projects, {
      partitionKey: { name: "uuid", type: AttributeType.STRING },
      billingMode: BillingMode.PAY_PER_REQUEST,
      stream: StreamViewType.NEW_AND_OLD_IMAGES,
    });

    this.outputSSM(
      config.app.name(`${Tables.Projects}SSM`),
      config.ssm.tables.projects.tableArn,
      projects.tableArn
    );
    this.outputSSM(
      config.app.name(`${Tables.Projects}StreamSSM`),
      config.ssm.tables.projects.streamArn,
      projects.tableStreamArn!
    );

    const projectDeployments = new Table(this, Tables.ProjectDeployments, {
      partitionKey: { name: "uuid", type: AttributeType.STRING },
      billingMode: BillingMode.PAY_PER_REQUEST,
      stream: StreamViewType.NEW_AND_OLD_IMAGES,
    });

    this.outputSSM(
      config.app.name(`${Tables.ProjectDeployments}SSM`),
      config.ssm.tables.projectDeployments.tableArn,
      projectDeployments.tableArn,
    );
    this.outputSSM(
      config.app.name(`${Tables.ProjectDeployments}StreamSSM`),
      config.ssm.tables.projectDeployments.streamArn,
      projectDeployments.tableStreamArn!,
    );
  }

  public static getInstance(scope: Construct, table: Tables): ITable {
    const tableArn = StringParameter.fromStringParameterName(
      scope,
      `${table}TableArn`,
      config.ssm.tables[`${table[0].toLowerCase()}${table.substring(1)}` as keyof typeof config.ssm.tables].tableArn
    ).stringValue;

    return Table.fromTableArn(scope, `${table}Table`, tableArn);
  }

  public static getStreamingInstance(scope: Construct, table: Tables): ITable {
    const tableArn = StringParameter.fromStringParameterName(
      scope,
      `${table}TableArn`,
      config.ssm.tables[`${table[0].toLowerCase()}${table.substring(1)}` as keyof typeof config.ssm.tables].tableArn
    ).stringValue;
    const tableStreamArn = StringParameter.fromStringParameterName(
      scope,
      `${table}StreamArn`,
      config.ssm.tables[`${table[0].toLowerCase()}${table.substring(1)}` as keyof typeof config.ssm.tables]
        .streamArn
    ).stringValue;

    return Table.fromTableAttributes(scope, `${table}Table`, {
      tableArn,
      tableStreamArn,
    });
  }

  public static grantReadIndex(
    table: ITable,
    fn: Function,
    index: string = "*"
  ): void {
    fn.addToRolePolicy(
      new PolicyStatement({
        actions: ["dynamodb:Query", "dynamodb:Scan", "dynamodb:GetItem"],
        effect: Effect.ALLOW,
        resources: [`${table.tableArn}/index/${index}`],
      })
    );
  }

  public static grantWriteIndex(
    table: ITable,
    fn: Function,
    index: string = "*"
  ): void {
    fn.addToRolePolicy(
      new PolicyStatement({
        actions: [
          "dynamodb:PutItem",
          "dynamodb:UpdateItem",
          "dynamodb:DeleteItem",
        ],
        effect: Effect.ALLOW,
        resources: [`${table.tableArn}/index/${index}`],
      })
    );
  }

  public static grantReadWriteIndex(
    table: ITable,
    fn: Function,
    index: string = "*"
  ): void {
    TablesStack.grantReadIndex(table, fn, index);
    TablesStack.grantWriteIndex(table, fn, index);
  }
}
