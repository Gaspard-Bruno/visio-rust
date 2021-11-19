SHELL := /bin/bash

ts := $(shell date -u +"%Y-%m-%dT%H:%M:%SZ")

.PHONY: help
help: ## This help message
	@echo "install maturin package first, pip install maturin"
	@echo -e "$$(grep -hE '^\S+:.*##' $(MAKEFILE_LIST) | sed -e 's/:.*##\s*/:/' -e 's/^\(.\+\):\(.*\)/\\x1b[36m\1\\x1b[m:\2/' | column -c2 -t -s :)"

.PHONY: build
build: nightly dev-packages ## Builds Rust code and visio-exif Python modules
	maturin build

.PHONY: build-release
build-release: nightly dev-packages ## Build visio-exif module in release mode
	maturin build --release

.PHONY: nightly
nightly: ## Set rust compiler to nightly version
	rustup override set nightly

.PHONY: install
install: nightly dev-packages ## Install visio-exif module into current virtualenv
	maturin develop --release

# .PHONY: publish
# publish: ## Publish crate on Pypi
# 	maturin publish

.PHONY: clean
clean: ## Clean up build artifacts
	cargo clean

.PHONY: cargo-test
cargo-test: ## Run cargo tests only
	cargo test

