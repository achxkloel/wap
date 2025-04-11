use crate::config::{Config};
use serde::{Deserialize, Serialize};
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
    pub db: sqlx::PgPool,
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

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema, Clone)]
pub struct Location {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub description: std::option::Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateLocationRequest {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub description: std::option::Option<String>,
}


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
