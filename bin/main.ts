#!/usr/bin/env node

import { App } from "aws-cdk-lib";
import "source-map-support/register";
import config from "../config";
import { APIDeploymentStack } from "../lib/api/api-deployment-stack";
import { APIProjectsStack } from "../lib/api/api-projects-stack";
import { APIProjectDeploymentsStack } from "../lib/api/api-project-deployments-stack";
import { APIStack } from "../lib/api/api-stack";
import { APIUsersStack } from "../lib/api/api-users-stack";
import { DeployStack } from "../lib/deploy-stack";
import { TablesStack } from "../lib/tables-stack";


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
const apiProjectsStack = new APIProjectsStack(
  app,
  config.app.name("APIProjectsStack"),
  { env }
);
const apiProjectDeploymentsStack = new APIProjectDeploymentsStack(
  app,
  config.app.name("APIProjectDeploymentsStack"),
  { env },
);

const apiDeploymentStack = new APIDeploymentStack(
  app,
  config.app.name("APIDeploymentStack"),
  { env }
);
