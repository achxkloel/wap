use jsonwebtoken::{encode, Header};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::config::{Config};


// -------------------------------------------------------------------------------------------------
// Structs
// -------------------------------------------------------------------------------------------------

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
#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

// JWT Claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
