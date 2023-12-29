.DEFAULT_GOAL:=remote
SHELL:=/bin/bash

help:  ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\033[36m***Commands***\033[0m \n \nUsage:\n  make \033[36m<target>\033[0m\n\nTargets:\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-10s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)


clippy:
	cargo clippy


format:
	cargo fmt

kill: ## kill
	docker-compose down

local: ## start all locally
	docker-compose up --build

dev-db: ## build & start only database.
	 docker-compose -f docker-compose-db-only.yaml up
