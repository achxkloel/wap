use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use bytes::Bytes;
use mime_guess::MimeGuess;
use std::sync::Arc;
use axum::http::{header, HeaderValue};
use tokio::fs;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use crate::routes::auth::middlewares::auth;
use crate::routes::auth::services::AuthService;
use crate::routes::uploads::models::UploadError;
use crate::routes::uploads::services::{UploadsService, UploadsServiceImpl};
use crate::shared::models::AppState;

#[utoipa::path(
    get,
    path = "/uploads",
    responses(
        (status = 200, description = "All user locations"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_photos<S>(
    State(service): State<Arc<S>>,
) -> Result<impl IntoResponse, (StatusCode, Json<UploadError>)>
where
    S: UploadsServiceImpl + Send + Sync + 'static,
{
    match service.list_photos().await {
        Ok(photos) => Ok((StatusCode::OK, Json(photos))),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(UploadError::NotFound))),
    }
}

#[utoipa::path(
    get,
    path = "/uploads/{filename}",
    responses(
        (status = 200, description = "All user locations"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_photo(
    Path(filename): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<UploadError>)> {
    let path = format!("uploads/{}", filename);

    // Try to read the file from disk
    let data = fs::read(&path)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, Json(UploadError::NotFound)))?;

    // Guess a MIME type from the extension
    let mime = MimeGuess::from_path(&path)
        .first_or_octet_stream()
        .to_string();

    // Return a (status, headers, body) tuple
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, mime)],
        Bytes::from(data),
    ))
}

/// Generic router allowing injection of any implementation of the domain service
pub fn router_with_service<S>(app: AppState, service: Arc<S>) -> OpenApiRouter
where
    S: UploadsServiceImpl + Send + Sync + 'static,
{
    let auth_service = Arc::new(AuthService {
        db: app.db.clone(),
        settings: app.settings.clone(),
        http: Default::default(),
    });
    OpenApiRouter::new()
        .routes(routes!(get_photo))
        .routes(routes!(list_photos))
        .layer(axum::middleware::from_fn_with_state(auth_service, auth))
        .with_state(service)
}

/// Convenience router using the Postgres-backed implementation
pub fn router(app: AppState) -> OpenApiRouter {
    let uploads_service = Arc::new(UploadsService {
        directory: "uploads".into(),
        url_prefix: "/uploads".to_string(),
    });
    router_with_service(app, uploads_service)
}

