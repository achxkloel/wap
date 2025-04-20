use crate::routes::auth::middlewares::auth;
use crate::shared::models::AppState;
use utoipa_axum::routes;

pub mod handlers;

pub fn router(app: AppState) -> utoipa_axum::router::OpenApiRouter {
    let router = utoipa_axum::router::OpenApiRouter::new()
        .routes(routes!(handlers::put_settings))
        .routes(routes!(handlers::get_settings))
        .layer(axum::middleware::from_fn_with_state(app.clone(), auth))
        .with_state(app);
    router
}
