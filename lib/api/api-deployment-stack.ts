import { StackProps } from "aws-cdk-lib";
import {
    AccessLogFormat,
    Deployment,
    LogGroupLogDestination,
    MethodLoggingLevel,
    Stage
} from "aws-cdk-lib/aws-apigateway";
import { LogGroup } from "aws-cdk-lib/aws-logs";
import { Construct } from "constructs";
import config from "../../config";
import { OutputStack } from "../utils/output-stack";
import { APIStack } from "./api-stack";


export class APIDeploymentStack extends OutputStack {
  constructor(scope: Construct, id: string, props: StackProps) {
    super(scope, id, props);

    const api = APIStack.getInstance(this);

    const deployment = new Deployment(
      this,
      config.app.name(`ApiDeployment-${config.api.version}`),
      { api }
    );
    deployment.addToLogicalId(new Date().getTime()); // force deployment to update

    const logsGroup = new LogGroup(
      this,
      config.app.name(`ApiLogs-${config.api.version}`),
      {
        logGroupName: config.app
          .name(`-api-${config.api.version}`)
          .toLowerCase(),
      }
    );

    const stage = new Stage(
      this,
      config.app.name(`ApiStage-${config.api.version}`),
      {
        stageName: config.api.version,
        deployment,
        dataTraceEnabled: true,
        tracingEnabled: true,
        loggingLevel: MethodLoggingLevel.INFO,
        accessLogDestination: new LogGroupLogDestination(logsGroup),
        accessLogFormat: AccessLogFormat.jsonWithStandardFields({
          caller: true,
          httpMethod: true,
          ip: true,
          protocol: true,
          requestTime: true,
          resourcePath: true,
          responseLength: true,
          status: true,
          user: true,
        }),
      }
    );

    this.output(
      config.app.name("APIEndpoint"),
      config.app.name("APIEndpoint"),
      stage.urlForPath("/")
    );
  }
}
