if (!process.env.APP_ENV || process.env.APP_ENV == "development") {
  require("dotenv").config();
}

const DEFAULT = "not-defined";
const APP_PREFIX = "App";
const _name = (name: string) => `${APP_PREFIX}${name}`;

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
