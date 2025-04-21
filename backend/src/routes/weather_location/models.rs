use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::routes::weather_location::UserId;

// pub type SharedState = std::sync::Arc<AppState>;
#[derive(Debug, Deserialize, Serialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "theme", rename_all = "lowercase")]
pub enum Theme {
    Dark,
    Light,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, sqlx::FromRow)]
pub struct UserSettingsUpdateRequest {
    pub theme: Theme,
    pub notifications_enabled: bool,
    pub radius: i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, sqlx::FromRow)]
pub struct UserSettings {
    pub id: i32,
    pub user_id: i32,
    pub theme: Theme,
    pub notifications_enabled: bool,
    pub radius: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, sqlx::FromRow)]
pub struct UserSettingsServiceSuccess {
    pub user_id: UserId,
    pub theme: Theme,
    pub notifications_enabled: bool,
    pub radius: i32,
}
