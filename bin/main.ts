#!/usr/bin/env node

import "source-map-support/register";

import { APIStack } from "../lib/api/api-stack";
import { APIUsersStack } from "../lib/api/api-users-stack";
import { App } from "aws-cdk-lib";
import { TablesStack } from "../lib/tables-stack";
import config from "../config";

const env = {
  region: config.aws.region,
  account: config.aws.account,
};

const app = new App();

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
