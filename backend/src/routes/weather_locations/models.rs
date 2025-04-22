use crate::shared::models::DatabaseId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ToSchema, Serialize, Deserialize)]
pub struct WeatherLocationId(pub i32);

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct WeatherLocation {
    pub id: Option<WeatherLocationId>,
    pub user_id: DatabaseId,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub is_default: bool,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateWeatherLocationRequest {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub is_default: bool,
    pub description: Option<String>,
}
