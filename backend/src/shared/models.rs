use crate::config::WapSettings;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub settings: WapSettings,
}

