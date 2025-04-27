#----------------------------------------------------------------------
# Frontend
#----------------------------------------------------------------------
FROM node:23.11 as frontend

ENV WORKDIR /opt/frontend
WORKDIR $WORKDIR

COPY frontend/ $WORKDIR/
RUN apt-get update -y && apt-get install -y fish vim tree cloc;
#RUN set -x; npm install; npm run build;

#----------------------------------------------------------------------
# Backend
#----------------------------------------------------------------------
FROM rust:1.85 AS backend

ENV WORKDIR /opt/backend
ENV HOMEDIR /root
WORKDIR $WORKDIR

COPY backend/.cargo $HOMEDIR/.cargo

# Install development packages
RUN apt-get update -y && apt-get install -y fish vim git;
RUN cargo install cargo-watch;

# Install production packages
RUN apt-get update -y && apt-get install -y postgresql postgresql-contrib cloc mold clang;
RUN cargo install sqlx-cli --no-default-features --features postgres;

#----------------------------------------------------------------------
# DB
#----------------------------------------------------------------------
FROM postgres:17.4 AS db

ENV WORKDIR /opt/db
WORKDIR $WORKDIR

RUN apt-get update -y && apt-get install -y fish vim;

FROM nginx:alpine as nginx
RUN rm -rf /usr/share/nginx/html/*
#RUN apt-get update -y && apt-get install -y fish tree;
COPY --from=frontend /opt/frontend/dist/ /usr/share/nginx/html/frontend/
COPY nginx/conf.d/ /etc/nginx/conf.d/
COPY nginx/snippets/ /etc/nginx/snippets/
COPY nginx/html/ /etc/nginx/html/
#COPY nginx/html/well-known/ /usr/share/nginx/frontend/well-known/
#COPY nginx/html/well-known/ /usr/share/nginx/well-known/
