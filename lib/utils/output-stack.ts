import { CfnOutput, Stack, StackProps } from "aws-cdk-lib";
import { ParameterType, StringParameter } from "aws-cdk-lib/aws-ssm";

import { Construct } from "constructs";

export abstract class OutputStack extends Stack {
  constructor(scope: Construct, id: string, props: StackProps) {
    super(scope, id, props);
  }

  public output(
    name: string,
    key: string,
    value: string,
    description?: string
  ): void {
    new CfnOutput(this, name, {
      exportName: key,
      value,
      description,
    });
  }

  public outputSSM(name: string, key: string, value: string): void {
    new StringParameter(this, name, {
      parameterName: key,
      stringValue: value,
      type: ParameterType.STRING,
    });
  }
}
