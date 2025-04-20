use axum::{
    http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    http::{HeaderValue, Method},
    Router,
};
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::future::IntoFuture;
use utoipa_axum::{router::OpenApiRouter, routes};

use backend::config::WapSettings;
use backend::shared::models::AppState;
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_axum::router::UtoipaMethodRouterExt;
use utoipa_scalar::{Scalar, Servable};

#[derive(OpenApi)]
// #[openapi(
//     tags(
//             (name = "fooo", description = "Todo items management API")
//     )
// )]
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

async fn app_router(app: AppState) -> OpenApiRouter {
    let setting_router = backend::routes::settings::router(app.clone());
    let auth_router = backend::routes::auth::router(app.clone());
    let natural_phenomenon_location_router =
        backend::routes::natural_phenomenon_location::router(app.clone());
    let weather_location_router = backend::routes::weather_location::router(app.clone());

    let router = OpenApiRouter::new().layer(prepare_cors());

    router
        .merge(setting_router)
        .merge(auth_router)
        .merge(weather_location_router)
        .merge(natural_phenomenon_location_router)
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

use std::future::Future;

use futures_util::{future, StreamExt};
use tokio_util::sync::CancellationToken;

async fn interrupt_signal<FT>(ft: FT)
where
    FT: Future + Send + 'static,
{
    use tokio::signal::unix;
    use tokio_stream::wrappers::SignalStream;

    let sigterm = SignalStream::new(
        unix::signal(unix::SignalKind::terminate()).expect("BUG: Error listening for SIGTERM"),
    );
    let sigint = SignalStream::new(
        unix::signal(unix::SignalKind::interrupt()).expect("BUG: Error listening for SIGINT"),
    );

    future::select(sigterm.into_future(), sigint.into_future()).await;
    ft.await;
}

pub(crate) trait HaltOnSignal {
    fn halt_on_signal(&self);
}

impl HaltOnSignal for CancellationToken {
    fn halt_on_signal(&self) {
        let this = self.clone();
        tokio::spawn(interrupt_signal(async move { this.cancel() }));
        // tokio::spawn(interrupt_signal(this.cancel()));
    }
}

// async fn test_end_print(token: CancellationToken) {
//     token.cancelled().await;
//     tracing::info!("Starting graceful shutdown");
// }

#[tokio::main]
async fn main() {
    // Logging
    let _r = tracing_subscriber::fmt::fmt().try_init();

    let state = AppState {
        db: init_db().await,
        settings: WapSettings::init(),
    };

    tracing::info!("Loaded config: {:#?}", state.settings);

    let (router, api_docs) = app_router(state).await.split_for_parts();

    let router = Router::new()
        .merge(router)
        .merge(Scalar::with_url("/scalar", api_docs));

    // run our app with hyper, listening globally on port 3000
    tracing::info!(
    "Starting up the server on http://localhost:3000, for OpenAPI docs go to: http://localhost:3000/scalar"
    );

    // tracing
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let token = CancellationToken::new();
    token.halt_on_signal();
    // tokio::spawn(test_end_print(token.clone()));
    axum::serve(listener, router)
        .with_graceful_shutdown(token.cancelled_owned())
        .await
        .unwrap();
}
