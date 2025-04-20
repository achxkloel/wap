use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::routes::natural_phenomenon_location::NaturalPhenomenonLocationService;
use crate::routes::weather_location::WeatherLocationService;

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
    pub theme: Theme,
    pub notifications_enabled: bool,
    pub radius: i32,
}
