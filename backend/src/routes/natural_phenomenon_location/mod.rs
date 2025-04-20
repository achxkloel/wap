use std::sync::Arc;
use utoipa_axum::routes;

mod domains;
mod handlers;
mod models;
mod services;

pub use domains::{
    CreateNaturalPhenomenonLocationRequest, NaturalPhenomenonLocation, NaturalPhenomenonLocationId,
    UserId,
};

use crate::routes::auth::middlewares::auth;
use crate::routes::natural_phenomenon_location::models::SharedService;
use crate::shared::models::AppState;
pub use services::{NaturalPhenomenonLocationService, PgNaturalPhenomenonLocationService};

pub fn router(app: AppState) -> utoipa_axum::router::OpenApiRouter {
    let service: SharedService = Arc::new(PgNaturalPhenomenonLocationService::new(app.clone().db));

    let router = utoipa_axum::router::OpenApiRouter::new()
        .routes(routes!(handlers::get_all_locations))
        .routes(routes!(handlers::get_location))
        .routes(routes!(handlers::create_location))
        .routes(routes!(handlers::update_location))
        .routes(routes!(handlers::delete_location))
        .layer(axum::middleware::from_fn_with_state(app.clone(), auth))
        .with_state(service);

    router
}
