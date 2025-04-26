use crate::routes::natural_phenomenon_locations::models::{
    CreateAndUpdateResponseSuccess, CreateNaturalPhenomenonLocationInnerWithImage,
    CreateNaturalPhenomenonLocationRequest, GetAllNaturalPhenomenonLocationResponseSuccess,
    GetByIdNaturalPhenomenonLocationResponseSuccess, NaturalPhenomenonLocationDb,
    NaturalPhenomenonLocationError, NaturalPhenomenonLocationResponseSuccess,
    PostNaturalPhenomenonLocationService, ServiceCreateAndUpdateResponseSuccess,
    UpdateNaturalPhenomenonLocationRequestWithIds, UpdateNaturalPhenomenonLocationResponseSuccess,
};
use crate::shared::models::DatabaseId;
use anyhow::Result;
use async_trait::async_trait;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use sanitize_filename::sanitize_with_options;
use sqlx::PgPool;
use tokio::fs;
use tracing::debug;
use uuid::Uuid;

/// Core service trait defining CRUD operations for natural phenomenon locations,
/// including file-based image storage cleanup on delete.
#[async_trait]
pub trait NaturalPhenomenonLocationServiceImpl: Send + Sync + 'static {
    /// Create a new natural phenomenon location with optional image upload.
    ///
    /// The `req` contains both metadata and the raw image bytes.
    /// Returns the created record on success, or an HTTP‐style error tuple on failure.
    async fn create(
        &self,
        req: PostNaturalPhenomenonLocationService,
    ) -> Result<CreateAndUpdateResponseSuccess, (StatusCode, Json<NaturalPhenomenonLocationError>)>;

    /// Retrieve all phenomenon locations belonging to the given `user_id`.
    ///
    /// Returns a vector of response DTOs or an HTTP‐style error on failure.
    async fn get_all(
        &self,
        user_id: DatabaseId,
    ) -> Result<
        Vec<GetAllNaturalPhenomenonLocationResponseSuccess>,
        (StatusCode, Json<NaturalPhenomenonLocationError>),
    >;

    /// Fetch a single location by its `id` for the specified `user_id`.
    ///
    /// Returns the matching DTO or an HTTP‐style error if not found or on DB error.
    async fn get_by_id(
        &self,
        user_id: DatabaseId,
        id: DatabaseId,
    ) -> Result<
        GetByIdNaturalPhenomenonLocationResponseSuccess,
        (StatusCode, Json<NaturalPhenomenonLocationError>),
    >;

    /// Update an existing location’s fields (name, coords, radius, description).
    ///
    /// Only non-`None` fields in the DTO will be overwritten.
    /// Returns the updated DTO or an HTTP‐style error.
    async fn update(
        &self,
        location: UpdateNaturalPhenomenonLocationRequestWithIds,
    ) -> Result<
        UpdateNaturalPhenomenonLocationResponseSuccess,
        (StatusCode, Json<NaturalPhenomenonLocationError>),
    >;

    /// Delete the record and its on-disk image (if any).
    ///
    /// Returns a `(204, Deleted)` response on success, or an HTTP‐style error tuple.
    async fn delete(
        &self,
        user_id: DatabaseId,
        id: DatabaseId,
    ) -> Result<
        (StatusCode, Json<NaturalPhenomenonLocationResponseSuccess>),
        (StatusCode, Json<NaturalPhenomenonLocationError>),
    >;
}

/// Postgres-backed implementation of the `NaturalPhenomenonLocationServiceImpl` trait.
///
/// Handles file-system writes for images, transactional DB operations,
/// and cleanup of image files upon deletion.
pub struct NaturalPhenomenonLocationService {
    /// SQLx Postgres connection pool.
    pub db: PgPool,
}

