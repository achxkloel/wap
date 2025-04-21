use utoipa_axum::routes;
use crate::routes::auth::middlewares::auth;
use crate::routes::settings::{handlers, services};
use crate::shared::models::AppState;

pub fn router(app: AppState) -> utoipa_axum::router::OpenApiRouter {
    let service = services::PgSettingsService::new(app.db.clone());

    let router = utoipa_axum::router::OpenApiRouter::new()
        .routes(routes!(handlers::get_settings))
        .routes(routes!(handlers::put_settings))
        .layer(axum::middleware::from_fn_with_state(app.clone(), auth))
        .with_state(service);

    router
}
