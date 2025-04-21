use serde::Serialize;
use std::sync::Arc;

use crate::routes::natural_phenomenon_location::models::{
    CreateNaturalPhenomenonLocationRequest, NaturalPhenomenonLocationCreateAndUpdateSuccess,
    UpdateNaturalPhenomenonLocationRequest, UpdateNaturalPhenomenonLocationRequestWithIds,
};
use crate::routes::natural_phenomenon_location::services::NaturalPhenomenonLocationService;
use crate::shared::models::DatabaseId;
use anyhow::Result;
use axum::extract::{Extension, Json, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use utoipa::ToSchema;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

#[utoipa::path(
    get,
    path = "/natural_phenomenon_locations",
    responses(
        (status = 200, description = "All user locations", body = Vec<NaturalPhenomenonLocationCreateAndUpdateSuccess>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_all_locations<S>(
    State(service): State<Arc<S>>,
    Extension(user_id): Extension<DatabaseId>,
) -> Result<Json<Vec<NaturalPhenomenonLocationCreateAndUpdateSuccess>>, (StatusCode, String)>
where
    S: NaturalPhenomenonLocationService,
{
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
        ("id" = DatabaseId, Path, description = "Location ID to retrieve"),
    ),
    responses(
        (status = 200, description = "Location found", body = NaturalPhenomenonLocationCreateAndUpdateSuccess),
        (status = 404, description = "Location not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_location<S>(
    State(service): State<Arc<S>>,
    Extension(user_id): Extension<DatabaseId>,
    Path(id): Path<DatabaseId>,
) -> Result<Json<NaturalPhenomenonLocationCreateAndUpdateSuccess>, (StatusCode, String)>
where
    S: NaturalPhenomenonLocationService,
{
    let location = service
        .get_by_id(user_id, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(location))
}

#[utoipa::path(
    post,
    path = "/natural_phenomenon_locations",
    request_body = NaturalPhenomenonLocationCreateAndUpdateSuccess,
    responses(
        (status = 201, description = "Location created", body = NaturalPhenomenonLocationCreateAndUpdateSuccess),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_location<S>(
    State(service): State<Arc<S>>, // ← concrete, no <S>
    Extension(user_id): Extension<DatabaseId>,
    Json(req): Json<CreateNaturalPhenomenonLocationRequest>,
) -> Result<Json<NaturalPhenomenonLocationCreateAndUpdateSuccess>, (StatusCode, String)>
where
    S: NaturalPhenomenonLocationService,
{
    let domain = CreateNaturalPhenomenonLocationRequest {
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
        ("id" = DatabaseId, Path, description = "Location ID to update")
    ),
    responses(
        (status = 200, description = "Location updated", body = NaturalPhenomenonLocationCreateAndUpdateSuccess),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_location<S>(
    State(service): State<Arc<S>>,
    Extension(user_id): Extension<DatabaseId>,
    Path(id): Path<DatabaseId>,
    Json(payload): Json<UpdateNaturalPhenomenonLocationRequest>, // ← body extractor
) -> Result<Json<NaturalPhenomenonLocationCreateAndUpdateSuccess>, (StatusCode, String)>
where
    S: NaturalPhenomenonLocationService,
{
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
        ("id" = DatabaseId, Path, description = "Location ID to update")
    ),
    responses(
        (status = 204, description = "Location deleted"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_location<S>(
    State(service): State<Arc<S>>,
    Extension(user_id): Extension<DatabaseId>,
    Path(id): Path<DatabaseId>,
) -> Result<impl IntoResponse, (StatusCode, String)>
where
    S: NaturalPhenomenonLocationService,
{
    service
        .delete(user_id, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok((StatusCode::OK, "Location deleted"))
}

// src/routes/natural_phenomenon_location/services/tests.rs

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::routes::natural_phenomenon_location::models::UpdateNaturalPhenomenonLocationRequestWithIds;
//     use crate::routes::natural_phenomenon_location::services::PgNaturalPhenomenonLocationService;
//     use crate::shared::models::DatabaseId;
//     use crate::tests::tests::TestApp;
//     use sqlx::PgPool;
// 
//     #[sqlx::test]
//     #[ignore]
//     async fn test_natural_phenomenon_location_crud(pool: PgPool) {
//         // Arrange: bring up test app to get a valid user
//         let test_app = TestApp::new(pool.clone()).await;
//         let user_id: DatabaseId = test_app.users[0].user.id; // Copy newtype
//         let service = PgNaturalPhenomenonLocationService::new(pool.clone());
// 
//         // 1) CREATE
//         let mut location = NaturalPhenomenonLocationCreateAndUpdateSuccess {
//             id: None,
//             user_id,
//             name: "Volcano".to_string(),
//             latitude: 36.2048,
//             longitude: 138.2529,
//             description: Some("A famous volcano".to_string()),
//         };
//         let created = service
//             .create(location.clone())
//             .await
//             .expect("create failed");
//         // id must be set
//         let id = created.id.expect("id should be returned");
//         assert_eq!(created.user_id, user_id);
//         assert_eq!(created.name, location.name);
//         assert_eq!(created.latitude, location.latitude);
//         assert_eq!(created.longitude, location.longitude);
//         assert_eq!(created.description, location.description);
// 
//         // 2) GET_ALL
//         let all = service.get_all(user_id).await.expect("get_all failed");
//         assert_eq!(all.len(), 1);
//         assert_eq!(all[0].id, Some(id));
// 
//         // 3) GET_BY_ID
//         let fetched = service
//             .get_by_id(user_id, id)
//             .await
//             .expect("get_by_id failed");
//         assert_eq!(fetched.id, Some(id));
//         assert_eq!(fetched.user_id, user_id);
// 
//         // 4) UPDATE
//         let update_req = UpdateNaturalPhenomenonLocationRequestWithIds {
//             user_id,
//             id,
//             payload: crate::routes::natural_phenomenon_location::models::UpdateNaturalPhenomenonLocationRequest {
//                 name: Some("Big Volcano".to_string()),
//                 latitude: Some(36.21),
//                 longitude: Some(138.25),
//                 description: Some("An even more famous volcano".to_string()),
//             },
//         };
//         let updated = service
//             .update(update_req.clone())
//             .await
//             .expect("update failed");
//         assert_eq!(updated.id, Some(id));
//         assert_eq!(updated.user_id, user_id);
//         // assert_eq!(updated.name, update_req.payload.name);
//         // assert_eq!(updated.latitude, update_req.payload.latitude);
//         // assert_eq!(updated.longitude, update_req.payload.longitude);
//         assert_eq!(updated.description, update_req.payload.description);
// 
//         // 5) DELETE
//         service.delete(user_id, id).await.expect("delete failed");
//         let remaining = service
//             .get_all(user_id)
//             .await
//             .expect("get_all after delete failed");
//         assert!(remaining.is_empty());
//     }
// }
