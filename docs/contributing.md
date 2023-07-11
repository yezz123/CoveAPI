# Contributing

!!! note
     When creating a Pull Request for something that doesn't yet have an issue, please describe the problem your pull request is attempting to solve

## Local Development

You'll need rust stable [installed](https://rustup.rs/), or rust nightly if you want to generate accurate coverage.

With rust installed, compiling CoveAPI should be possible with roughly the following:

<div class="termy">

```console
$ git clone https://github.com/yezz123/CoveAPI.git


$ cd CoveAPI
```

</div>

### Docker Development

In case you don't wanna interact with Local Environment you can use the docker container to test and contribute following this:

* Build Image:

<div class="termy">

```console
$ docker build -t coveapi .
```

</div>

### Format

For Providing a good and consistent experience, we recommend using
[pre-commit](https://pre-commit.com/) - a tool that runs a set of checks before
you commit your code.

#### Git Hooks

First you need to install the [pre-commit](https://pre-commit.com/) tool, which
is installed before with the Dev Dependencies.

Now, install the pre-commit hooks in your `.git/hooks/` directory:

<div class="termy">

```console
$ pre-commit install
```

</div>

This one will provide a linting check before you commit your code.

#### Including

The `.pre-commit-config.yaml` contains the following configuration with the
linting packages.

- `pre-commit-hooks` - Some out-of-the-box hooks for pre-commit.
- `Locale hook` - Using cargo `fmt` to format the code & `clippy` to lint the code.

## Documentation

First, make sure you set up your environment as described above, that will
install all the requirements.

The documentation uses
<a href="https://www.mkdocs.org/" class="external-link" target="_blank">MkDocs</a>.

All the documentation is in Markdown format in the directory `./docs`.

### Including

To Build AuthX Documentation we need the following packages, which are:

- `mkdocs` - The tool that builds the documentation.
- `mkdocs-material` - The theme that AuthX uses.
- `mkdocs-markdownextradata-plugin` - The plugin that allows to add extra data
  to the documentation.

### Development

To Help enhance the documentation, you can use the pre-configured development environment:

<div class="termy">

```console
$ pip install -r requirements.txt
```

</div>

after Installing the requirements, you can run the development server:

<div class="termy">

```console
$ mkdocs serve
```

</div>

## Testing

### Test the crate

As we know, the tests are very important to make sure that the code is working If you want to generate the test report:

<div class="termy">

```console
$ make test
```

</div>
