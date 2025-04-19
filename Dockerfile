#----------------------------------------------------------------------
# Frontend
#----------------------------------------------------------------------
FROM node:23.11 as frontend

ENV WORKDIR /opt/frontend
WORKDIR $WORKDIR

COPY frontend/package.json frontend/package-lock.json $WORKDIR/
RUN <<EOF
    apt-get update -y && apt-get install -y fish vim tree
    npm install
EOF

#----------------------------------------------------------------------
# Backend
#----------------------------------------------------------------------
FROM rust:1.85 AS backend

ENV WORKDIR /opt/backend
WORKDIR $WORKDIR

# Install development packages
RUN <<EOF
    apt-get update -y && apt-get install -y fish vim git;
    cargo install cargo-watch;
EOF

# Install production packages
RUN <<EOF
    apt-get update -y && apt-get install -y postgresql postgresql-contrib;
    cargo install sqlx-cli --no-default-features --features postgres;
EOF

#----------------------------------------------------------------------
# DB
#----------------------------------------------------------------------
FROM postgres:17.4 AS db

ENV WORKDIR /opt/db
WORKDIR $WORKDIR

RUN <<EOF
    apt-get update -y && apt-get install -y fish vim;
EOF

