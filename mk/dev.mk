.PHONY: lint
lint: ## run clippy linter
	@$(call log,"linting the workspace")
	@cargo clippy --all-targets --all-features -- -D warnings

.PHONY: format
format: ## fix code formatting
	@$(call log,"formatting the workspace")
	@cargo fmt --verbose --all

.PHONY: format-check
format-check: ## check code formatting
	@$(call log,"checking code formatting")
	@cargo fmt --verbose --all -- --check

.PHONY: build
build: ## build all targets
	@$(call log,"building all targets")
	@cargo build --all-targets

.PHONY: test
test: ## run all tests
	@$(call log,"running all tests")
	@cargo test --all -- --no-capture

.PHONY: bench 
bench: ## run benchmark tests
	@$(call log,"running benchmark tests")
	@cargo bench --verbose

.PHONY: clean
clean: ## clean build artifacts
	@$(call log,"cleaning build artifacts")
	@cargo clean

.PHONY: ci
ci: lint format-check build test bench ## run all CI checks

.PHONY: profile-acm
profile-acm: ## run the ACM profiling example
	@$(call log,"building ACM profiling benchmark")
	@cargo build --release --example profile_acm -p rawr-acm
	@$(call log,"profiling ACM - baseline category")
	@mkdir -p target/profiles
	@samply record --save-only -o target/profiles/acm.json cargo run --release --example profile_acm -p rawr-acm -- baseline
	@samply load target/profiles/acm.json