use axum::{
    http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    http::{HeaderValue, Method},
    Json, Router,
};
use futures_util::{future, StreamExt};
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::future::Future;
use std::future::IntoFuture;
use std::process::exit;
use tokio_util::sync::CancellationToken;
use tracing::log::LevelFilter;
use tracing::{Instrument, Level};
use utoipa_axum::{router::OpenApiRouter, routes};

use backend::config::{WapSettings, WapSettingsImpl};
use backend::routes::auth::models::{
    AuthErrorKind, LoginResponse, LoginUserSchema, RegisterUserRequestSchema, RegisterUserSchema,
    UserData,
};
use backend::routes::auth::services::{create_login_response, AuthServiceImpl};
use backend::shared::models::AppState;
use tower_http::cors::CorsLayer;
use tracing_subscriber::filter::Directive;
use tracing_subscriber::EnvFilter;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_axum::router::UtoipaMethodRouterExt;
use utoipa_scalar::{Scalar, Servable};

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
    // OpenAPI
    #[derive(OpenApi)]
    #[openapi(
        modifiers(&SecurityAddon),
        tags(
            (name = "Wap Backend - OpenApi documentation", description = "")
        )
    )]
    struct ApiDoc;

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            if let Some(components) = openapi.components.as_mut() {
                components.add_security_scheme(
                    "Authorization 1",
                    SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
                )
            }
        }
    }

    let setting_router = backend::routes::settings::handlers::router(app.clone());
    let auth_router = backend::routes::auth::handlers::router(app.clone());
    let natural_phenomenon_location_router =
        backend::routes::natural_phenomenon_locations::handlers::router(app.clone());
    let weather_location_router = backend::routes::weather_locations::handlers::router(app.clone());
    let uploads_router = backend::routes::uploads::handlers::router(app.clone());

    let router = OpenApiRouter::with_openapi(ApiDoc::openapi());

    router
        .merge(setting_router)
        .merge(auth_router)
        .merge(weather_location_router)
        .merge(natural_phenomenon_location_router)
        .merge(uploads_router)
        .layer(prepare_cors())
}

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

pub trait HaltOnSignal {
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

async fn create_development_user(app: &AppState) {
    let register_request = RegisterUserRequestSchema {
        email: "test1@wap.com".into(),
        password: "test1@wap.com".into(),
    };
    let auth_service = backend::routes::auth::services::AuthService {
        db: app.db.clone(),
        settings: app.settings.clone(),
        http: Default::default(),
    };

    // let user = auth_service.register_new_user(&register_request).await;
    let user = match auth_service.register_new_user(&register_request).await {
        Ok(user) => user,
        Err((_, Json(kind))) => {
            if kind == AuthErrorKind::UserAlreadyExists {
                tracing::info!("Development user already exists");
                auth_service
                    .get_user_by_id_or_email(&None, &Some(register_request.email.clone()))
                    .await
                    .unwrap()
            } else {
                tracing::error!("Failed to create development user: {:?}", kind);
                return;
            }
        }
    };

    let _ = auth_service.change_password(user.id, &"", &register_request.password, true).await;

    let login_request = LoginUserSchema {
        email: register_request.email.clone(),
        password: register_request.password.clone(),
    };
    let auth_result = auth_service.login(&login_request).await;

    let auth_result = match auth_result {
        Ok(auth_result) => auth_result,
        Err(e) => {
            tracing::error!("Failed to login user: {:?}", e);
            return;
        }
    };
    let data = create_login_response(user.clone(), &auth_service).await;

    tracing::info!("Development user created and logged in: {:?}", auth_result);
    tracing::debug!("You can login with: Bearer {}", data.access_token);
}

#[tokio::main]
async fn main() {
    // Logging
    let filter = EnvFilter::builder()
        .with_default_directive(Level::DEBUG.into())
        .from_env()
        .unwrap()
        .add_directive("backend=debug".parse().unwrap())
        .add_directive("sqlx=info".parse().unwrap());


    let _r = tracing_subscriber::fmt::fmt()
        .without_time()
        .with_max_level(Level::DEBUG)
        .with_file(true)
        .with_line_number(true)
        .with_env_filter(filter)
        .try_init();

    tracing::debug!("Starting server");

    let state = AppState {
        db: init_db().await,
        settings: WapSettings::init().await,
    };

    tracing::info!("Loaded config: {:#?}", state.settings);
    if state.settings.is_development().await {
        create_development_user(&state).await;
    }

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
