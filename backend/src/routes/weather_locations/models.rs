use crate::shared::models::DatabaseId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct WeatherLocation {
    pub id: DatabaseId,
    pub user_id: DatabaseId,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub is_default: bool,
    pub description: String,

    /// When the row was created
    pub(crate) created_at: chrono::DateTime<chrono::Utc>,

    /// When the row was last updated
    pub(crate) updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateWeatherLocationRequest {
    pub user_id: DatabaseId,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub is_default: bool,
    pub description: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct WeatherLocationCreateRequestSuccess {
    pub id: DatabaseId,
    pub user_id: DatabaseId,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub is_default: bool,
    pub description: String,
    /// When the row was created
    pub(crate) created_at: chrono::DateTime<chrono::Utc>,

    /// When the row was last updated
    pub(crate) updated_at: chrono::DateTime<chrono::Utc>,
}
