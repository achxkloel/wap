use std::collections::hash_set::Union;
use jsonwebtoken::{encode, Header};
use crate::config::{Config};
use serde::{Deserialize, Serialize};
use serde::de::Unexpected::Option;
use sqlx::PgPool;
use sqlx::types::time::OffsetDateTime;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterUserRequestSchema {
    pub email: String,
    pub password: String,
}

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub env: Config,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "theme", rename_all = "lowercase")]
pub enum Theme {
    Dark,
    Light,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, sqlx::FromRow)]
pub struct Settings {
    pub theme: Theme,
    pub notifications_enabled: bool,
    pub radius: i32,
}
