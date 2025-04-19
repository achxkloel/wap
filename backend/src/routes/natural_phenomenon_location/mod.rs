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

pub fn natural_phenomenon_location_router(db: PgPool) -> (Router, utoipa::openapi::OpenApi) {
    // let store = Arc::new(PgNaturalPhenomenonLocationService::new(db.clone()));
    let service = Arc::new(PgNaturalPhenomenonLocationService::new(db));

    // let _middleware = axum::middleware::from_fn_with_state(service.clone(), auth);

    let (router, api_docs) = OpenApiRouter::new()
        .routes(routes!(create_location))
        // .routes(routes!(get_all_locations))
        // .routes(routes!(get_location))
        // .routes(routes!(update_location))
        // .routes(routes!(delete_location))
        // .layer(_middleware)
        .with_state(service)
        .split_for_parts();

    (router, api_docs)
}
