use crate::config::WapSettings;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use utoipa::ToSchema;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub settings: WapSettings,
}

#[derive(
    Clone,
    sqlx::FromRow,
    sqlx::Type,
    Deserialize,
    Serialize,
    ToSchema,
    PartialEq,
    Debug,
    Default,
    Copy,
    Eq,
)]
#[sqlx(transparent)] // <<— this tells SQLx “I’m just a wrapper around an INT4”
pub struct DatabaseId(pub i32);

impl From<i32> for DatabaseId {
    fn from(x: i32) -> Self {
        DatabaseId(x)
    }
}
impl FromStr for DatabaseId {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s.parse::<i32>()?;
        Ok(DatabaseId(id))
    }
}

#[derive(Debug, Clone)]
pub enum AppStage {
    Development,
    Staging,
    Production,
    Testing,
}
