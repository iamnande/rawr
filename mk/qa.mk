BASELINE ?= false

benchmark = cargo bench \
	-p $(PACKAGE) \
	--bench $(NAME) \
	-- --verbose \
	$(if $(BASELINE),--save-baseline main,--baseline main)

.PHONY: benchmark-component
benchmark-component:
	@$(call log,"running $(PACKAGE)/$(NAME) benchmarks")
	@$(benchmark)

benchmark-acm: NAME=acm
benchmark-acm: PACKAGE=rawr-acm
benchmark-acm: benchmark-component ## quality: benchmark ACM

benchmark-resource-name: NAME=parse
benchmark-resource-name: PACKAGE=rawr-resource-name
benchmark-resource-name: benchmark-component ## quality: benchmark ResourceName

CPU_PROFILE_BUILD_CMD ?= cargo build --release
CPU_PROFILE_CMD ?= samply record --save-only
CPU_PROFILE_LOAD_CMD ?= samply load
CPU_PROFILE_PATH ?= target/profiles

.PHONY: cpu-profile-component
cpu-profile-component:
	@$(call log,"building $(PACKAGE)/$(NAME) profiling benchmark")
	@$(CPU_PROFILE_BUILD_CMD) \
		-p $(PACKAGE) \
		--example profile_$(NAME)
	@$(call log,"profiling $(PACKAGE)/$(NAME)")
	@mkdir -p $(CPU_PROFILE_PATH)
	@$(CPU_PROFILE_CMD) \
		-o $(CPU_PROFILE_PATH)/$(PACKAGE)_$(NAME).json \
		cargo run \
		--release \
		--example profile_$(NAME) \
		-p $(PACKAGE) -- baseline
	@$(CPU_PROFILE_LOAD_CMD) $(CPU_PROFILE_PATH)/$(PACKAGE)_$(NAME).json

.PHONY: cpu-profile-acm
cpu-profile-acm: NAME=acm
cpu-profile-acm: PACKAGE=rawr-acm
cpu-profile-acm: cpu-profile-component ## quality: CPU profile ACM

.PHONY: cpu-profile-resource-name
cpu-profile-resource-name: NAME=resource_name
cpu-profile-resource-name: PACKAGE=rawr-resource-name
cpu-profile-resource-name: cpu-profile-component ## quality: CPU profile ResourceName