impl NaturalPhenomenonLocationService {
    /// Construct a new service using the given Postgres pool.
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}
#[async_trait]
impl NaturalPhenomenonLocationServiceImpl for NaturalPhenomenonLocationService {
    async fn create(
        &self,
        req: PostNaturalPhenomenonLocationService,
    ) -> Result<CreateAndUpdateResponseSuccess, (StatusCode, Json<NaturalPhenomenonLocationError>)>
    {
        debug!("\n|| creating location: {:?}", req);

        // make sure uploads/ exists
        fs::create_dir_all("uploads").await.map_err(|e| {
            tracing::error!("\n|| Error creating uploads/ dir: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(NaturalPhenomenonLocationError::DatabaseError(e.to_string())),
            )
        })?;

        // if there's image data, write it and record a path; otherwise leave it None
        debug!("\n|| req.image_bytes.len(): {}", req.image_bytes.len());
        let image_path_opt = if !req.image_bytes.is_empty() {
            let safe = sanitize_with_options(&req.image_filename, Default::default());
            let filename = format!("{}_{}", Uuid::new_v4(), safe);
            let path = format!("uploads/{}", filename);

            fs::write(&path, &req.image_bytes).await.map_err(|e| {
                tracing::error!("\n|| Error writing image to disk: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(NaturalPhenomenonLocationError::DatabaseError(e.to_string())),
                )
            })?;
            debug!("\n|| wrote image to disk at {}", path);
            Some(path)
        } else {
            debug!("\n|| image not present");
            None
        };
        debug!("\n|| image_path_opt: {:?}", image_path_opt);
        // let db_image_path = image_path_opt.unwrap_or_default();
        // tracing::debug!("\ndb_image_path: {:?}", db_image_path);

        // 2) insert into DB, passing `image_path_opt` which is NULL if no bytes
        let rec = sqlx::query_as!(
            NaturalPhenomenonLocationDb,
            r#"
            INSERT INTO natural_phenomenon_locations
                (user_id, name, latitude, longitude, description, image_path, radius)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
            req.user_id.0,
            req.name,
            req.latitude,
            req.longitude,
            req.description,
            image_path_opt,
            // db_image_path,
            req.radius,
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| {
            tracing::error!("\n|| DB insert error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(NaturalPhenomenonLocationError::DatabaseError(e.to_string())),
            )
        })?;

        tracing::debug!("\n|| DB row created: {:?}", rec);

        // 3) build and return your DTO, using rec.image_path (Option<String>) directly
        Ok(CreateAndUpdateResponseSuccess {
            id: rec.id,
            user_id: rec.user_id,
            name: rec.name,
            latitude: rec.latitude,
            longitude: rec.longitude,
            description: rec.description,
            image_path: rec.image_path.unwrap_or_default(),
            radius: rec.radius,
            created_at: rec.created_at,
            updated_at: rec.updated_at,
        })
    }

    async fn get_all(
        &self,
        user_id: DatabaseId,
    ) -> Result<
        Vec<GetAllNaturalPhenomenonLocationResponseSuccess>,
        (StatusCode, Json<NaturalPhenomenonLocationError>),
    > {
        // 1) try to fetch all rows, and map any SQL error into our tuple
        let records = sqlx::query_as!(
            NaturalPhenomenonLocationDb,
            r#"
                SELECT *
                FROM natural_phenomenon_locations
                WHERE user_id = $1
            "#,
            user_id.0
        )
        .fetch_all(&self.db)
        .await
        .map_err(|e| {
            tracing::error!("\nError fetching locations: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(NaturalPhenomenonLocationError::DatabaseError(e.to_string())),
            )
        })?;

        // 2) now map the Vec<NaturalPhenomenonLocationDb> into our response DTOs
        let locations = records
            .into_iter()
            .map(|rec| GetAllNaturalPhenomenonLocationResponseSuccess {
                id: rec.id,
                user_id: rec.user_id,
                name: rec.name,
                latitude: rec.latitude,
                longitude: rec.longitude,
                description: rec.description,
                radius: rec.radius,
                image_path: rec.image_path.unwrap_or_default(),
            })
            .collect();

        Ok(locations)
    }

