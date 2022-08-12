import { StackProps } from "aws-cdk-lib";
import {
    AuthorizationType,
    EndpointType,
    IResource,
    IRestApi,
    LambdaIntegration,

    Resource,
    RestApi
} from "aws-cdk-lib/aws-apigateway";
import { ServicePrincipal } from "aws-cdk-lib/aws-iam";
import {
    Architecture,
    AssetCode,
    Function,
    Runtime
} from "aws-cdk-lib/aws-lambda";
import { StringParameter } from "aws-cdk-lib/aws-ssm";
import { Construct } from "constructs";
import config from "../../config";
import { OutputStack } from "../utils/output-stack";


export class APIStack extends OutputStack {
  public static readonly principal = new ServicePrincipal(
    "apigateway.amazonaws.com"
  );

  constructor(scope: Construct, id: string, props: StackProps) {
    super(scope, id, props);

    const api = new RestApi(this, "Api", {
      restApiName: config.app.name("-api").toLowerCase(),
      endpointTypes: [EndpointType.REGIONAL],
      disableExecuteApiEndpoint: false, // TODO: change once domain is configured
      minimumCompressionSize: 0,
      deploy: false,
      defaultCorsPreflightOptions: {
        allowOrigins: ["*"],
        statusCode: 200,
        allowMethods: ["ANY"],
      },
      defaultMethodOptions: {
        authorizer: undefined,
        authorizationType: AuthorizationType.NONE,
      },
    });
    this.outputSSM(config.app.name("Api"), config.ssm.api.id, api.restApiId);

    // API Root Resource
    this.outputSSM(
      config.app.name("apiRoot"),
      config.ssm.api.resources.root.id,
      api.root.resourceId
    );

    const anyIntegration = new LambdaIntegration(
      new Function(this, "any", {
        description: "Default API root resource handler",
        runtime: Runtime.PROVIDED_AL2,
        code: AssetCode.fromAsset("target/lambda/api-root-any/bootstrap.zip"),
        architecture: Architecture.X86_64,
        handler: "bootstrap",
        environment: {
          RUST_BACKTRACE: "1",
        },
      })
    );

    api.root.addMethod("ANY", anyIntegration);
  }

  public static getInstance(scope: Construct): IRestApi {
    const apiId = StringParameter.fromStringParameterName(
      scope,
      "apiIdSSM",
      config.ssm.api.id
    ).stringValue;
    return RestApi.fromRestApiId(scope, "Api", apiId);
  }

  public static getRootResource(
    scope: Construct,
    api: IRestApi | RestApi
  ): IResource {
    const rootId = StringParameter.fromStringParameterName(
      scope,
      "ApiRootResourceId",
      config.ssm.api.resources.root.id
    ).stringValue;
    return Resource.fromResourceAttributes(scope, "ApiRootResource", {
      restApi: api,
      resourceId: rootId,
      path: `/${config.api.version}`,
    });
  }
}
