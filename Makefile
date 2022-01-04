SHELL := /bin/bash

ts := $(shell date -u +"%Y-%m-%dT%H:%M:%SZ")

.PHONY: help
help: ## This help message
	@echo -e "$$(grep -hE '^\S+:.*##' $(MAKEFILE_LIST) | sed -e 's/:.*##\s*/:/' -e 's/^\(.\+\):\(.*\)/\\x1b[36m\1\\x1b[m:\2/' | column -c2 -t -s :)"

.PHONY: install
install: ## Install visio-rust module into current virtualenv
	pip install -r requirements.txt
	maturin develop --release

.PHONY: clean
clean: ## Clean up build artifacts
	cargo clean