# Buildor Pages

## Requirements

- AWS Profile (tested with Admin privileges)
- NodeJS: ^16.16
- Rust
  - cargo: 1.61
  - rustup: 1.24.3

## Usage

### Dependencies

```bash
$ npm install
$ rustup target add x86_64-unknown-linux-gnu
$ cargo install cargo-lambda --version=0.9.0
```

Compile Rust code:

```bash
$ npm run build:lambdas
```

### Infrastructure

1. Prepare env vars:

   ```bash
   $ cp .example.env .env
   # Update values in .env
   $ source .env
   ```

1. (First time action) Bootstrap AWS environment:

   ```bash
   $ npm run cdk -- bootstrap aws://$AWS_ACCOUNT/$AWS_REGION --toolkit-stack-name $(echo $APP_PREFIX)Toolkit --profile <name>
   ```

1. Deploy Tables:

   ```bash
   $ npm run cdk -- deploy $(echo $APP_PREFIX)TablesStack --toolkit-stack-name $(echo $APP_PREFIX)Toolkit --profile <name>
   ```

1. Resources to automatically deploy SPAs:

   ```bash
   $ npm run cdk -- deploy $(echo $APP_PREFIX)DeployStack --toolkit-stack-name $(echo $APP_PREFIX)Toolkit --profile <name>
   ```

1. Deploy API:

   ```bash
   $ npm run cdk -- deploy $(echo $APP_PREFIX)APIStack --require-approval never --toolkit-stack-name $(echo $APP_PREFIX)Toolkit --profile <name>
   $ npm run cdk -- deploy $(echo $APP_PREFIX)APIUsersStack --force --require-approval never --toolkit-stack-name $(echo $APP_PREFIX)Toolkit --profile <name>
   $ npm run cdk -- deploy $(echo $APP_PREFIX)APIProjectsStack --force --require-approval never --toolkit-stack-name $(echo $APP_PREFIX)Toolkit --profile <name>
   $ npm run cdk -- deploy $(echo $APP_PREFIX)APIDeploymentStack --require-approval never --toolkit-stack-name $(echo $APP_PREFIX)Toolkit --profile <name>
   ```

1. Test API Endpoints:

Once the APIDeploymentStack is deployed, you should be able to see the API URL in the command logs. Use that URL in the following commands:

```bash
# List users
$ curl -vvv <API_URL>/users

# Register a new user
$ curl -vvv <API_URL>/users -d '{"fname": "John", "lname": "Doe"}'
```

## Clean Up

```bash
$ npm run cdk -- destroy --force $(echo $APP_PREFIX)APIProjectsStack --toolkit-stack-name $(echo $APP_PREFIX)Toolkit --profile <name>
$ npm run cdk -- destroy --force $(echo $APP_PREFIX)APIUsersStack --toolkit-stack-name $(echo $APP_PREFIX)Toolkit --profile <name>
$ npm run cdk -- destroy --force $(echo $APP_PREFIX)APIStack --toolkit-stack-name $(echo $APP_PREFIX)Toolkit --profile <name>
$ npm run cdk -- destroy --force $(echo $APP_PREFIX)DeployStack --toolkit-stack-name $(echo $APP_PREFIX)Toolkit --profile <name>
$ npm run cdk -- destroy --force $(echo $APP_PREFIX)TablesStack --toolkit-stack-name $(echo $APP_PREFIX)Toolkit --profile <name>
```
