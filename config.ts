if (!process.env.APP_ENV || process.env.APP_ENV == "development") {
  require("dotenv").config();
}

const DEFAULT = "not-defined";
const APP_PREFIX = "App";
const _name = (name: string) => `${APP_PREFIX}${name}`;

// Default Logging Levels
const LOGS_LEVEL_LAMBDAS_DEFAULT = "info";

const config = {
  app: {
    name: _name,
    prefix: APP_PREFIX,
  },
  aws: {
    region: DEFAULT,
    account: DEFAULT,
  },
  api: {
    version: "v1",
    logging: "INFO", // OFF, ERROR, INFO
    resources: {
      // logging: any value for RUST_LOG env var (error,warning,info,debug)
      projectDeployments: {
        deployment: {
          get: {
            logging: process.env.LOGS_LEVEL_API_PROJECT_DEPLOYMENTS_DEPLOYMENT_GET ? process.env.LOGS_LEVEL_API_PROJECT_DEPLOYMENTS_DEPLOYMENT_GET : LOGS_LEVEL_LAMBDAS_DEFAULT,
          }
        },
        post: {
          logging: process.env.LOGS_LEVEL_API_PROJECT_DEPLOYMENTS_POST ? process.env.LOGS_LEVEL_API_PROJECT_DEPLOYMENTS_POST : LOGS_LEVEL_LAMBDAS_DEFAULT,
        },
      },
      projects: {
        list: {
          logging: process.env.LOGS_LEVEL_API_PROJECTS_LIST ? process.env.LOGS_LEVEL_API_PROJECTS_LIST : LOGS_LEVEL_LAMBDAS_DEFAULT,
        },
        post: {
          logging: process.env.LOGS_LEVEL_API_PROJECTS_POST ? process.env.LOGS_LEVEL_API_PROJECTS_POST : LOGS_LEVEL_LAMBDAS_DEFAULT,
        },
      },
      root: {
        any: {
          logging: process.env.LOGS_LEVEL_API_ROOT_ANY ? process.env.LOGS_LEVEL_API_ROOT_ANY : LOGS_LEVEL_LAMBDAS_DEFAULT,
        },
      },
      users: {
        list: {
          logging: process.env.LOGS_LEVEL_API_USERS_LIST ? process.env.LOGS_LEVEL_API_USERS_LIST : LOGS_LEVEL_LAMBDAS_DEFAULT,
        },
        post: {
          logging: process.env.LOGS_LEVEL_API_USERS_POST ? process.env.LOGS_LEVEL_API_USERS_POST : LOGS_LEVEL_LAMBDAS_DEFAULT,
        },
      },
    },
  },
  codebuild: {
    events: {
      processing: {
        logging: process.env.LOGS_LEVEL_CODEBUILD_EVENTS_PROCESSING ? process.env.LOGS_LEVEL_CODEBUILD_EVENTS_PROCESSING : LOGS_LEVEL_LAMBDAS_DEFAULT,
      },
    },
  },
  ssm: {
    api: {
      id: `/${APP_PREFIX}/api/id`,
      resources: {
        root: {
          id: `/${APP_PREFIX}/api/resources/root/id`,
        },
      },
    },
    tables: {
      users: {
        tableArn: `/${APP_PREFIX}/tables/users/tableArn`,
        streamArn: `/${APP_PREFIX}/tables/users/streamArn`,
      },
      projects: {
        tableArn: `/${APP_PREFIX}/tables/projects/tableArn`,
        streamArn: `/${APP_PREFIX}/tables/projects/streamArn`,
      },
      projectDeployments: {
        tableArn: `/${APP_PREFIX}/tables/projectDeployments/tableArn`,
        streamArn: `/${APP_PREFIX}/tables/projectDeployments/streamArn`,
      },
    },
    codebuild: {
      project: {
        name: `/${APP_PREFIX}/codebuild/project/name`,
        arn: `/${APP_PREFIX}/codebuild/project/arn`,
      },
    },
  },
};

if (!process.env.AWS_REGION) {
  throw new Error("AWS_REGION env var is required");
}
if (!process.env.AWS_ACCOUNT) {
  throw new Error("AWS_ACCOUNT env var is required");
}

config.aws.region = process.env.AWS_REGION;
config.aws.account = process.env.AWS_ACCOUNT;

export default config;
