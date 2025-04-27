all-down: down prod-down

#----------------------------------------------------------------------
# Development
#----------------------------------------------------------------------
build:
	docker compose build

up:
	docker compose up

up-detach:
	docker compose up --detach

logs:
	docker compose logs --follow

down:
	docker compose down

frontend-up:
	docker compose up frontend

frontend-fish:
	docker compose run --rm -it --service-ports --entrypoint fish frontend

backend-up:
	docker compose up backend

backend-fish:
	docker compose run --rm -it --service-ports --entrypoint fish backend

backend-exec:
	docker compose exec backend fish

pgsql-up:
	docker compose up pgsql

pgsql-fish:
	docker compose run --rm -it --service-ports --entrypoint fish pgsql

pgsql-exec:
	docker compose exec pgsql fish

#----------------------------------------------------------------------
# Production
#----------------------------------------------------------------------
prod-build:
	docker compose -f docker-compose.prod.yaml build

prod-up:
	docker compose -f docker-compose.prod.yaml up

prod-down:
	docker compose -f docker-compose.prod.yaml down

prod-up-detach:
	docker compose -f docker-compose.prod.yaml up --detach

prod-frontend-fish:
	docker compose -f docker-compose.prod.yaml run --rm -it --service-ports --entrypoint fish frontend

prod-backend-fish:
	docker compose -f docker-compose.prod.yaml run --rm -it --service-ports --entrypoint fish backend

prod-nginx-exec:
	docker compose -f docker-compose.prod.yaml exec nginx sh


prod-logs:
	docker compose -f docker-compose.prod.yaml logs --follow

#-----------------------------------------------------------------------
# Deploy
#-----------------------------------------------------------------------
NODE=zlapik-compute-01-zlapik
rsync-with-delete: ## Rsync this repo to the remote server and delete files that are not in the repo
	rsync --archive --verbose --compress \
			--exclude='.git' \
			--exclude='.idea' \
			--exclude='.DS_Store' \
			--exclude='backend/target' \
			--exclude='backend/tmp' \
			--exclude='frontend/node_modules' \
			--exclude='frontend/dist' \
			--exclude='frontend/tmp' \
			--exclude='tmp' \
			--delete \
			$$PWD $(NODE):/home/zlapik/apps/

clean:
	rm -rf \
		frontend/dist \
		frontend/node_modules \
		backend/target


pack:
	zip -r wap.zip \
		frontend \
		backend \
		docker-compose.yaml docker-compose.prod.yaml Makefile README.md
