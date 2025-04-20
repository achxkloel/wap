use serde::Deserialize;
use utoipa::ToSchema;
use crate::routes::natural_phenomenon_location::{NaturalPhenomenonLocationId, UserId};

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

