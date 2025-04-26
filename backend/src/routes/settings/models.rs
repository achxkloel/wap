use crate::shared::models::DatabaseId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Available UI themes for the application.
#[derive(Debug, Deserialize, Serialize, ToSchema, sqlx::Type, Copy, Clone, PartialEq)]
#[sqlx(type_name = "theme", rename_all = "lowercase")]
pub enum Theme {
    /// Dark mode theme.
    Dark,
    /// Light mode theme.
    Light,
}

/// Payload for updating a user’s settings.
///
/// Contains all fields that can be modified in one call.
#[derive(Debug, Deserialize, Serialize, ToSchema, sqlx::FromRow, Clone, Copy, PartialEq)]
pub struct UserSettingsUpdateRequest {
    /// The UI theme to apply.
    pub theme: Theme,

    /// Whether the user should receive notifications.
    pub notifications_enabled: bool,

    /// The radius (in kilometers) used for location-based alerts.
    pub radius: i32,
}

/// Representation of a settings row stored in the database.
#[derive(Debug, Deserialize, Serialize, ToSchema, sqlx::FromRow)]
pub struct UserSettingsDb {
    /// Primary key of the settings row.
    pub id: DatabaseId,

    /// The user to whom these settings belong.
    pub user_id: DatabaseId,

    /// The currently selected UI theme.
    pub theme: Theme,

    /// Whether notifications are enabled.
    pub notifications_enabled: bool,

    /// The radius (in kilometers) for geofencing or alerts.
    pub radius: i32,

    /// When this settings record was created.
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// When this settings record was last updated.
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Fields accepted when creating a new settings record.
///
/// All fields are optional; missing values will be filled in with defaults.
#[derive(Debug, Deserialize, Serialize, ToSchema, Default)]
pub struct UserSettingsCreate {
    /// The user ID for which to create settings.
    pub user_id: DatabaseId,

    /// Optional initial theme (defaults to Dark).
    #[serde(default)]
    pub theme: Option<Theme>,

    /// Optional flag for notifications (defaults to true).
    #[serde(default)]
    pub notifications_enabled: Option<bool>,

    /// Optional radius value (defaults to 10).
    #[serde(default)]
    pub radius: Option<i32>,
}

/// DTO returned by the service when fetching existing settings.
///
/// Does not include the database row ID or timestamps.
#[derive(Debug, Deserialize, Serialize, ToSchema, sqlx::FromRow, Clone, Copy, PartialEq)]
pub struct UserSettingsServiceSuccess {
    /// The user to whom these settings belong.
    pub user_id: DatabaseId,

    /// The currently selected UI theme.
    pub theme: Theme,

    /// Whether notifications are enabled.
    pub notifications_enabled: bool,

    /// The radius (in kilometers) configured for the user.
    pub radius: i32,
}
