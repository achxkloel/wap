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

// Application state holding the DB pool and JWT secret
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub env: Config,
}

// User signup request
#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
}

// User login request
#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

// JWT Claims structure
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Claims {
//     pub sub: String,
//     pub exp: usize,
// }
