use crate::config::Config;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub env: Config,
}

