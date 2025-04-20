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

