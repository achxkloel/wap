use crate::routes::weather_location::services::WeatherLocationAppStateImpl;
use crate::routes::weather_location::{WeatherLocation, WeatherLocationId, WeatherLocationService,
};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use serde::Deserialize;
use utoipa::ToSchema;
use crate::shared::models::DatabaseId;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateWeatherLocationRequest {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub is_default: bool,
    pub description: Option<String>,
}

#[utoipa::path(
    get,
    path = "/weather_locations",
    responses(
        (status = 200, description = "All user locations", body = Vec<WeatherLocation>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_all_locations(
    State(service): State<WeatherLocationAppStateImpl>,
    Extension(user_id): Extension<DatabaseId>,
) -> anyhow::Result<Json<Vec<WeatherLocation>>, (StatusCode, String)> {
    let locations = service
        .get_all(&user_id)
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
pub async fn get_location_by_id(
    State(service): State<WeatherLocationAppStateImpl>,
    Extension(user_id): Extension<DatabaseId>,
    Path(id): Path<i32>,
) -> anyhow::Result<Json<WeatherLocation>, (StatusCode, String)>
{
    let location = service
        .get_by_id(&user_id, &WeatherLocationId(id))
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
pub async fn create_location(
    State(service): State<WeatherLocationAppStateImpl>,
    Extension(user_id): Extension<DatabaseId>,
    Json(request): Json<CreateWeatherLocationRequest>,
) -> anyhow::Result<Json<WeatherLocation>, (StatusCode, String)>
{
    let location = WeatherLocation {
        id: None,
        user_id,
        name: request.name,
        latitude: request.latitude,
        longitude: request.longitude,
        is_default: request.is_default,
        description: request.description,
    };

    let created_location = service
        .create(location)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(created_location))
}

#[utoipa::path(
    delete,
    path = "/weather_locations/{id}",
    responses(
        (status = 204, description = "Location deleted"),
        (status = 404, description = "Location not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = i32, Path, description = "Location ID")
    )
)]
pub async fn delete_location(
    State(service): State<WeatherLocationAppStateImpl>,
    Extension(user_id): Extension<DatabaseId>,
    Path(id): Path<i32>,
) -> anyhow::Result<StatusCode, (StatusCode, String)>
{
    service
        .delete(&user_id, &WeatherLocationId(id))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}
