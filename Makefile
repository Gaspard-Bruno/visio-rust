.PHONY: help
help: ## Show this help
	@egrep -h '\s##\s' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

.PHONY: build
setup:	## Build cargo
	pip install maturin

.PHONY: build
build-cargo:	## Build cargo
	cargo build --release


.PHONY: run
run:	## Run project 
	cargo run --release
