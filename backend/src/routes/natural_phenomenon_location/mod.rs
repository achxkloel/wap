use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;
use utoipa_axum::{router::OpenApiRouter, routes};

mod domain;
mod handlers;
mod model;
mod service;

pub use domain::{
    CreateNaturalPhenomenonLocationRequest, NaturalPhenomenonLocation, NaturalPhenomenonLocationId,
    UserId,
};

pub use handlers::{
    // delete_location, __path_delete_location,
    // get_all_locations, __path_get_all_locations,
    // get_location, __path_get_location,
    // update_location, __path_update_location,
    create_location, __path_create_location,
};

pub use service::{NaturalPhenomenonLocationService, PgNaturalPhenomenonLocationService};
use crate::routes::natural_phenomenon_location::service::SharedService;

pub fn natural_phenomenon_location_router(db: PgPool) -> (Router, utoipa::openapi::OpenApi) {
    let service: SharedService = std::sync::Arc::new(PgNaturalPhenomenonLocationService::new(db));

    let (router, api) = utoipa_axum::router::OpenApiRouter::new()
        .routes(routes!(create_location))
        .with_state(service)          // type matches the handler’s `State`
        .split_for_parts();

    (router, api)
}
