use axum::{
    http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    http::{HeaderValue, Method},
    Router,
};
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::future::IntoFuture;
use std::sync::Arc;
use tokio::sync::Mutex;
use utoipa_axum::{router::OpenApiRouter, routes};

use backend::config::Config;
use backend::routes::auth::middlewares::auth;
use backend::routes::natural_phenomenon_location::natural_phenomenon_location_router;
use backend::shared::models::AppState;
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_axum::router::UtoipaMethodRouterExt;
use utoipa_scalar::{Scalar, Servable};

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    tags(
            (name = TODO_TAG, description = "Todo items management API")
    )
)]
struct ApiDoc;

pub async fn init_db() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:postgres@localhost:5432/postgres".into());

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database")
}

fn prepare_cors() -> CorsLayer {
    let allowed_origins = vec![
        "http://localhost:3000",
        "http://backend:3000",
        "http://localhost:5173",
    ];

    CorsLayer::new()
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
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
}

async fn app_router(state: Arc<Mutex<AppState>>) -> (Router, utoipa::openapi::OpenApi) {
    let auth_middleware = axum::middleware::from_fn_with_state(state.clone(), auth);

    let db = state.lock().await.db.clone();
    let natural_phenomenon_location_router = natural_phenomenon_location_router(db);

    let (router, api_docs) = OpenApiRouter::new()
        // Auth routes
        .routes(routes!(backend::routes::auth::handlers::register))
        .routes(routes!(backend::routes::auth::handlers::login))
        .routes(routes!(backend::routes::auth::handlers::logout))
        .routes(routes!(backend::routes::auth::handlers::refresh))
        // Settings routes
        // .routes(routes!(backend::routes::settings::handlers::put_settings).layer(auth_middleware))
        // .routes(routes!(backend::routes::settings::handlers::get_settings).layer(auth_middleware))
        // .routes( routes!(get_settings) //.layer(axum::middleware::from_fn_with_state(state.clone(), auth)),
        //              )
        // Weather Location routes
        .routes(
            routes!(backend::routes::weather_location::handlers::create_weather_location)
                .layer(axum::middleware::from_fn_with_state(state.clone(), auth)),
        )
        .routes(
            routes!(backend::routes::weather_location::handlers::delete_weather_location)
                .layer(axum::middleware::from_fn_with_state(state.clone(), auth)),
        )
        .layer(prepare_cors())
        .with_state(state)
        .split_for_parts();

    (
        router.merge(natural_phenomenon_location_router.0),
        api_docs.merge_from(natural_phenomenon_location_router.1),
    )
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct Login {
    username: String,
    password: String,
}

#[tokio::main]
async fn main() {
    // Initialize database
    let db = init_db().await;

    let v = vec![1, 2, 3];
    println!("{:?}", v);

    // Load env variables
    let config = Config::init();

    // Create shared application state
    let state = Arc::new(AppState {
        db: db.clone(),
        env: config.clone(),
    });

    println!("Starting server with config: {:?}", state.env);

    let (router, api_docs) = app_router(state);

    let router = Router::new()
        .merge(router)
        .merge(Scalar::with_url("/scalar", api_docs));
    // .route("/", get(|| async { "Hello, World!" }))

    // run our app with hyper, listening globally on port 3000
    println!(
        "Listening on http://localhost:3000, for OpenAPI docs go to: http://localhost:3000/scalar"
    );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
