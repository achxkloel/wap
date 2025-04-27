#----------------------------------------------------------------------
# Docker
#----------------------------------------------------------------------
build:
	cargo watch --watch src --watch migrations/ --watch Cargo.toml --exec "build"

run:
	cargo watch --watch src --watch migrations/ --watch Cargo.toml --exec "run"

run-prod:
	cargo run --release

# Tests
test:
	cargo watch --watch src --watch migrations/ --watch Cargo.toml --exec "test -- --nocapture"

_example-test-one:
	cargo watch test test_natural_phenomenon_location_crud

test-ignore:
	cargo watch --exec "test -- --nocapture --ignored"

test-all:
	cargo watch --exec "test -- --nocapture"

test-auth-service:
	cargo watch --exec "test routes::auth::services -- --nocapture"

test-natural-phenomenon-locations-service:
	cargo watch --exec "test routes::natural_phenomenon_locations::services -- --nocapture"

test-settings-service:
	cargo watch --exec "test routes::settings::services -- --nocapture"

test-weather-locations-service:
	cargo watch --exec "test routes::weather_locations::services -- --nocapture"

# Database
migrate:
	sqlx migrate run

migration-info:
	sqlx migrate info

connect-to-db-local:
	psql postgres://postgres:postgres@localhost:5432/wap_db

