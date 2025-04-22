#----------------------------------------------------------------------
# Docker
#----------------------------------------------------------------------
build:
	cargo build

run:
	cargo run

watch-run:
	cargo watch -x run

migrate:
	sqlx migrate run

migration-info:
	sqlx migrate info

connect-to-db-local:
	psql postgres://postgres:postgres@localhost:5432/wap_db

test:
	cargo test -- --nocapture

test-watch-ignore:
	cargo watch --exec "test -- --nocapture --ignored"

test-watch-all:
	cargo watch --exec "test -- --nocapture"

#test-watch-current:
#	cargo watch --exec "test -- --exact routes::auth::services::tests"

test-watch-current:
	cargo watch --exec "test routes::auth::services -- --nocapture"
