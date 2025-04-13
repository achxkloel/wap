#----------------------------------------------------------------------
# Docker
#----------------------------------------------------------------------
build:
	docker compose build

up:
	docker compose -f docker-compose.yaml -f docker-compose.dev.yaml up

up-detach:
	docker compose -f docker-compose.yaml -f docker-compose.dev.yaml up --detach

logs:
	docker compose logs --follow

down:
	docker compose down

frontend-up:
	docker compose -f docker-compose.yaml -f docker-compose.dev.yaml up frontend

frontend-fish:
	docker compose -f docker-compose.yaml -f docker-compose.dev.yaml run --rm -it --service-ports frontend fish

backend-up:
	docker compose up backend

backend-fish:
	docker compose run --rm -it --service-ports backend fish

backend-exec:
	docker compose exec backend fish

pgsql-up:
	docker compose up pgsql

pgsql-fish:
	docker compose run --rm -it --service-ports pgsql fish

pgsql-exec:
	docker compose exec pgsql fish

