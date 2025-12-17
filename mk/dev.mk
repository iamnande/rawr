.PHONY: check
check: ## dev: do we compile?
	@$(call log,"checking compilation")
	@cargo check --all --tests --benches

.PHONY: clean
clean: ## dev: purge build artifacts
	@$(call log,"cleaning build artifacts")
	@cargo clean

.PHONY: lint
lint: ## dev: are we idiomatic?
	@$(call log,"linting the workspace")
	@cargo clippy --all-targets --all-features -- -D warnings

.PHONY: format
format: ## dev: format the code
	@$(call log,"formatting the workspace")
	@cargo fmt --verbose --all

.PHONY: format-check
format-check: ## dev: are we pretty?
	@$(call log,"checking code formatting")
	@cargo fmt --verbose --all -- --check

.PHONY: test
test: ## dev: do we profess to work?
	@$(call log,"running all tests")
	@cargo nextest run --profile ci --workspace

.PHONY: test-docs
test-docs: ## dev: do our docs work?
	@$(call log,"running all documentation tests")
	@cargo test --doc --workspace