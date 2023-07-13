# Postman Integration

## Prerequisites

Before setting up your CI pipeline with CoveAPI, make sure you have the following:

- Running Application: Ensure that your API is up and running and accessible.
- Accessible OpenAPI Specification: Have an OpenAPI specification file for your API that can be accessed either via a URL or stored locally.
- Postman Collection: Prepare a Postman collection that contains your integration tests.

## Create a New Environment

Create a new environment in Postman for your test suite. To do this, click on "New" and select "Environment". In the environment settings, update the port value of your base URL to match the port used by CoveAPI.

By default, CoveAPI uses port 13750, but you can change it using the `port` parameter in the CoveAPI configuration (see [configuration options](./configuration.md) for more details). If you're not using environments, manually update the port in all your tests.

## Configure CoveAPI

Configure CoveAPI in the Preparation Stage of your CI pipeline. This step sets up CoveAPI for later use. Place this stage **after** starting your service and **before** running your integration tests.

Modify the `openapi-source` parameter to point to the location of your OpenAPI specification file. This can be a local file path or a URL. Update the `instance-url` parameter to match the base URL of your service (excluding the base path specified in your OpenAPI spec). Optionally, set a desired `test-coverage` percentage for your endpoints.

```yaml
- name: Initialize CoveAPI
  uses: yezz123/coveapi@2.1.0
  with:
    stage: "preparation"
    openapi-source: "docs/swagger.json"
    instance-url: "http://localhost:8080"
    test-coverage: "75%"
```

## Run [Newman](https://github.com/postmanlabs/newman)

Add a step to run [Newman](https://github.com/postmanlabs/newman), the Postman CLI tool, after configuring CoveAPI. This step executes your integration tests.

```yaml
- uses: postmanlabs/newman@v2
  name: Run Integration Tests
  with:
    collection: tests/coveapi-example.postman_collection.json
    environment: tests/coveapi-example-ci.postman_environment.json
```

Make sure to update the `collection` and `environment` parameters to match your Postman collection and environment file names. You can omit the `environment` parameter if you're not using one.

## Evaluate Test Coverage

Add the Evaluation Stage to your pipeline. This stage evaluates your tests after running Newman and fails the pipeline if the configured test coverage threshold is not met. No additional configuration is required for this stage.

```yaml
- uses: yezz123/coveapi@2.1.0
  name: Evaluate CoveAPI Test Coverage
  with:
    stage: "evaluation"
```

## Result

a sample pipeline could look like the following example:

```yaml
name: Integration Tests

on:
  push:
    branches: [ main ]
  workflow_dispatch:

jobs:
  run-integration-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install Newman
        run: npm install -g newman
      - name: Initialize CoveAPI
        uses: yezz123/coveapi@2.1.0
        with:
          stage: "preparation"
          openapi-source: "docs/swagger.json"
          instance-url: "http://localhost:8080"
          test-coverage: "90%"
      - uses: postmanlabs/newman@v2
        name: Run Integration Tests
        with:
          collection: tests/coveapi-example.postman_collection.json
          environment: tests/coveapi-example-ci.postman_environment.json
      - uses: yezz123/coveapi@2.1.0
        name: Evaluate CoveAPI Test Coverage
        with:
          stage: "evaluation"
```

Feel free to adapt and customize this pipeline according to your specific project requirements, and enjoy the benefits of comprehensive API test coverage with CoveAPI.
