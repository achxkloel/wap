use crate::routes::auth::middlewares::auth;
use crate::routes::natural_phenomenon_location::handlers::*;
use crate::routes::natural_phenomenon_location::services::{
    NaturalPhenomenonLocationService, PgNaturalPhenomenonLocationService,
};
use crate::shared::models::AppState;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

/// Generic router allowing injection of any implementation of the domain service
pub fn router_with_service<S>(app: AppState, service: Arc<S>) -> OpenApiRouter
where
    S: NaturalPhenomenonLocationService + Send + Sync + 'static,
{
    let get_all_locations = get_all_locations::<S>;
    let get_location = get_location::<S>;
    let create_location = create_location::<S>;
    let update_location = update_location::<S>;
    let delete_location = delete_location::<S>;

    OpenApiRouter::new()
        .routes(routes!(get_all_locations))
        .routes(routes!(get_location))
        .routes(routes!(create_location))
        .routes(routes!(update_location))
        .routes(routes!(delete_location))
        .layer(axum::middleware::from_fn_with_state(app.clone(), auth))
        .with_state(service)
}

/// Convenience router using the Postgres-backed implementation
pub fn router(app: AppState) -> OpenApiRouter {
    let service = Arc::new(PgNaturalPhenomenonLocationService::new(app.db.clone()));
    router_with_service(app, service)
}
