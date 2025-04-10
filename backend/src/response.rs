use chrono::{DateTime, Utc};
use crate::config::{Config};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::types::time::OffsetDateTime;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone, ToSchema)]
pub struct UserRegisterResponse {
    pub id: i32,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct RegisterResponse {
    pub data: UserRegisterResponse,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone, ToSchema)]
pub struct LoginResponse {
    pub status: String,
    pub token: String,
}
