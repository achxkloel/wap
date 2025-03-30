#----------------------------------------------------------------------
# Frontend
#----------------------------------------------------------------------
FROM node:18.10 as frontend

ENV WORKDIR /opt/frontend
WORKDIR $WORKDIR

COPY frontend/package.json frontend/package-lock.json $WORKDIR/
RUN <<EOF
    apt-get update -y && apt-get install -y fish vim
    npm install
EOF

#----------------------------------------------------------------------
# Backend
#----------------------------------------------------------------------
FROM rust:1.85 AS backend

ENV WORKDIR /opt/backend
WORKDIR $WORKDIR

COPY backend/Cargo.toml backend/Cargo.lock $WORKDIR/
RUN <<EOF
    apt-get update -y && apt-get install -y fish vim
    cargo install cargo-watch
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

