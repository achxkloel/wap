use crate::routes::natural_phenomenon_location::{NaturalPhenomenonLocationId, NaturalPhenomenonLocationService, UserId};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateNaturalPhenomenonLocationRequest {
    pub name:        Option<String>,
    pub latitude:    Option<f64>,
    pub longitude:   Option<f64>,
    pub description: Option<String>,
}

pub struct UpdateNaturalPhenomenonLocationRequestWithIds {
    pub id:          NaturalPhenomenonLocationId,
    pub user_id:     UserId,
    pub payload:     UpdateNaturalPhenomenonLocationRequest,
}

pub type SharedService = std::sync::Arc<dyn NaturalPhenomenonLocationService + Send + Sync>;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema, Clone)]
pub struct NaturalPhenomenonLocation {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub description: std::option::Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
