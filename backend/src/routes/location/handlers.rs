use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;
use utoipa::ToSchema;

use crate::model::{AppState, CreateLocationRequest, Location, User};
use axum::{extract::{Json, Path, State}, http::StatusCode, response::IntoResponse, Extension};
use std::sync::Arc;

#[utoipa::path(
    get,
    path = "/locations",
    responses(
        (status = 200, description = "All user locations", body = [Location]),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_all_locations(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_id = user.id; // Replace with actual auth logic

    let locations = sqlx::query_as!(
        Location,
        "SELECT * FROM locations WHERE user_id = $1",
        user_id
    )
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
        })?;

    Ok((StatusCode::OK, Json(locations)))
}


#[utoipa::path(
    get,
    path = "/locations/{id}",
    params(
        ("id" = i32, Path, description = "ID of the location to fetch")
    ),
    responses(
        (status = 200, description = "Location found", body = Location),
        (status = 404, description = "Location not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_location_by_id(
    Path(id): Path<i32>,
    Extension(user): Extension<User>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_id = user.id; // Replace with actual auth logic

    let location = sqlx::query_as!(
        Location,
        "SELECT * FROM locations WHERE id = $1 AND user_id = $2",
        id,
        user_id
    )
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
        })?;

    match location {
        Some(loc) => Ok((StatusCode::OK, Json(loc))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Location not found" })),
        )),
    }
}


#[utoipa::path(
    method(post),
    path = "/location",
    request_body(content = CreateLocationRequest, content_type = "application/json"),
    responses(
        (status = axum::http::StatusCode::OK, description = "Success", body = str, content_type = "text/plain"),
        (status = axum::http::StatusCode::BAD_REQUEST, description = "Error", content_type = "text/plain")
    )
)]
pub async fn create_location(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateLocationRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_id = user.id;

    sqlx::query!(
        "INSERT INTO locations (user_id, name, latitude, longitude, description) VALUES ($1, $2, $3, $4, $5)",
        user_id,
        payload.name,
        payload.latitude,
        payload.longitude,
        payload.description
    )
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    Ok((StatusCode::CREATED, "Location created"))
}

#[utoipa::path(
    method(put),
    path = "/location/{id}",
    request_body(content = CreateLocationRequest, content_type = "application/json"),
    responses(
        (status = axum::http::StatusCode::OK, description = "Success", body = str, content_type = "text/plain"),
        (status = axum::http::StatusCode::BAD_REQUEST, description = "Error", content_type = "text/plain")
    )
)]
pub async fn update_location(
    State(state): State<Arc<AppState>>,
    Path(location_id): Path<i32>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateLocationRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_id = user.id;

    sqlx::query!(
        "UPDATE locations SET name = $1, latitude = $2, longitude = $3, description = $4, updated_at = NOW() WHERE id = $5 AND user_id = $6",
        payload.name,
        payload.latitude,
        payload.longitude,
        payload.description,
        location_id,
        user_id
    )
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    Ok((StatusCode::OK, "Location updated"))
}

#[utoipa::path(
    method(delete),
    path = "/location/{id}",
    responses(
        (status = 200, description = "Todo marked done successfully"),
    ),
    params(
        ("id" = i32, Path, description = "Todo database id")
    ),
)]
pub async fn delete_location(
    State(state): State<Arc<AppState>>,
    Path(location_id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_id = 1;

    sqlx::query!(
        "DELETE FROM locations WHERE id = $1 AND user_id = $2",
        location_id,
        user_id
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
    })?;

    Ok((StatusCode::OK, "Location deleted"))
}
