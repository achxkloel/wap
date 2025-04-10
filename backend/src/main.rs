// // mod routes;
// pub mod routes;
// mod structs;

use axum::response::Html;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::future::IntoFuture;
use std::sync::Arc;
use utoipa_axum::{router::OpenApiRouter, routes, PathItemExt};

use axum::routing::get;
use backend::config::Config;
use utoipa::openapi::security::SecurityScheme::ApiKey;
use utoipa::openapi::OpenApiBuilder;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};
// use crate::structs::AppState;
// use routes::auth::{signup};

//--------------------------------------------------------------------------------------------------
#[derive(OpenApi)]
#[openapi(info(description = "OpenAPI schema WAP"))]
struct ApiDoc;

//--------------------------------------------------------------------------------------------------
pub async fn init_db() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:postgres@localhost:5432/postgres".into());

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Cannot connect to the database")
}

// -------------------------------------------------------------------------------------------------
#[tokio::main]
async fn main() {
    // Initialize database
    let db = init_db().await;

    // Load env variables
    let config = Config::init();

    // Create shared application state
    let state = Arc::new(backend::model::AppState {
        db: db.clone(),
        env: config.clone(),
    });

    println!("{}", state.env);

    async fn show_scalar() -> Html<String> {
        // Generate HTML from your OpenAPI doc using Scalar
        let html = utoipa_scalar::Scalar::new(ApiDoc::openapi()).to_html();
        Html(html)
    }

    let (router, api_docs) = OpenApiRouter::new()
        // .nest("/api", api_router)
        .routes(routes!(backend::routes::auth::handlers::register))
        .routes(routes!(backend::routes::auth::handlers::login))
        // .routes(routes!(health))
        .with_state(state)
        .split_for_parts();

    let router = Router::new()
        .merge(router)
        // .layer(cors)
        .route("/", get(|| async { "Hello, World!" }))
        .merge(Scalar::with_url("/scalar", api_docs));

    // run our app with hyper, listening globally on port 3000
    println!("Listening on http://localhost:3000 - for all endpoint go to: http://localhost:3000/scalar");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
    // axum::serve(listener, app.into_router()).await.unwrap();
}
