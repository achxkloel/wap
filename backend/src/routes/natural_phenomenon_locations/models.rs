use crate::shared::models::DatabaseId;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use utoipa::ToSchema;

/// Database representation of a natural phenomenon location.
///
/// Contains all persisted fields, including optional image path
/// and geofence radius.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema, Clone, PartialEq)]
pub struct NaturalPhenomenonLocationDb {
    /// Primary key of the location record.
    pub id: DatabaseId,

    /// ID of the user who owns this location.
    pub user_id: DatabaseId,

    /// Human‐readable name for the phenomenon (e.g., “Grand Canyon”).
    pub name: String,

    /// Latitude coordinate of the phenomenon.
    pub latitude: f64,

    /// Longitude coordinate of the phenomenon.
    pub longitude: f64,

    /// Optional local filesystem path (or URL) to an associated image.
    pub image_path: Option<String>,

    /// Alert radius, in kilometers.
    pub radius: i32,

    /// User‐provided description or notes about this location.
    pub description: String,

    /// Timestamp when the row was created.
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Timestamp when the row was last updated.
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Request payload for updating an existing location.
///
/// Any field set to `None` will not be changed.
#[derive(Debug, Deserialize, ToSchema, Clone, Serialize, PartialEq)]
pub struct UpdateNaturalPhenomenonLocationRequest {
    /// New name, if updating.
    pub name: Option<String>,

    /// New latitude, if updating.
    pub latitude: Option<f64>,

    /// New longitude, if updating.
    pub longitude: Option<f64>,

    /// New alert radius, if updating.
    pub radius: Option<i32>,

    /// New description, if updating.
    pub description: Option<String>,
}

/// Wrapper combining path parameters and update payload.
///
/// Carries the `id` and `user_id` along with the fields to change.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct UpdateNaturalPhenomenonLocationRequestWithIds {
    /// ID of the location to update.
    pub id: DatabaseId,

    /// ID of the user who owns the location.
    pub user_id: DatabaseId,

    /// The optional‐field payload.
    pub payload: UpdateNaturalPhenomenonLocationRequest,
}

/// Response DTO for both create and update operations.
///
/// Mirrors the DB model but normalizes `Option<String>` → `String` for image_path.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub struct CreateAndUpdateResponseSuccess {
    /// ID of the created or updated location.
    pub id: DatabaseId,

    /// ID of the owning user.
    pub user_id: DatabaseId,

    /// Location name.
    pub name: String,

    /// Location latitude.
    pub latitude: f64,

    /// Location longitude.
    pub longitude: f64,

    /// Alert radius in kilometers.
    pub radius: i32,

    /// User-provided description.
    pub description: String,

    /// Stored image path or empty string if none.
    pub image_path: String,

    /// Record creation timestamp.
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Record last‐updated timestamp.
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Response DTO when listing all locations.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub struct GetAllNaturalPhenomenonLocationResponseSuccess {
    /// Location ID.
    pub id: DatabaseId,

    /// Owning user ID.
    pub user_id: DatabaseId,

    /// Location name.
    pub name: String,

    /// Latitude.
    pub latitude: f64,

    /// Longitude.
    pub longitude: f64,

    /// Radius in kilometers.
    pub radius: i32,

    /// Description.
    pub description: String,

    /// Image path or empty string.
    pub image_path: String,
}

/// Response DTO when fetching a single location by ID.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub struct GetByIdNaturalPhenomenonLocationResponseSuccess {
    /// Location ID.
    pub id: DatabaseId,

    /// Owning user ID.
    pub user_id: DatabaseId,

    /// Location name.
    pub name: String,

    /// Latitude.
    pub latitude: f64,

    /// Longitude.
    pub longitude: f64,

    /// Radius.
    pub radius: i32,

    /// Description.
    pub description: String,

    /// Image path or empty string.
    pub image_path: String,
}

/// Response DTO after a successful update.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub struct UpdateNaturalPhenomenonLocationResponseSuccess {
    /// Location ID.
    pub id: DatabaseId,

    /// Owning user ID.
    pub user_id: DatabaseId,

    /// Location name.
    pub name: String,

    /// Latitude.
    pub latitude: f64,

    /// Longitude.
    pub longitude: f64,

    /// Radius.
    pub radius: i32,

    /// Description.
    pub description: String,

    /// Image path or empty string.
    pub image_path: String,
}

/// Payload for creating a location via JSON.
///
/// All fields required except `image_path` which holds a pre-uploaded URL or path.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateNaturalPhenomenonLocationRequest {
    /// Owning user ID.
    pub user_id: DatabaseId,

    /// Location name.
    pub name: String,

    /// Latitude.
    pub latitude: f64,

    /// Longitude.
    pub longitude: f64,

    /// Alert radius.
    pub radius: i32,

    /// Pre‐existing image URL or path.
    pub image_path: String,

    /// Description text.
    pub description: String,
}

