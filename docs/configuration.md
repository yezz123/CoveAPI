# Configuration

!!! Attention
    CoveAPI does not support OpenAPI 3.x yet, but it will be supported in the future.

## Overview

The following is an overview of all options available for configuring CoveAPI. Each table entry corresponds to a property that can be set when creating an action.

Option                           | Description | Values | Examples
---------------------------------|-------------|--------|---------
account-for-security-forbidden   | Take security annotations into account and require 403 cases to be handled (default `false`) | boolean | `true`
account-for-security-unauthorized | Take security annotations into account and require 401 cases to be handled (default `false`) | boolean | `true`
debug                            | Enables Debug mode (default `false`) | boolean | `true`
instance-url                     | Base of service, excluding base path from OpenAPI | URL | `http://localhost:8080`
only-account-for-pr              | Indicates if only changes within a PR should be taken into account, doesn't take effect outside a PR (default: `false`) | boolean | `true`
openapi-source                   | Location of OpenAPI/Swagger spec | Path or URL | `docs/swagger.yaml`
port                             | Port for CoveAPI to listen on (default `13750`) | unsigned 16-bit integer | `13750`
services                         | Configuration for multiple services, conflicts with port, openapi-source, instance-url | `instance-url; openapi-source; port;\n` | see [here](#multiple-services)
stage                            | Specifies which stage to use | `preparation`, `evaluation` | `preparation`
test-coverage                    | Coverage to enforce in the evaluation stage (default `70%`) | Percentage or float | `0.75`, `75%`
groupings                        | Allows for certain configurations to be grouped together or ignored | `path; method; status_code; ignored;\n` | see [here](#groupings)

## Detailed Information

### Multiple Services
Getting test coverage on a system with multiple services is also possible with CoveAPI. Instead of providing a single `instance-url`, `openapi-source`, and `port`, you can provide a mapping via the `services` option.

The mapping should be provided in the following format:

```yaml
instance-url; openapi-source; port;
instance-url; openapi-source; port;
// etc ...
```

Here, the ports have to be unique and cannot be used twice. The valid fields for `instance-url`, `openapi-source`, and `port` are the same as their respective single options.

An example for a port mapping, looks as follows:

```yaml
    services: |
        http://localhost:8080; docs/swagger1.yaml; 13751;
        http://localhost:8443; docs/swagger2.yaml; 13752;
```

### Networking

Your integration tests can connect to CoveAPI in two different ways:

1. `http://localhost:13750`
2. `http://coveapi:13750`

The first configuration simply replaces the "localhost" text with the IP of the Docker container.

The second configuration creates a Docker network and adds all running Docker containers to it. When running integration tests from within a Docker container, this option could be advantageous.

### Security Headers

CoveAPI can pick up on security annotations in an OpenAPI spec. By default, it ignores these annotations. However, with the options `account-for-security-forbidden` and `account-for-security-unauthorized`, CoveAPI automatically requires you to check `401` and `403` errors, respectively.

### Groupings

Sometimes endpoints reuse the same logic and shouldn't need to be tested twice. Other times, some configurations simply can't be tested and need to be ignored from a test coverage perspective. The groupings feature allows you to define groups. A group only requires a single test to count as tested for all endpoints within the group. If the `ignored` flag is set, the endpoints are assumed to be tested and are excluded from the coverage calculation.

An example grouping configuration could look as follows:

```yaml
groupings: |
    /foo/bar; GET; 200; true;
    /foo/{bar}/moo; GET, POST; 200, 418; false;
```

Feel free to explore and utilize CoveAPI for efficient and comprehensive test coverage of your API.
