use serde::Serialize;

use axum::extract::{Extension, Json, State};
use std::sync::Arc;

use crate::routes::natural_phenomenon_location::domain::{
    CreateNaturalPhenomenonLocationRequest, NaturalPhenomenonLocation, UserId,
};
use crate::routes::natural_phenomenon_location::service::NaturalPhenomenonLocationService;
use anyhow::Result;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

// #[utoipa::path(
//     get,
//     path = "/natural_phenomenon_locations",
//     responses(
//         (status = 200, description = "All user locations", body = Vec<NaturalPhenomenonLocation>),
//         (status = 500, description = "Internal server error")
//     )
// )]
// pub async fn get_all_locations(
//     State(service): State<Arc<dyn NaturalPhenomenonLocationService>>,
//     Extension(user_id): Extension<UserId>,
// ) -> Result<Json<Vec<NaturalPhenomenonLocation>>> {
//     let locations = service.get_all(user_id).await?;
//     Ok(Json(locations))
// }
//
// #[utoipa::path(
//     get,
//     path = "/natural_phenomenon_locations/{id}",
//     params(
//         ("id" = NaturalPhenomenonLocationId, Path, description = "Location ID to retrieve"),
//     ),
//     responses(
//         (status = 200, description = "Location found", body = NaturalPhenomenonLocation),
//         (status = 404, description = "Location not found"),
//         (status = 500, description = "Internal server error")
//     )
// )]
// pub async fn get_location(
//     State(service): State<Arc<dyn NaturalPhenomenonLocationService>>,
//     Extension(user_id): Extension<UserId>,
//     Path(id): Path<NaturalPhenomenonLocationId>,
// ) -> Result<Json<NaturalPhenomenonLocation>> {
//     let location = service.get_by_id(user_id, id).await?;
//     Ok(Json(location))
// }

#[utoipa::path(
    post,
    path = "/natural_phenomenon_locations",
    request_body = CreateNaturalPhenomenonLocationRequest,
    responses(
        (status = 201, description = "Location created", body = NaturalPhenomenonLocation),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_location<S>(
    State(state): State<Arc<S>>,
    Extension(user_id): Extension<UserId>,
    Json(request): Json<CreateNaturalPhenomenonLocationRequest>,
) -> Result<Json<NaturalPhenomenonLocation>>
where
    S: NaturalPhenomenonLocationService,
{
    let location = NaturalPhenomenonLocation {
        id: None,
        user_id,
        name: request.name,
        latitude: request.latitude,
        longitude: request.longitude,
        description: request.description,
    };

    let created = state.create(location).await?;
    Ok(Json(created))
}

// #[utoipa::path(
//     put,
//     path = "/natural_phenomenon_locations/{id}",
//     request_body = NaturalPhenomenonLocation,
//     params(
//         ("id" = NaturalPhenomenonLocationId, Path, description = "Location ID to update"),
//     ),
//     responses(
//         (status = 200, description = "Location updated", body = NaturalPhenomenonLocation),
//         (status = 500, description = "Internal server error")
//     )
// )]
// pub async fn update_location(
//     State(service): State<Arc<dyn NaturalPhenomenonLocationService>>,
//     Extension(user_id): Extension<UserId>,
//     Path(id): Path<NaturalPhenomenonLocationId>,
// ) -> Result<Json<NaturalPhenomenonLocation>> {
//     let updated = service.update(user_id, id).await?;
//     Ok(Json(updated))
// }
//
// #[utoipa::path(
//     delete,
//     path = "/natural_phenomenon_locations/{id}",
//     responses(
//         (status = 204, description = "Location deleted"),
//         (status = 500, description = "Internal server error")
//     )
// )]
// pub async fn delete_location(
//     State(service): State<Arc<dyn NaturalPhenomenonLocationService>>,
//     Extension(user_id): Extension<UserId>,
//     Path(id): Path<NaturalPhenomenonLocationId>,
// ) -> Result<()> {
//     service.delete(user_id, id).await
// }
