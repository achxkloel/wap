use serde::{Deserialize, Serialize};
use sqlx::Type;
use utoipa::ToSchema;
use crate::routes::auth::models::UserId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ToSchema, Serialize, Deserialize, Type)]
pub struct NaturalPhenomenonLocationId(pub i32);

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NaturalPhenomenonLocation {
    pub id: Option<NaturalPhenomenonLocationId>, // None for new entities
    pub user_id: UserId,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateNaturalPhenomenonLocationRequest {
    pub user_id: UserId,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub description: Option<String>,
}
