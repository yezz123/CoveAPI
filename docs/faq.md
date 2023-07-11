# Frequently Asked Questions

## Why does CoveAPI require an OpenAPI specification?

CoveAPI requires an OpenAPI specification for endpoint discovery. The OpenAPI spec provides a standardized way to describe the structure and behavior of your API. By analyzing the OpenAPI spec, CoveAPI can accurately determine which endpoints exist in your application and generate test coverage reports accordingly. Without an OpenAPI spec, CoveAPI would not have the necessary information to perform comprehensive test coverage analysis.

## Is CoveAPI compatible with my language/web framework?

CoveAPI is compatible with a wide range of languages and web frameworks that can be used to develop RESTful applications. As long as your language/framework supports the creation of REST endpoints, CoveAPI can be used for test coverage analysis.

The key requirement is that you provide an OpenAPI specification for your API. CoveAPI leverages the information in the OpenAPI spec to understand your API's structure and endpoints.

However, it's worth noting that non-RESTful applications that use technologies such as GraphQL are not compatible with CoveAPI, as it specifically focuses on RESTful APIs.

## Is CoveAPI compatible with my testing framework?

CoveAPI is compatible with various API testing frameworks that can be directed towards its reverse proxy. You can use popular testing frameworks such as Postman/Newman, Insomnia, or JMeter to perform API testing and integrate them with CoveAPI.

By configuring your testing framework to interact with CoveAPI's reverse proxy, you can capture API requests and responses, which CoveAPI uses to track test coverage.

## Why enforce coverage for integration tests?

Enforcing coverage for integration tests is essential for ensuring the quality and reliability of your API. Integration tests aim to validate the interactions between different components and services in your application. By achieving high test coverage in integration testing, you can identify potential issues, bugs, and inconsistencies before deploying to production.

While there are various tools available to measure test coverage for unit tests, there is a lack of generic tools for integration tests. CoveAPI fills this gap by providing a solution specifically designed for measuring test coverage in integration tests. It allows you to set minimum coverage thresholds and generate detailed reports, enabling you to assess the effectiveness of your integration tests and identify any gaps in test coverage.

By enforcing coverage in integration tests, you can increase confidence in your API's functionality, reliability, and performance, leading to a more robust and stable application.

## How to Support Project?

You can financially support the author (me) through
<a href="https://github.com/sponsors/yezz123" class="external-link" target="_blank">Github Sponsors</a>.

And you can also become a Silver or Gold sponsor for CoveAPI, just contact me by email.

- Email: <a href="mailto:hello@yezz.me" class="external-link" target="_blank">hello@yezz.me</a>
