#----------------------------------------------------------------------
# Backend
#----------------------------------------------------------------------
FROM rust:1.85 AS backend

ENV WORKDIR /opt/backend
WORKDIR $WORKDIR

COPY Cargo.toml Cargo.lock $WORKDIR/
RUN <<EOF
    apt-get update -y && apt-get install -y fish vim postgresql postgresql-contrib
    cargo install cargo-watch
    cargo install sqlx-cli --no-default-features --features postgres
EOF

#----------------------------------------------------------------------
# DB
#----------------------------------------------------------------------
FROM postgres:17.4 AS db

ENV WORKDIR /opt/db
WORKDIR $WORKDIR

RUN <<EOF
    apt-get update -y && apt-get install -y fish vim
EOF

