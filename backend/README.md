# # WAP - Backend

## Backend

### Local Development

Run rustrover on your computer and let it setup project from Cargo.toml
then run be container using (it will also start db), which is also needed for compilation -> db schema validation

```bash
make backend-fish
```

Inside docker run

```bash
make build # or: cargo build

# More comfortable way to run program is:
make watch-run # or: cargo watch -x run
```

### Docker Development

Use .devcontainers/devcontainers.json to set it up (but it not conformable)

## Database

### Migrate:

```bash
# Go into docker container
make backend-fish

# Run migrations
sqlx migrate run
# check migration state
sqlx migrate info
```
