use std::sync::Arc;
use axum::Router;
use sqlx::PgPool;
use utoipa_axum::routes;

mod domain;
mod handlers;
mod model;
mod service;

pub use domain::{
    CreateNaturalPhenomenonLocationRequest, NaturalPhenomenonLocation, NaturalPhenomenonLocationId,
    UserId,
};

pub use handlers::{
    __path_create_location,
    __path_update_location,
    // __path_delete_location,
    create_location,
    update_location,
    // delete_location,
};

use crate::routes::natural_phenomenon_location::service::SharedService;
pub use service::{NaturalPhenomenonLocationService, PgNaturalPhenomenonLocationService};

pub fn natural_phenomenon_location_router(db: PgPool) -> (Router, utoipa::openapi::OpenApi) {
    let service: SharedService = Arc::new(PgNaturalPhenomenonLocationService::new(db));

    let (router, api) = utoipa_axum::router::OpenApiRouter::new()
        .routes(routes!(create_location))
        .routes(routes!(update_location))
        // .routes(routes!(delete_location))
        .with_state(service) // type matches the handler’s `State`
        .split_for_parts();

    (router, api)
}
