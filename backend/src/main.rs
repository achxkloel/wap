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

    // Create shared application state
    let state = Arc::new(backend::structs::AppState {
        db,
        jwt_secret: "my_secret_key".to_string(),
    });

    async fn show_scalar() -> Html<String> {
        // Generate HTML from your OpenAPI doc using Scalar
        let html = utoipa_scalar::Scalar::new(ApiDoc::openapi()).to_html();
        Html(html)
    }

    let (router, api_docs) = OpenApiRouter::new()
        // .nest("/api", api_router)
        .routes(routes!(backend::authjwt::handlers_auth::register))
        // .routes(routes!(health))
        .with_state(state)
        .split_for_parts();

    let router = Router::new()
        .merge(router)
        // .layer(cors)
        .route("/", get(|| async { "Hello, World!" }))
        .merge(Scalar::with_url("/scalar", api_docs));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
    // axum::serve(listener, app.into_router()).await.unwrap();
}
