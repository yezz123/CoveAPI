.DEFAULT_GOAL := all

.PHONY: build-prod
build-prod:
	cargo build --release

.PHONY: install-rust-coverage
install-rust-coverage:
	cargo install rustfilt coverage-prepare
	rustup component add llvm-tools-preview

.PHONY: format
format:
	cargo fmt

.PHONY: lint
lint:
	cargo fmt --version
	cargo fmt --all -- --check
	cargo clippy --version
	cargo clippy --tests --

.PHONY: test
test:
	cargo test --all
