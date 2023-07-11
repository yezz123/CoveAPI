![Cover](docs/img/cover.png)

CoveAPI is an advanced test coverage analysis tool based on the OpenAPI standard. It offers a comprehensive solution for teams to effectively measure and improve their integration test coverage within CI/CD pipelines.

With CoveAPI, teams can easily establish and enforce specific coverage thresholds, ensuring that critical parts of their application are thoroughly tested. By integrating CoveAPI into their existing CI/CD workflows, teams can automatically track and monitor test coverage metrics, making it easier to identify areas that require additional testing.

**NOTE: CoveAPI is still under heavy development and not yet stable or even feature complete**

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
