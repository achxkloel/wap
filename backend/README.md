# WAP - Backend

# Lint project!
```bash
cargo fmt
```

## Backend

#### Local Development

Run rustrover on your computer and let it setup project from Cargo.toml
then run be container using (it will also start db), which is also needed for compilation -> db schema validation

```bash
make -C .. backend-fish
```

Inside docker run:

```bash
cargo build

# More comfortable way to run program is:
cargo watch --exec run

# Run migrations
sqlx migrate run

# check migration state
sqlx migrate info
```

#### Docker Development

Use .devcontainers/devcontainers.json to set it up (but it not conformable)
