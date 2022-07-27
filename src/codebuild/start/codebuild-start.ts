import { CodeBuild } from "aws-sdk";
const codebuild = new CodeBuild();

export const handler = async (event: any, context: any) => {
  try {
    console.log("Trigger codebuild project to deploy SPA");
    const result = await codebuild
      .startBuild({
        projectName: "App-Dynamically-Deploy-SPAs",
        environmentVariablesOverride: [
          {
            name: "PROJECT_NAME",
            value: "buildspace-solana-pay",
          },
          {
            name: "REPO_URL",
            value: "https://github.com/ccddan/buildspace-solana-pay.git",
          },
        ],
        buildspecOverride: JSON.stringify(
          {
            version: "0.2",
            env: {
              variables: {
                MY_ENV_VAR: "value",
              },
            },
            phases: {
              install: {
                commands: [
                  "echo Download project",
                  "node -v",
                  "git clone $REPO_URL $PROJECT_NAME",
                  "ls -las",
                ],
              },
              pre_build: {
                commands: [
                  "echo Install project dependencies",
                  "cd $PROJECT_NAME",
                  "ls -las",
                  "npm install",
                ],
              },
              build: {
                commands: [
                  "echo Build project",
                  "npm run release",
                  "ls -las",
                  "echo Move build output to artifacts location",
                  "mv out ../dist/$PROJECT_NAME",
                  "ls -las ../dist",
                ],
              },
              post_build: {
                commands: ["echo Build has completed and artifacts were moved"],
              },
            },
            artifacts: {
              "discard-paths": "no",
              files: ["dist"],
            },
          },
          null,
          4
        ),
      })
      .promise();

    console.log("Codebuild start result:", result);
  } catch (error) {
    console.error("ERROR:", error);
  }
};
