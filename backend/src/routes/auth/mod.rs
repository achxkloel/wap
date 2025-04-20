use utoipa_axum::router::UtoipaMethodRouterExt;
use utoipa_axum::routes;

pub mod handlers;
pub mod middlewares;
pub mod models;
pub mod service;
pub mod utils;

use crate::routes::auth::service::{AuthService, PgAuthService};
use crate::shared::models::AppState;

pub fn router(app: AppState) -> utoipa_axum::router::OpenApiRouter {
    let service = PgAuthService::new(app.db.clone(), app.settings.clone());

    let router = utoipa_axum::router::OpenApiRouter::new()
        .routes(routes!(handlers::register))
        .routes(routes!(handlers::login))
        .routes(
            routes!(handlers::refresh).layer(axum::middleware::from_fn_with_state(
                app.clone(),
                middlewares::auth,
            )),
        )
        .with_state(service);

    router
}
