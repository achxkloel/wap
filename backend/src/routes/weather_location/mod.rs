use utoipa_axum::routes;

pub mod domains;
pub mod handlers;
pub mod models;
pub mod services;

use crate::routes::auth::middlewares::auth;
use crate::routes::weather_location::handlers::{
    __path_create_location, __path_delete_location, __path_get_all_locations,
    __path_get_location_by_id, create_location, delete_location, get_all_locations,
    get_location_by_id,
};
use crate::routes::weather_location::services::WeatherLocationAppStateImpl;
use crate::shared::models::AppState;
pub use domains::*;
pub use services::WeatherLocationService;

// pub fn router<Z:WeatherLocationService>(app: AppState<Z>) -> utoipa_axum::router::OpenApiRouter {
pub fn router(app: AppState) -> utoipa_axum::router::OpenApiRouter {
    let state = WeatherLocationAppStateImpl { db: app.db.clone() };
    let router = utoipa_axum::router::OpenApiRouter::new()
        .routes(routes!(get_all_locations))
        .routes(routes!(get_location_by_id))
        .routes(routes!(create_location))
        .routes(routes!(delete_location))
        .layer(axum::middleware::from_fn_with_state(app.clone(), auth))
        .with_state(state);

    router
}