impl Display for CreateNaturalPhenomenonLocationRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CreateNaturalPhenomenonLocationRequest {{ user_id: {:?}, name: {}, latitude: {}, longitude: {}, description: {} }}",
            self.user_id, self.name, self.latitude, self.longitude, self.description
        )
    }
}

/// Internal service payload including raw image bytes and filename.
#[derive(Debug, ToSchema, Deserialize, Serialize, Clone)]
pub struct PostNaturalPhenomenonLocationService {
    /// Owning user ID.
    pub user_id: DatabaseId,

    /// Location name.
    pub name: String,

    /// Latitude.
    pub latitude: f64,

    /// Longitude.
    pub longitude: f64,

    /// Description.
    pub description: String,

    /// Alert radius.
    pub radius: i32,

    /// Raw image bytes for upload.
    #[schema(format = Binary, content_media_type = "application/octet-stream")]
    #[serde(with = "serde_bytes")]
    pub image_bytes: Vec<u8>,

    /// Original filename (for extension/preservation).
    #[schema(example = "photo.jpg")]
    pub image_filename: String,
}

impl Display for PostNaturalPhenomenonLocationService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PostNaturalPhenomenonLocationService {{ user_id: {:?}, name: {}, latitude: {}, longitude: {}, description: {} }}",
            self.user_id, self.name, self.latitude, self.longitude, self.description
        )
    }
}

/// OpenAPI schema for multipart‐form create requests.
///
/// Only the raw image bytes under the field "image" are accepted here.
#[derive(Debug, ToSchema, Deserialize, Serialize)]
pub struct PostNaturalPhenomenonLocationSchema {
    /// Owning user ID.
    pub user_id: DatabaseId,

    /// Location name.
    pub name: String,

    /// Latitude.
    pub latitude: f64,

    /// Longitude.
    pub longitude: f64,

    /// Description.
    pub description: String,

    /// Alert radius.
    pub radius: i32,

    /// Raw image bytes field for multipart/form-data.
    #[schema(format = Binary, content_media_type = "application/octet-stream")]
    #[serde(with = "serde_bytes")]
    pub image: Vec<u8>,
}

/// Inner struct for create operations with separate image bytes & filename.
#[derive(Debug, ToSchema, Deserialize, Serialize)]
pub struct CreateNaturalPhenomenonLocationInnerWithImage {
    /// Owning user ID.
    pub user_id: DatabaseId,

    /// Location name.
    pub name: String,

    /// Latitude.
    pub latitude: f64,

    /// Longitude.
    pub longitude: f64,

    /// Description.
    pub description: String,

    /// Raw image bytes.
    pub image_bytes: Vec<u8>,

    /// Original filename.
    pub image_filename: String,
}

/// Success response variants for deletion.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "data")]
pub enum NaturalPhenomenonLocationResponseSuccess {
    /// Indicates the record was successfully deleted.
    Deleted,
}

impl Display for NaturalPhenomenonLocationResponseSuccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NaturalPhenomenonLocationResponseSuccess::Deleted => write!(f, "Deleted"),
        }
    }
}

/// Error variants for location operations.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "data")]
pub enum NaturalPhenomenonLocationError {
    /// The requested location was not found.
    NotFound,

    /// A location with the same key already exists.
    AlreadyExists,

    /// A database error occurred; contains the error message.
    DatabaseError(String),

    /// The record could not be deleted.
    LocationCouldNotBeDeleted,

    /// The associated image could not be removed from storage.
    ImageInLocationCouldNotBeDeleted,
}

impl Display for NaturalPhenomenonLocationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NaturalPhenomenonLocationError::NotFound => write!(f, "Not Found"),
            NaturalPhenomenonLocationError::AlreadyExists => write!(f, "Already Exists"),
            NaturalPhenomenonLocationError::DatabaseError(err) => {
                write!(f, "Database Error: {}", err)
            }
            NaturalPhenomenonLocationError::LocationCouldNotBeDeleted => {
                write!(f, "Location Could Not Be Deleted")
            }
            NaturalPhenomenonLocationError::ImageInLocationCouldNotBeDeleted => {
                write!(f, "Image Could Not Be Deleted")
            }
        }
    }
}

/// Shared DTO for create/update responses without timestamps.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub struct ServiceCreateAndUpdateResponseSuccess {
    /// Location ID.
    pub id: DatabaseId,

    /// Owning user ID.
    pub user_id: DatabaseId,

    /// Location name.
    pub name: String,

    /// Latitude coordinate.
    pub latitude: f64,

    /// Longitude coordinate.
    pub longitude: f64,

    /// Alert radius.
    pub radius: i32,

    /// User description.
    pub description: String,
}
