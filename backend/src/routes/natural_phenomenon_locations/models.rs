use crate::shared::models::DatabaseId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema, Clone, Serialize, PartialEq)]
pub struct UpdateNaturalPhenomenonLocationRequest {
    pub name: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct UpdateNaturalPhenomenonLocationRequestWithIds {
    pub id: DatabaseId,
    pub user_id: DatabaseId,
    pub payload: UpdateNaturalPhenomenonLocationRequest,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema, Clone)]
pub struct NaturalPhenomenonLocationDb {
    pub id: i32,
    pub user_id: DatabaseId,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub description: std::option::Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub struct ServiceCreateAndUpdateResponseSuccess {
    pub id: DatabaseId, // None for new entities
    pub user_id: DatabaseId,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateNaturalPhenomenonLocationRequest {
    pub user_id: DatabaseId,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub description: Option<String>,
}

// Implement display for CreateNaturalPhenomenonLocationRequest
impl std::fmt::Display for CreateNaturalPhenomenonLocationRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CreateNaturalPhenomenonLocationRequest {{ user_id: {:?}, name: {}, latitude: {}, longitude: {}, description: {} }}",
            self.user_id, self.name, self.latitude, self.longitude, self.description.as_deref().unwrap_or("None"))
    }
}
