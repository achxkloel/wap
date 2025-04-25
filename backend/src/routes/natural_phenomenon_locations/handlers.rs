use serde::Serialize;
use std::sync::Arc;

use crate::routes::auth::middlewares::auth;
use crate::routes::auth::models::UserDb;
use crate::routes::auth::services::AuthService;
use crate::routes::natural_phenomenon_locations::models::{CreateAndUpdateResponseSuccess, CreateNaturalPhenomenonLocationInnerWithImage, CreateNaturalPhenomenonLocationRequest, CreateNaturalPhenomenonLocationWithImage, CreateNaturalPhenomenonLocationWithImage2, GetAllNaturalPhenomenonLocationResponseSuccess, GetByIdNaturalPhenomenonLocationResponseSuccess, NaturalPhenomenonLocationErrorKind, NaturalPhenomenonResponseSuccess, UpdateNaturalPhenomenonLocationRequest, UpdateNaturalPhenomenonLocationRequestWithIds, UpdateNaturalPhenomenonLocationResponseSuccess};
use crate::routes::natural_phenomenon_locations::services::{
    NaturalPhenomenonLocationService, PgNaturalPhenomenonLocationService,
};
use crate::shared::models::{AppState, DatabaseId};
use anyhow::Result;
use axum::extract::{Extension, Json, Multipart, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

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
    (StatusCode, Json<NaturalPhenomenonLocationErrorKind>),
>
where
    S: NaturalPhenomenonLocationService,
{
    let locations = service.get_all(user.id).await?;
    Ok(Json(locations))
}


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
) -> Result<Json<GetByIdNaturalPhenomenonLocationResponseSuccess>, (StatusCode, Json<NaturalPhenomenonLocationErrorKind>)>
where
    S: NaturalPhenomenonLocationService,
{
    let location = service
        .get_by_id(user.id, id)
        .await?;
    Ok(Json(location))
}

#[utoipa::path(
    post,
    path = "/natural_phenomenon_locations",
    request_body(content = CreateNaturalPhenomenonLocationWithImage2, content_type = "multipart/form-data"),
    responses(
        (status = 201, description = "Location created", body = CreateAndUpdateResponseSuccess),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_location<S>(
    State(service): State<Arc<S>>,
    Extension(user): Extension<UserDb>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, Json<NaturalPhenomenonLocationErrorKind>)>
where
    S: NaturalPhenomenonLocationService,
{
    // 1) pull all fields + image into our DTO
    let mut dto = CreateNaturalPhenomenonLocationWithImage {
        user_id: user.id,
        name: String::new(),
        latitude: 0.0,
        longitude: 0.0,
        description: String::new(),
        image_bytes: vec![],
        image_filename: String::new(),
    };
    println!("{:?}", dto);

    while let Some(mut field) = multipart
        .next_field()
        .await
        .map_err(|_| {
            (StatusCode::BAD_REQUEST, Json(NaturalPhenomenonLocationErrorKind::DatabaseError))
        })?
    {
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
            // this branch pulls _both_ the bytes and the filename
            "image" => {
                if let Some(filename) = field.file_name() {
                    dto.image_filename = filename.to_string();
                }
                dto.image_bytes = field.bytes().await
                    .map_err(|_| {
                        (StatusCode::BAD_REQUEST, Json(NaturalPhenomenonLocationErrorKind::DatabaseError))
                    })?
                    .to_vec();
            }
            _ => {
                // ignore any unexpected fields
            }
        }
    }

    println!("{:?}", dto);

    // 2) hand off to service
    let created = service.create(dto).await?;

    Ok((StatusCode::CREATED, Json(created)))
}

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
    (StatusCode, Json<NaturalPhenomenonLocationErrorKind>),
>
where
    S: NaturalPhenomenonLocationService,
{
    let dto = UpdateNaturalPhenomenonLocationRequestWithIds {
        id,
        user_id: user.id,
        payload,
    };

    let updated = service
        .update(dto)
        .await?;

    Ok(Json(updated))
}

