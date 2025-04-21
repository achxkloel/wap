use serde::Serialize;

use axum::extract::{Extension, Json, Path, State};

use crate::routes::natural_phenomenon_location::domains::{
    CreateNaturalPhenomenonLocationRequest, NaturalPhenomenonLocation,
};
use crate::routes::natural_phenomenon_location::models::{
    SharedService, UpdateNaturalPhenomenonLocationRequest,
    UpdateNaturalPhenomenonLocationRequestWithIds,
};
use crate::routes::natural_phenomenon_location::services::NaturalPhenomenonLocationService;
use crate::routes::natural_phenomenon_location::NaturalPhenomenonLocationId;
use anyhow::Result;
use axum::http::StatusCode;
use utoipa::ToSchema;
use crate::shared::models::DatabaseId;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

#[utoipa::path(
    get,
    path = "/natural_phenomenon_locations",
    responses(
        (status = 200, description = "All user locations", body = Vec<NaturalPhenomenonLocation>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_all_locations(
    State(service): State<SharedService>,
    Extension(user_id): Extension<DatabaseId>,
) -> Result<Json<Vec<NaturalPhenomenonLocation>>, (StatusCode, String)> {
    let locations = service
        .get_all(user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(locations))
}

#[utoipa::path(
    get,
    path = "/natural_phenomenon_locations/{id}",
    params(
        ("id" = NaturalPhenomenonLocationId, Path, description = "Location ID to retrieve"),
    ),
    responses(
        (status = 200, description = "Location found", body = NaturalPhenomenonLocation),
        (status = 404, description = "Location not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_location(
    State(service): State<SharedService>,
    Extension(user_id): Extension<DatabaseId>,
    Path(id): Path<NaturalPhenomenonLocationId>,
) -> Result<Json<NaturalPhenomenonLocation>, (StatusCode, String)> {
    let location = service
        .get_by_id(user_id, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(location))
}

#[utoipa::path(
    post,
    path = "/natural_phenomenon_locations",
    request_body = CreateNaturalPhenomenonLocationRequest,
    responses(
        (status = 201, description = "Location created", body = NaturalPhenomenonLocation),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_location(
    State(service): State<SharedService>, // ← concrete, no <S>
    Extension(user_id): Extension<DatabaseId>,
    Json(req): Json<CreateNaturalPhenomenonLocationRequest>,
) -> Result<Json<NaturalPhenomenonLocation>, (StatusCode, String)> // ← see §2
{
    let domain = NaturalPhenomenonLocation {
        id: None,
        user_id,
        name: req.name,
        latitude: req.latitude,
        longitude: req.longitude,
        description: req.description,
    };

    service
        .create(domain)
        .await
        .map(Json) // Ok side
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR, // Err side
                e.to_string(),
            )
        })
}
#[utoipa::path(
    put,
    path = "/natural_phenomenon_locations/{id}",
    request_body = UpdateNaturalPhenomenonLocationRequest,
    params(
        ("id" = NaturalPhenomenonLocationId, Path, description = "Location ID to update")
    ),
    responses(
        (status = 200, description = "Location updated", body = NaturalPhenomenonLocation),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_location(
    State(service): State<SharedService>,
    Extension(user_id): Extension<DatabaseId>,
    Path(id): Path<NaturalPhenomenonLocationId>,
    Json(payload): Json<UpdateNaturalPhenomenonLocationRequest>, // ← body extractor
) -> Result<Json<NaturalPhenomenonLocation>, (StatusCode, String)> {
    let dto = UpdateNaturalPhenomenonLocationRequestWithIds {
        id,
        user_id,
        payload,
    };

    let updated = service
        .update(dto)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(updated))
}

#[utoipa::path(
    delete,
    path = "/natural_phenomenon_locations/{id}",
    params(
        ("id" = NaturalPhenomenonLocationId, Path, description = "Location ID to update")
    ),
    responses(
        (status = 204, description = "Location deleted"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_location(
    State(service): State<SharedService>,
    Extension(user_id): Extension<DatabaseId>,
    Path(id): Path<NaturalPhenomenonLocationId>,
) -> Result<(), (StatusCode, String)> {
    service.delete(user_id, id).await;
    Ok(())
}
