use crate::routes::auth::middlewares::auth;
use crate::routes::settings::services::SettingsService;
use crate::routes::settings::{
    handlers::{__path_get_settings, __path_put_settings, get_settings, put_settings},
    services,
};
use crate::shared::models::AppState;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

/// Fully‐generic: you supply the `service: S`.
pub fn router_with_service<S>(app: AppState, service: Arc<S>) -> OpenApiRouter
where
    S: SettingsService + Send + Sync + 'static,
{
    let get_settings = get_settings::<S>;
    let put_settings = put_settings::<S>;
    OpenApiRouter::new()
        .routes(routes!(get_settings))
        .routes(routes!(put_settings))
        .layer(axum::middleware::from_fn_with_state(app.clone(), auth))
        .with_state(service)
}

/// A convenience wrapper that uses the real Postgres implementation.
pub fn router(app: AppState) -> OpenApiRouter {
    let service = services::PgSettingsService::new(app.db.clone());
    let arc_service = Arc::new(service);
    router_with_service(app.clone(), arc_service)
}
