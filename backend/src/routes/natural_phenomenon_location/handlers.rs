use serde::Serialize;

use axum::extract::{Extension, Json, State};
use std::sync::Arc;

use crate::routes::natural_phenomenon_location::domain::{
    CreateNaturalPhenomenonLocationRequest, NaturalPhenomenonLocation, UserId,
};
use crate::routes::natural_phenomenon_location::service::{NaturalPhenomenonLocationService, SharedService};
use anyhow::Result;
use axum::http::StatusCode;

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
pub async fn create_location(
    State(service): State<SharedService>,          // ← concrete, no <S>
    Extension(user_id): Extension<UserId>,
    Json(req): Json<CreateNaturalPhenomenonLocationRequest>,
) -> Result<Json<NaturalPhenomenonLocation>, (StatusCode, String)>  // ← see §2
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
        .map(Json)                                        // Ok side
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR,  // Err side
                      e.to_string()))
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
