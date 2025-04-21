use std::str::FromStr;
use serde::{Deserialize, Serialize};
use sqlx::Error::Database;
use tracing_subscriber::registry::Data;
use utoipa::ToSchema;
use crate::config::WapSettings;


#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub settings: WapSettings,
}


#[derive(Clone, sqlx::FromRow, sqlx::Type, Deserialize, Serialize, ToSchema, PartialEq, Debug, Default, Copy)]
#[sqlx(transparent)]   // <<— this tells SQLx “I’m just a wrapper around an INT4”
pub struct DatabaseId (pub i32);

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
