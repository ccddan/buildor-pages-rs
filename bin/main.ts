#!/usr/bin/env node

import "source-map-support/register";

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
const apiUsersStack = new APIUsersStack(app, config.app.name("APIUsersStack"), {
  env,
});
