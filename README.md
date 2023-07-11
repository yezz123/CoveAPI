<p align="center">
<a href="https://github.com/yezz123/CoveAPI" target="_blank">
    <img src="https://raw.githubusercontent.com/yezz123/CoveAPI/main/docs/img/cover.png">
</a>
<p align="center">
    <em>Ready-to-use OpenAPI test coverage analysis tool that helps teams improve integration</em>
</p>
<p align="center">
<a href="https://github.com/yezz123/CoveAPI/actions/workflows/ci.yml" target="_blank">
    <img src="https://github.com/yezz123/CoveAPI/actions/workflows/ci.yml/badge.svg" alt="lint">
</a>
<a href="https://codecov.io/gh/yezz123/CoveAPI" >
    <img src="https://codecov.io/gh/yezz123/CoveAPI/branch/main/graph/badge.svg"/>
</a>
<a href="https://github.com/yezz123/CoveAPI/blob/main/LICENSE" >
    <img src="https://img.shields.io/github/license/yezz123/CoveAPI.svg"/>
</a>
<a href="https://github.com/yezz123/CoveAPI" >
    <img src="https://img.shields.io/github/repo-size/yezz123/coveapi"/>
</a>
</p>
</p>

CoveAPI is an advanced test coverage analysis tool based on the OpenAPI standard. It offers a comprehensive solution for teams to effectively measure and improve their integration test coverage within CI/CD pipelines.

With CoveAPI, teams can easily establish and enforce specific coverage thresholds, ensuring that critical parts of their application are thoroughly tested. By integrating CoveAPI into their existing CI/CD workflows, teams can automatically track and monitor test coverage metrics, making it easier to identify areas that require additional testing.

## Help

See [documentation](https://coveapi.yezz.me/) for more details.

## Usage

### Integrate CoveAPI into your CI/CD pipeline

Direct your integration tests towards the CoveAPI reverse proxy to enable analysis and interpretation of requests that occur during the testing process. This ensures that CoveAPI effectively handles requests during integration testing.

Configure your tests to target either `http://localhost:13750` (without Docker) or `http://coveapi:13750` (with Docker). If you are using Docker, CoveAPI will automatically set up the networking for your container to establish a connection with it.

### Setup Preparation Stage

To ensure CoveAPI is properly configured for later use, follow these steps. Place this preparation stage after starting your service and before running your integration tests.

Please remember to replace the location of your OpenAPI spec and the instance URL. You can provide the OpenAPI spec as either a local file path or a URL.

```yaml
- name: Initialize CoveAPI
  uses: yezz123/CoveAPI@v2.0.0
  with:
    stage: "preparation"
    openapi-source: "docs/swagger.json"
    instance-url: "http://localhost:8080"
    test-coverage: "75%"
```

Make sure to modify the `openapi-source` parameter to point to the location of your OpenAPI or Swagger specification. This can be either a local file path or a URL.

Similarly, adjust the `instance-url` parameter to match the base URL of your service, excluding the base path specified in your OpenAPI spec.

Optionally, you can set a desired `test-coverage` value for your endpoints.

By following these steps, CoveAPI will be properly prepared for integration testing, ensuring accurate analysis and interpretation of requests.

### Setup Evaluation Stage

Place the CoveAPI evaluation stage somewhere after your integration tests have run.

```yaml
  - uses: yezz123/CoveAPI@v2.0.0
    name: Evaluate CoveAPI
    with:
      stage: "evaluation"
```

This stage will fail if test coverage isn't met and can display additional information gathered during the integration tests.

## Contributing

For guidance on setting up a development environment and how to make a contribution to CoveAPI, see [Contributing to CoveAPI](https://coveapi.yezz.me/contributing).

## Reporting a Security Vulnerability

See our [security policy](https://github.com/yezz123/CoveAPI/security/policy).
