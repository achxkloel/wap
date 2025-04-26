use crate::shared::models::DatabaseId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A user’s saved weather‐report location.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct WeatherLocation {
    /// The unique ID of this location record.
    pub id: DatabaseId,

    /// The user to whom this location belongs.
    pub user_id: DatabaseId,

    /// A human‐friendly name for the location (e.g. “Home”, “Grand Canyon”).
    pub name: String,

    /// Latitude coordinate of the location.
    pub latitude: f64,

    /// Longitude coordinate of the location.
    pub longitude: f64,

    /// Whether this is the user’s default location.
    pub is_default: bool,

    /// Optional user‐provided description or notes about this location.
    pub description: String,

    /// Timestamp when this record was first created.
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Timestamp when this record was last updated.
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Payload for creating a new weather location.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateWeatherLocationRequest {
    /// The ID of the user creating this location.
    pub user_id: DatabaseId,

    /// A friendly name for the new location.
    pub name: String,

    /// Latitude coordinate of the new location.
    pub latitude: f64,

    /// Longitude coordinate of the new location.
    pub longitude: f64,

    /// Whether this new location should be marked as the user’s default.
    pub is_default: bool,

    /// Optional user‐provided notes or description.
    pub description: String,
}

/// Response returned after successfully creating a weather location.
#[derive(Debug, Deserialize, ToSchema)]
pub struct WeatherLocationCreateRequestSuccess {
    /// The unique ID of the newly created location.
    pub id: DatabaseId,

    /// The ID of the user who owns this location.
    pub user_id: DatabaseId,

    /// The human‐friendly name of the location.
    pub name: String,

    /// Latitude coordinate of the location.
    pub latitude: f64,

    /// Longitude coordinate of the location.
    pub longitude: f64,

    /// Whether this location is the user’s default.
    pub is_default: bool,

    /// User‐provided description or notes.
    pub description: String,

    /// Timestamp when the new record was created.
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Timestamp when the new record was last updated.
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
