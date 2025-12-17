# core
.DEFAULT_GOAL := help
WORKDIR       := $(shell pwd)
SHELL         := /usr/bin/env bash

# vcs info
VCS_COMMIT   := $(shell git rev-parse --short=7 HEAD)
VCS_IS_DIRTY := $(shell test -n "$$(git status --porcelain)" && echo "-alpha")

# colors are pretty
COLOR_CYAN=\033[0;36m
COLOR_GREEN=\033[0;32m
COLOR_MAGENTA=\033[0;35m
COLOR_YELLOW=\033[0;33m
COLOR_NONE=\033[0m

# project information
OWNER_NAME      := iamnande
PROJECT_NAME    := rawr
PROJECT_VERSION ?= 0.1.0$(VCS_IS_DIRTY)
PROJECT_SLUG    := $(OWNER_NAME)-$(PROJECT_NAME)-$(PROJECT_VERSION)

# modules
include mk/log.mk
include mk/dev.mk
include mk/qa.mk

.PHONY: help
help: ## help: display available targets
	@echo -e "${COLOR_GREEN}================================================================================${COLOR_NONE}"
	@echo -e "                    [ ${COLOR_MAGENTA}$(OWNER_NAME)${COLOR_YELLOW}/${COLOR_MAGENTA}$(PROJECT_NAME) ${COLOR_NONE} - ${COLOR_MAGENTA}$(PROJECT_VERSION)${COLOR_NONE} ] "
	@echo -e "${COLOR_GREEN}================================================================================${COLOR_NONE}"
	@grep -h -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "} \
		{printf "${COLOR_CYAN}%-35s${COLOR_NONE} %s %s\n", $$1, "     ", $$2}'

.PHONY: version
version: ## help: display version
	@echo $(PROJECT_VERSION)