PODMAN_CMD := podman
COMPOSE_FILE := local/environment.yaml
COMPOSE_FILE_ARG := --file $(COMPOSE_FILE)
COMPOSE_CMD := $(PODMAN_CMD) compose $(COMPOSE_FILE_ARG)

.PHONY: create
env-create: env-destroy ## env: (re)create environment
	@$(COMPOSE_CMD) up --detach --no-log-prefix --pull missing

.PHONY: status
env-status: ## env: check environment status
	@$(COMPOSE_CMD) ps

.PHONY: destroy
env-destroy: ## env: destroy environment
	@$(COMPOSE_CMD) down --remove-orphans --volumes
	@sleep 3

.PHONY: db-shell
db-shell: ## db: get a psql shell
	@PGPASSWORD=$(DB_PASSWORD) $(COMPOSE_CMD) exec postgres psql -U $(DB_USER) -d $(DB_NAME)

.PHONY: db-migrate
db-migrate: ## db: run database migrations
	@cargo run --bin migrate --package rawr-db