#[utoipa::path(
    delete,
    path = "/natural_phenomenon_locations/{id}",
    params(
        ("id" = DatabaseId, Path, description = "Location ID to update")
    ),
    request_body(content = CreateNaturalPhenomenonLocationWithImage, content_type = "multipart/form-data") ,
    responses(
        (status = 204, description = "Location deleted", content_type = "application/json"),
        (status = 500, description = "Internal server error", content_type = "application/json")
    )
)]
pub async fn delete_location<S>(
    State(service): State<Arc<S>>,
    Extension(user): Extension<UserDb>,
    Path(id): Path<DatabaseId>,
) -> Result<impl IntoResponse, (StatusCode, Json<NaturalPhenomenonResponseSuccess>)>
where
    S: NaturalPhenomenonLocationService,
{
    service.delete(user.id, id).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(NaturalPhenomenonResponseSuccess {
                message: e.to_string(),
            }),
        )
    })?;
    Ok((
        StatusCode::NO_CONTENT,
        Json(NaturalPhenomenonResponseSuccess {
            message: "Location deleted".to_string(),
        }),
    ))
}

// src/routes/natural_phenomenon_location/services/tests.rs

/// Generic router allowing injection of any implementation of the domain service
pub fn router_with_service<S>(app: AppState, service: Arc<S>) -> OpenApiRouter
where
    S: NaturalPhenomenonLocationService + Send + Sync + 'static,
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
    let service = Arc::new(PgNaturalPhenomenonLocationService::new(app.db.clone()));
    router_with_service(app, service)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routes::natural_phenomenon_locations::services::PgNaturalPhenomenonLocationService;
    use crate::shared::models::DatabaseId;
    use crate::tests::tests::TestApp;
    use sqlx::PgPool;

    #[sqlx::test]
    #[ignore]
    async fn test_natural_phenomenon_location_crud(pool: PgPool) {
        // Arrange: bring up test app to get a valid user
        let test_app = TestApp::new(pool.clone()).await;
        let user_id: DatabaseId = test_app.users[0].user.id; // Copy newtype
        let service = PgNaturalPhenomenonLocationService::new(pool.clone());

        // 1) CREATE
        // let mut location = Box::new(CreateNaturalPhenomenonLocationInnerWithImage {
        //     user_id,
        //     name: "Volcano".to_string(),
        //     latitude: 36.2048,
        //     longitude: 138.2529,
        //     description: "A famous volcano".to_string(),
        //     image_bytes: vec![],
        //     image_filename: "volcano.jpg".to_string(),
        // });
        // let created = service
        //     .create(location.into())
        //     .await
        //     .expect(&format!("create {location} failed"));
        // // id must be set
        // // let id = created.id.expect("id should be returned");
        // assert_eq!(created.user_id, user_id);
        // assert_eq!(created.name, location.name);
        // assert_eq!(created.latitude, location.latitude);
        // assert_eq!(created.longitude, location.longitude);
        // assert_eq!(created.description, location.description);

        // // 2) GET_ALL
        // let all = service.get_all(user_id).await.expect("get_all failed");
        // assert_eq!(all.len(), 1);
        // assert_eq!(all[0].id, Some(id));
        //
        // // 3) GET_BY_ID
        // let fetched = service
        //     .get_by_id(user_id, id)
        //     .await
        //     .expect("get_by_id failed");
        // assert_eq!(fetched.id, Some(id));
        // assert_eq!(fetched.user_id, user_id);
        //
        // // 4) UPDATE
        // let update_req = UpdateNaturalPhenomenonLocationRequestWithIds {
        //     user_id,
        //     id,
        //     payload: crate::routes::natural_phenomenon_location::models::UpdateNaturalPhenomenonLocationRequest {
        //         name: Some("Big Volcano".to_string()),
        //         latitude: Some(36.21),
        //         longitude: Some(138.25),
        //         description: Some("An even more famous volcano".to_string()),
        //     },
        // };
        // let updated = service
        //     .update(update_req.clone())
        //     .await
        //     .expect("update failed");
        // assert_eq!(updated.id, Some(id));
        // assert_eq!(updated.user_id, user_id);
        // // assert_eq!(updated.name, update_req.payload.name);
        // // assert_eq!(updated.latitude, update_req.payload.latitude);
        // // assert_eq!(updated.longitude, update_req.payload.longitude);
        // assert_eq!(updated.description, update_req.payload.description);
        //
        // // 5) DELETE
        // service.delete(user_id, id).await.expect("delete failed");
        // let remaining = service
        //     .get_all(user_id)
        //     .await
        //     .expect("get_all after delete failed");
        // assert!(remaining.is_empty());
    }
}
