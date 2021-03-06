# Deployment

> **IMPORTANT:** following steps are targeting Amazon Linux 2 Runtime

## Requirements

- AWS Profile with enough permissions:
  - create roles
  - attach policies to roles
  - create, update and invoke lambdas

## Build

1. (First time action) Install target architecture:

   ```bash
   # For Arm64 Lambda functions
   $ rustup target add aarch64-unknown-linux-gnu

   # Or, for x86_64 Lambda functions
   $ rustup target add x86_64-unknown-linux-gnu
   ```

2. (First time action) Install Zig:

   ```bash
   $ cargo install cargo-lambda
   ```

   > **IMPORTANT:** when you build the project for the first time you might be asked to install Zig using Python or NodeJS. Select whichever approach works the best for you and confirm its installation.

3. Build lambda:

   ```bash
   $ cargo lambda build --release --target x86_64-unknown-linux-gnu --output-format zip
   ```

## Deploy to AWS

1. (First time action) Create Table:

   ```bash
   $ aws dynamodb create-table \
    --table-name Users \
    --attribute-definitions AttributeName=uuid,AttributeType=S \
    --key-schema AttributeName=uuid,KeyType=HASH \
    --billing-mode PAY_PER_REQUEST \
    --region <aws-region> \
    --profile <aws-profile-name>
   ```

1. (First time action) Create Lambda execution role:

   ```bash
   $ aws iam create-role \
      --role-name lambda-rs-role \
      --assume-role-policy-document '{"Version": "2012-10-17","Statement": [{ "Effect": "Allow", "Principal": {"Service": "lambda.amazonaws.com"}, "Action": "sts:AssumeRole"}]}' \
      --region <aws-region> \
      --profile <aws-profile-name>
   $ export TABLE_ARN=$(aws dynamodb describe-table --table-name Users --query "Table.TableArn" --region <aws-region> --profile <aws-profile-name>) &&
      aws iam create-policy \
      --policy-name users-table-access \
      --policy-document "{\"Version\": \"2012-10-17\", \"Statement\": [{ \"Sid\": \"FullTableAccess\", \"Effect\": \"Allow\", \"Action\": \"dynamodb:*\", \"Resource\": $TABLE_ARN},{\"Sid\": \"CreateLogGroup\",\"Effect\": \"Allow\",\"Action\": \"logs:CreateLogGroup\",\"Resource\": \"*\"}]}" \
      --region <aws-region> \
      --profile <aws-profile-name>
   ```

   Attach required permissions to the role:

   ```bash
   $ aws iam attach-role-policy \
      --role-name lambda-rs-role \
      --policy-arn arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole \
      --region <aws-region> \
      --profile <aws-profile-name>
   $ aws iam attach-role-policy \
      --role-name lambda-rs-role \
      --policy-arn $(aws iam list-policies --query 'Policies[?PolicyName==`users-table-access`].Arn' --output text --region <aws-region> --profile <aws-profile-name>) \
      --region <aws-region> \
      --profile <aws-profile-name>
   ```

1. (First time action) Create Lambda

   ```bash
   $ aws lambda create-function --function-name lambda-rs \
       --handler bootstrap \
       --zip-file fileb://./target/lambda/lambda-rs/bootstrap.zip \
       --runtime provided.al2 \
       --role $(aws iam get-role --role-name lambda-rs-role --profile cc --query 'Role.Arn' | tr -d \") \
       --environment "Variables={RUST_BACKTRACE=1,TABLE_NAME=Users,TABLE_REGION=us-west-2}" \
       --architectures "x86_64" \
       --tracing-config Mode=Active \
       --region <aws-region> \
       --profile <aws-profile-name>
   ```

1. Update Lambda code:

```bash
$ aws lambda update-function-code \
    --function-name  lambda-rs \
    --zip-file fileb://./target/lambda/lambda-rs/bootstrap.zip \
    --region <aws-region> \
    --profile <aws-profile-name>
```

1. (Optional) Test lambda invocation:

   ```bash
   $ aws lambda invoke \
     --cli-binary-format raw-in-base64-out \
     --function-name lambda-rs \
     --payload '{ "fname": "Unknown", "lname": "Guy" }' \
     --region <aws-region> \
     --profile <aws-profile-name> \
     lambda-rs-output.json
   $ cat lambda-rs-output.json
   ```

# Clean Up

- Delete lambda:

  ```bash
  $ aws lambda delete-function \
    --function-name lambda-rs \
    --region <aws-region> \
    --profile <aws-profile-name>
  ```

- Delete Lambda role:

  ```bash
  $ aws lambda delete-function \
    --function-name lambda-rs \
    --region <aws-region> \
    --profile <aws-profile-name>

  $ aws iam detach-role-policy \
    --role-name lambda-rs-role \
    --policy-arn arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole \
    --region <aws-region> \
    --profile <aws-profile-name>

  $ export POLICY_ARN=$(aws iam list-policies --query 'Policies[?PolicyName==`users-table-access`].Arn' --output text --region <aws-region> --profile <aws-profile-name>)
    aws iam detach-role-policy \
    --role-name lambda-rs-role \
    --policy-arn $POLICY_ARN \
    --region <aws-region> \
    --profile <aws-profile-name>
  $ aws iam delete-policy \
    --policy-arn $POLICY_ARN \
    --region <aws-region> \
    --profile <aws-profile-name>

  $ aws iam delete-role \
    --role-name lambda-rs-role \
    --region <aws-region> \
    --profile <aws-profile-name>
  ```
