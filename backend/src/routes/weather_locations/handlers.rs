use crate::routes::auth::middlewares::auth;
use crate::routes::auth::models::UserDb;
use crate::routes::auth::services::AuthService;
use crate::routes::weather_locations::models::{CreateWeatherLocationRequest, WeatherLocation};
use crate::routes::weather_locations::services::{
    WeatherLocationService, WeatherLocationServiceImpl,
};
use crate::shared::models::{AppState, DatabaseId};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use std::sync::Arc;
use utoipa::ToSchema;
use utoipa_axum::routes;

#[utoipa::path(
    get,
    path = "/weather_locations",
    responses(
        (status = 200, description = "All user locations", body = Vec<WeatherLocation>, content_type = "application/json"),
        (status = 500, description = "Internal server error", content_type = "application/json")
    )
)]
pub async fn get_all_locations<S>(
    State(service): State<Arc<S>>,
    Extension(user): Extension<UserDb>,
) -> anyhow::Result<Json<Vec<WeatherLocation>>, (StatusCode, String)>
where
    S: WeatherLocationServiceImpl,
{
    let locations = service
        .get_all(&user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(locations))
}

#[utoipa::path(
    get,
    path = "/weather_locations/{id}",
    responses(
        (status = 200, description = "Location found", body = WeatherLocation),
        (status = 404, description = "Location not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = i32, Path, description = "Location ID")
    )
)]
pub async fn get_location_by_id<S>(
    State(service): State<Arc<S>>,
    Extension(user): Extension<UserDb>,
    Path(id): Path<i32>,
) -> anyhow::Result<Json<WeatherLocation>, (StatusCode, String)>
where
    S: WeatherLocationServiceImpl,
{
    let location = service
        .get_by_id(&user.id, &DatabaseId(id))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(location))
}

#[utoipa::path(
    post,
    path = "/weather_locations",
    request_body = CreateWeatherLocationRequest,
    responses(
        (status = 201, description = "Location created", body = WeatherLocation),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_location<S>(
    State(service): State<Arc<S>>,
    Extension(user): Extension<UserDb>,
    Json(request): Json<CreateWeatherLocationRequest>,
) -> anyhow::Result<Json<WeatherLocation>, (StatusCode, String)>
where
    S: WeatherLocationServiceImpl,
{
    let location = CreateWeatherLocationRequest {
        user_id: user.id,
        name: request.name,
        latitude: request.latitude,
        longitude: request.longitude,
        is_default: request.is_default,
        description: request.description,
    };

    let created_location = service
        .create(&location)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(created_location))
}

#[utoipa::path(
    delete,
    path = "/weather_locations/{id}",
    responses(
        (status = 204, description = "Location deleted", content_type = "application/json"),
        (status = 404, description = "Location not found", content_type = "application/json"),
        (status = 500, description = "Internal server error", content_type = "application/json")
    ),
    params(
        ("id" = i32, Path, description = "Location ID")
    )
)]
pub async fn delete_location<S>(
    State(service): State<Arc<S>>,
    Extension(user): Extension<UserDb>,
    Path(id): Path<DatabaseId>,
) -> anyhow::Result<StatusCode, (StatusCode, String)>
where
    S: WeatherLocationServiceImpl,
{
    service
        .delete(&user.id, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}

// pub fn router<Z:WeatherLocationService>(app: AppState<Z>) -> utoipa_axum::router::OpenApiRouter {
pub fn router(app: AppState) -> utoipa_axum::router::OpenApiRouter {
    let weather_service = Arc::new(WeatherLocationService { db: app.db.clone() });
    let auth_service = Arc::new(AuthService {
        db: app.db.clone(),
        settings: app.settings.clone(),
        http: Default::default(),
    });

    let router = utoipa_axum::router::OpenApiRouter::new()
        .routes(routes!(get_all_locations))
        .routes(routes!(get_location_by_id))
        .routes(routes!(create_location))
        .routes(routes!(delete_location))
        .layer(axum::middleware::from_fn_with_state(auth_service, auth))
        .with_state(weather_service);

    router
}
