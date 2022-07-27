#!/usr/bin/env node

import "source-map-support/register";

import { APIDeploymentStack } from "../lib/api/api-deployment-stack";
import { APIStack } from "../lib/api/api-stack";
import { APIUsersStack } from "../lib/api/api-users-stack";
import { App } from "aws-cdk-lib";
import { DeployStack } from "../lib/deploy-stack";
import { TablesStack } from "../lib/tables-stack";
import config from "../config";

const env = {
  region: config.aws.region,
  account: config.aws.account,
};

const app = new App();

const deployStack = new DeployStack(app, config.app.name("DeployStack"), {
  env,
});

const tablesStack = new TablesStack(app, config.app.name("TablesStack"), {
  env,
});

// API
const apiStack = new APIStack(app, config.app.name("APIStack"), {
  env,
});
const apiUsersStack = new APIUsersStack(app, config.app.name("APIUsersStack"), {
  env,
});

const apiDeploymentStack = new APIDeploymentStack(
  app,
  config.app.name("APIDeploymentStack"),
  { env }
);
