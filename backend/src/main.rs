// // mod routes;
// pub mod routes;
// mod structs;

use axum::response::Html;
use axum::{
    extract::{Json, State},
    http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    http::StatusCode,
    http::{HeaderValue, Method},
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
use backend::routes;
use backend::routes::auth::middleware::auth;
use routes::auth::middleware;
use tower_http::cors::CorsLayer;
use utoipa::openapi::security::SecurityScheme::ApiKey;
use utoipa::openapi::OpenApiBuilder;
use utoipa::OpenApi;
use utoipa_axum::router::UtoipaMethodRouterExt;
use utoipa_scalar::{Scalar, Servable};
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

    let allowed_origins = vec![
        "http://localhost:3000",
        "http://backend:3000",
        "http://localhost:5173",
    ];

    let cors = CorsLayer::new()
        .allow_origin(
            allowed_origins
                .into_iter()
                .map(|origin| HeaderValue::from_str(origin).unwrap())
                .collect::<Vec<_>>(),
        )
        .allow_methods(
            [
                Method::GET,
                Method::POST,
                Method::PATCH,
                Method::DELETE,
                Method::PUT,
                Method::OPTIONS,
                Method::HEAD,
            ]
            .to_vec(),
        )
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let (router, api_docs) = OpenApiRouter::new()
        // .nest("/api", api_router)
        // Auth
        .routes(routes!(backend::routes::auth::handlers::register))
        .routes(routes!(backend::routes::auth::handlers::login))
        .routes(routes!(backend::routes::auth::handlers::logout))
        .routes(routes!(backend::routes::auth::handlers::refresh))
        // Settings
        .routes(
            routes!(backend::routes::settings::handlers::put_settings)
                .layer(axum::middleware::from_fn_with_state(state.clone(), auth)),
        )
        .routes(
            routes!(backend::routes::settings::handlers::get_settings)
                .layer(axum::middleware::from_fn_with_state(state.clone(), auth)),
        )
        // Location
        .routes(
            routes!(backend::routes::location::handlers::get_all_locations)
                .layer(axum::middleware::from_fn_with_state(state.clone(), auth)),
        )
        .routes(
            routes!(backend::routes::location::handlers::get_location_by_id)
                .layer(axum::middleware::from_fn_with_state(state.clone(), auth)),
        )
        .routes(
            routes!(backend::routes::location::handlers::create_location)
                .layer(axum::middleware::from_fn_with_state(state.clone(), auth)),
        )
        .routes(
            routes!(backend::routes::location::handlers::update_location)
                .layer(axum::middleware::from_fn_with_state(state.clone(), auth)),
        )
        .routes(
            routes!(backend::routes::location::handlers::delete_location)
                .layer(axum::middleware::from_fn_with_state(state.clone(), auth)),
        )
        .layer(cors)
        // .routes(routes!(health))
        .with_state(state)
        .split_for_parts();

    let router = Router::new()
        .merge(router)
        // .layer(cors)
        .route("/", get(|| async { "Hello, World!" }))
        .merge(Scalar::with_url("/scalar", api_docs));

    // run our app with hyper, listening globally on port 3000
    println!(
        "Listening on http://localhost:3000, for OpenAPI docs go to: http://localhost:3000/scalar"
    );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
    // axum::serve(listener, app.into_router()).await.unwrap();
}
