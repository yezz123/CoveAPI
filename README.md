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


## Usage

## Getting Started

### Local Development

You'll need rust stable [installed](https://rustup.rs/), or rust nightly if you want to generate accurate coverage.

With rust installed, compiling CoveAPI should be possible with roughly the following:

```bash
$ git clone https://github.com/yezz123/CoveAPI.git
$ cd CoveAPI
```

If you want to contribute to CoveAPI, you'll want to use some other make commands:

* `make build-prod` to perform an optimized build for benchmarking
* `make test` to run the tests
* `make lint` to run the linter
* `make format` to format python and rust code

### Docker Development

In case you don't wanna interact with Local Environment you can use the docker container to test and contribute following this:

* Build Image:

```bash
docker build -t coveapi .
```
