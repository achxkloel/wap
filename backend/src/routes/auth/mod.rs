use utoipa_axum::router::UtoipaMethodRouterExt;
use utoipa_axum::routes;
use crate::routes::weather_location::services::WeatherLocationAppStateImpl;
use crate::shared::models::AppState;

mod handlers;
pub mod models;

pub mod middlewares;

pub fn router(app: AppState) -> utoipa_axum::router::OpenApiRouter {
    let router = utoipa_axum::router::OpenApiRouter::new()
        .routes(routes!(handlers::register))
        .routes(routes!(handlers::login))
        // .routes(routes!(handlers::logout).layer(axum::middleware::from_fn_with_state(app.clone(), middlewares::auth)))
        .routes(routes!(handlers::refresh).layer(axum::middleware::from_fn_with_state(app.clone(), middlewares::auth)))
        .with_state(app);
    router
}
