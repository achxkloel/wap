use crate::shared::models::DatabaseId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// pub type SharedState = std::sync::Arc<AppState>;
#[derive(Debug, Deserialize, Serialize, ToSchema, sqlx::Type, Copy, Clone, PartialEq)]
#[sqlx(type_name = "theme", rename_all = "lowercase")]
pub enum Theme {
    Dark,
    Light,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, sqlx::FromRow, Clone, Copy, PartialEq)]
pub struct UserSettingsUpdateRequest {
    pub theme: Theme,
    pub notifications_enabled: bool,
    pub radius: i32,
}

#[derive(Deserialize, Serialize, ToSchema, sqlx::FromRow)]
pub struct UserSettingsDb {
    pub id: DatabaseId,
    pub user_id: DatabaseId,
    pub theme: Theme,
    pub notifications_enabled: bool,
    pub radius: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Default)]
pub struct UserSettingsCreate {
    pub user_id: DatabaseId,
    #[serde(default)] // makes JSON deserializing optional too
    pub theme: Option<Theme>,
    #[serde(default)]
    pub notifications_enabled: Option<bool>,
    #[serde(default)]
    pub radius: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, sqlx::FromRow, Clone, Copy, PartialEq)]
pub struct UserSettingsServiceSuccess {
    pub user_id: DatabaseId,
    pub theme: Theme,
    pub notifications_enabled: bool,
    pub radius: i32,
}
