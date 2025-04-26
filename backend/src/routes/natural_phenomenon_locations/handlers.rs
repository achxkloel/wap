use serde::Serialize;
use std::sync::Arc;

use crate::routes::auth::middlewares::auth;
use crate::routes::auth::models::UserDb;
use crate::routes::auth::services::AuthService;
use crate::routes::natural_phenomenon_locations::models::{
    CreateAndUpdateResponseSuccess, CreateNaturalPhenomenonLocationInnerWithImage,
    CreateNaturalPhenomenonLocationRequest, GetAllNaturalPhenomenonLocationResponseSuccess,
    GetByIdNaturalPhenomenonLocationResponseSuccess, NaturalPhenomenonLocationError,
    NaturalPhenomenonLocationResponseSuccess, PostNaturalPhenomenonLocationSchema,
    PostNaturalPhenomenonLocationService, UpdateNaturalPhenomenonLocationRequest,
    UpdateNaturalPhenomenonLocationRequestWithIds, UpdateNaturalPhenomenonLocationResponseSuccess,
};
use crate::routes::natural_phenomenon_locations::services::{
    NaturalPhenomenonLocationService, NaturalPhenomenonLocationServiceImpl,
};
use crate::shared::models::{AppState, DatabaseId};
use axum::extract::{Extension, Json, Multipart, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tokio::fs;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

/// Fetch all natural phenomenon locations for the current user.
#[utoipa::path(
    get,
    path = "/natural_phenomenon_locations",
    responses(
        (status = 200, description = "All user locations", body = Vec<CreateAndUpdateResponseSuccess>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_all_locations<S>(
    State(service): State<Arc<S>>,
    Extension(user): Extension<UserDb>,
) -> Result<
    Json<Vec<GetAllNaturalPhenomenonLocationResponseSuccess>>,
    (StatusCode, Json<NaturalPhenomenonLocationError>),
>
where
    S: NaturalPhenomenonLocationServiceImpl,
{
    let locations = service.get_all(user.id).await?;
    Ok(Json(locations))
}

/// Fetch a single natural phenomenon location by its ID for the current user.
#[utoipa::path(
    get,
    path = "/natural_phenomenon_locations/{id}",
    params(
        ("id" = DatabaseId, Path, description = "Location ID to retrieve"),
    ),
    responses(
        (status = 200, description = "Location found", body = CreateAndUpdateResponseSuccess),
        (status = 404, description = "Location not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_location_by_id<S>(
    State(service): State<Arc<S>>,
    Extension(user): Extension<UserDb>,
    Path(id): Path<DatabaseId>,
) -> Result<
    Json<GetByIdNaturalPhenomenonLocationResponseSuccess>,
    (StatusCode, Json<NaturalPhenomenonLocationError>),
>
where
    S: NaturalPhenomenonLocationServiceImpl,
{
    let location = service.get_by_id(user.id, id).await?;
    Ok(Json(location))
}

/// Create a new natural phenomenon location for the current user.
#[utoipa::path(
    post,
    path = "/natural_phenomenon_locations",
    request_body(content = PostNaturalPhenomenonLocationSchema, content_type = "multipart/form-data"
    ),
    responses(
        (status = 201, description = "Location created", body = CreateAndUpdateResponseSuccess),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_location<S>(
    State(service): State<Arc<S>>,
    Extension(user): Extension<UserDb>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, Json<NaturalPhenomenonLocationError>)>
where
    S: NaturalPhenomenonLocationServiceImpl,
{
    // 1) pull all fields + image into our DTO
    let mut dto = PostNaturalPhenomenonLocationService {
        user_id: user.id,
        name: String::new(),
        latitude: 0.0,
        longitude: 0.0,
        description: String::new(),
        image_bytes: vec![],
        radius: 0,
        image_filename: String::new(),
    };

    tracing::debug!("processing multipart form data");
    while let Some(mut field) = multipart.next_field().await.map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(NaturalPhenomenonLocationError::DatabaseError(e.to_string())),
        )
    })? {
        let name = field.name().unwrap_or_default();
        println!("name: {}", name);
        match name {
            "name" => {
                dto.name = field.text().await.unwrap_or_default();
            }
            "latitude" => {
                dto.latitude = field
                    .text()
                    .await
                    .unwrap_or_default()
                    .parse::<f64>()
                    .unwrap_or_default();
            }
            "longitude" => {
                dto.longitude = field
                    .text()
                    .await
                    .unwrap_or_default()
                    .parse::<f64>()
                    .unwrap_or_default();
            }
            "description" => {
                dto.description = field.text().await.unwrap_or_default();
            }
            "radius" => {
                dto.radius = field
                    .text()
                    .await
                    .unwrap_or_default()
                    .parse::<i32>()
                    .unwrap_or_default();
            }
            // this branch pulls _both_ the bytes and the filename
            "image" => {
                if let Some(filename) = field.file_name() {
                    dto.image_filename = filename.to_string();
                }
                dto.image_bytes = field
                    .bytes()
                    .await
                    .map_err(|e| {
                        (
                            StatusCode::BAD_REQUEST,
                            Json(NaturalPhenomenonLocationError::DatabaseError(e.to_string())),
                        )
                    })?
                    .to_vec();
            }
            _ => {
                // ignore any unexpected fields
            }
        }
    }

    // 2) hand off to service
    let created = service.create(dto).await?;

    tracing::debug!("created location: {:?}", created);
    Ok((StatusCode::CREATED, Json(created)))
}

/// Update a natural phenomenon location for the current user.
#[utoipa::path(
    put,
    path = "/natural_phenomenon_locations/{id}",
    request_body = UpdateNaturalPhenomenonLocationRequest,
    params(
        ("id" = DatabaseId, Path, description = "Location ID to update")
    ),
    responses(
        (status = 200, description = "Location updated", body = CreateAndUpdateResponseSuccess),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_location<S>(
    State(service): State<Arc<S>>,
    Extension(user): Extension<UserDb>,
    Path(id): Path<DatabaseId>,
    Json(payload): Json<UpdateNaturalPhenomenonLocationRequest>, // ← body extractor
) -> Result<
    Json<UpdateNaturalPhenomenonLocationResponseSuccess>,
    (StatusCode, Json<NaturalPhenomenonLocationError>),
>
where
    S: NaturalPhenomenonLocationServiceImpl,
{
    let dto = UpdateNaturalPhenomenonLocationRequestWithIds {
        id,
        user_id: user.id,
        payload,
    };

    let updated = service.update(dto).await?;

    Ok(Json(updated))
}

/// Delete a natural phenomenon location for the current user.
#[utoipa::path(
    delete,
    path = "/natural_phenomenon_locations/{id}",
    params(
        ("id" = DatabaseId, Path, description = "Location ID to delete")
    ),
    responses(
        (status = 204, description = "Location deleted"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_location<S>(
    State(service): State<Arc<S>>,
    Extension(user): Extension<UserDb>,
    Path(id): Path<DatabaseId>,
) -> Result<impl IntoResponse, (StatusCode, Json<NaturalPhenomenonLocationError>)>
where
    S: NaturalPhenomenonLocationServiceImpl,
{
    // forward straight through — the service already returns the proper Result<Success, Error> tuple
    service.delete(user.id, id).await
}

/// Generic router allowing injection of any implementation of the domain service
pub fn router_with_service<S>(app: AppState, service: Arc<S>) -> OpenApiRouter
where
    S: NaturalPhenomenonLocationServiceImpl + Send + Sync + 'static,
{
    let auth_service = Arc::new(AuthService {
        db: app.db.clone(),
        settings: app.settings.clone(),
        http: Default::default(),
    });
    OpenApiRouter::new()
        .routes(routes!(get_all_locations))
        .routes(routes!(get_location_by_id))
        .routes(routes!(create_location))
        .routes(routes!(update_location))
        .routes(routes!(delete_location))
        .layer(axum::middleware::from_fn_with_state(auth_service, auth))
        .with_state(service)
}

/// Convenience router using the Postgres-backed implementation
pub fn router(app: AppState) -> OpenApiRouter {
    let service = Arc::new(NaturalPhenomenonLocationService::new(app.db.clone()));
    router_with_service(app, service)
}
