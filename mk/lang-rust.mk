.PHONY: lint
lint: ## run clippy linter
	@cargo clippy --all-targets --all-features -- -D warnings

.PHONY: format
format: ## check code formatting
	@cargo fmt --all -- --check

.PHONY: format-fix
format-fix: ## fix code formatting
	@cargo fmt --all

.PHONY: build
build: ## build all targets
	@cargo build --all-targets

.PHONY: test
test: ## run all tests
	@cargo test --all

.PHONY: clean
clean: ## clean build artifacts
	@cargo clean

.PHONY: ci
ci: lint format build test ## run all CI checks