    async fn get_by_id(
        &self,
        user_id: DatabaseId,
        id: DatabaseId,
    ) -> Result<
        GetByIdNaturalPhenomenonLocationResponseSuccess,
        (StatusCode, Json<NaturalPhenomenonLocationError>),
    > {
        let rec = sqlx::query_as!(
            NaturalPhenomenonLocationDb,
            r#"
                SELECT *
                FROM natural_phenomenon_locations
                WHERE id = $1 AND user_id = $2
                "#,
            id.0,
            user_id.0
        )
        .fetch_one(&self.db)
        .await
        .map_err(|_| {
            tracing::error!("\nError fetching location by ID: {:?}", id);
            (
                StatusCode::NOT_FOUND,
                Json(NaturalPhenomenonLocationError::NotFound),
            )
        })?;

        Ok(GetByIdNaturalPhenomenonLocationResponseSuccess {
            id: rec.id,
            user_id: rec.user_id,
            name: rec.name,
            latitude: rec.latitude,
            longitude: rec.longitude,
            radius: rec.radius,
            description: rec.description,
            image_path: rec.image_path.unwrap_or_default(),
        })
    }

    async fn update(
        &self,
        location: UpdateNaturalPhenomenonLocationRequestWithIds,
    ) -> Result<
        UpdateNaturalPhenomenonLocationResponseSuccess,
        (StatusCode, Json<NaturalPhenomenonLocationError>),
    > {
        let record = sqlx::query_as!(
            NaturalPhenomenonLocationDb,
            r#"
        UPDATE natural_phenomenon_locations
        SET
            name        = COALESCE($1, name),
            latitude    = COALESCE($2, latitude),
            longitude   = COALESCE($3, longitude),
            radius      = COALESCE($4, radius),
            description = COALESCE($5, description)
        WHERE id = $6 AND user_id = $7
        RETURNING *
        "#,
            // these are Option<...>, so COALESCE will pick the existing value when None:
            location.payload.name,
            location.payload.latitude,
            location.payload.longitude,
            location.payload.radius,
            location.payload.description,
            location.id.0,
            location.user_id.0,
        )
        .fetch_one(&self.db)
        .await
        .map_err(|_| {
            tracing::error!("\nError updating location: {:?}", location);
            (
                StatusCode::NOT_FOUND,
                Json(NaturalPhenomenonLocationError::NotFound),
            )
        })?;

        Ok(UpdateNaturalPhenomenonLocationResponseSuccess {
            id: record.id,
            user_id: record.user_id,
            name: record.name,
            latitude: record.latitude,
            longitude: record.longitude,
            description: record.description,
            image_path: record.image_path.unwrap_or_default(),
            radius: record.radius,
        })
    }

    async fn delete(
        &self,
        user_id: DatabaseId,
        id: DatabaseId,
    ) -> Result<
        (StatusCode, Json<NaturalPhenomenonLocationResponseSuccess>),
        (StatusCode, Json<NaturalPhenomenonLocationError>),
    > {
        // 1) Delete the DB row, grabbing the image_path
        let rec = sqlx::query!(
            r#"
            DELETE FROM natural_phenomenon_locations
             WHERE id = $1 AND user_id = $2
            RETURNING image_path
            "#,
            id.0,
            user_id.0,
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| {
            tracing::error!("\nDB delete error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(NaturalPhenomenonLocationError::DatabaseError(e.to_string())),
            )
        })?;

        // 2) If there was an image_path, remove the file (ignore FS errors)
        if let Some(path) = rec.image_path {
            tracing::debug!("\nremoving image file at {}", path);
            let _ = fs::remove_file(&path).await;
        } else {
            tracing::debug!("\nno image file to remove");
        }

        // 3) Success → return a 204 + our Deleted enum
        Ok((
            StatusCode::NO_CONTENT,
            Json(NaturalPhenomenonLocationResponseSuccess::Deleted),
        ))
    }
}
