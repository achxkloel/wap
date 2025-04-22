use crate::shared::models::DatabaseId;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::error::Error;
use std::fmt;
use std::str::FromStr;
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Serialize, Deserialize, ToSchema)]
pub(crate) struct UserDb {
    /// Primary key
    pub(crate) id: DatabaseId,

    /// Unique email address
    pub(crate) email: String,

    /// Argon2 (or bcrypt, etc.) hash of the user's password
    pub(crate) password_hash: String,

    /// Optional first name
    pub(crate) first_name: Option<String>,

    /// Optional last name
    pub(crate) last_name: Option<String>,

    /// Optional URL to the user's avatar/image
    pub(crate) image_url: Option<String>,

    /// Optional Provider
    pub(crate) provider: Option<String>,

    /// Optional Google OAuth ID
    pub(crate) google_id: Option<String>,

    /// When the row was created
    pub(crate) created_at: chrono::DateTime<chrono::Utc>,

    /// When the row was last updated
    pub(crate) updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, ToSchema, Serialize, Deserialize, sqlx::Type, sqlx::FromRow,
)]
pub(crate) struct JwtToken(pub(crate) String);

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct LoginSuccess {
    pub(crate) access_token: String,
    pub(crate) refresh_token: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct LoginError {
    pub(crate) message: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CreateUser {
    pub(crate) email: String,
    pub(crate) password: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct LoginUser {
    pub(crate) email: String,
    pub(crate) password: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct LoginUserSchema {
    pub(crate) email: String,
    pub(crate) password: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct RegisterUserRequestSchema {
    pub(crate) email: String,
    pub(crate) password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub(crate) struct CreateLocationRequest {
    pub(crate) name: String,
    pub(crate) latitude: f64,
    pub(crate) longitude: f64,
    pub(crate) description: std::option::Option<String>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone, ToSchema)]
pub(crate) struct UserRegisterResponse {
    pub(crate) id: DatabaseId,
    pub(crate) email: String,
    pub(crate) created_at: chrono::DateTime<chrono::Utc>,
    pub(crate) updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub(crate) struct RegisterSuccess {
    pub(crate) data: UserRegisterResponse,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub(crate) struct LogoutSuccess {}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub(crate) struct LogoutError {
    pub(crate) message: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub(crate) struct RegisterError {
    pub(crate) message: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone, ToSchema)]
pub(crate) struct LoginResponse {
    pub(crate) access_token: String,
    pub(crate) refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TokenClaims {
    pub(crate) sub: String,
    pub(crate) iat: usize,
    pub(crate) exp: usize,
}

// #[derive(Debug, Serialize, ToSchema)]
// enum AuthResponseKind {
//     login_failed
// }

// #[derive(Debug, Serialize)]
// pub struct AuthError {
//     pub status: &str,
//     pub message: &str,
//     // status_code: i32,
//     // message: String,
//     // kind: AuthResponseKind,
// }

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub(crate) struct AuthError {
    /// A human‑readable message
    pub(crate) message: String,
}

impl AuthError {
    /// Create a new AuthError with the given HTTP status code and message.
    pub(crate) fn new<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            message: message.into(),
        }
    }

    // Shortcut for 401 Unauthorized
    // pub fn unauthorized<M: Into<String>>(msg: M) -> Self {
    //     Self::new(StatusCode::UNAUTHORIZED, msg)
    // }
    //
    // /// Shortcut for 400 Bad Request
    // pub fn bad_request<M: Into<String>>(msg: M) -> Self {
    //     Self::new(StatusCode::BAD_REQUEST, msg)
    // }
}

// So you can do `?` on any AuthError
impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AuthError {}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub(crate) struct RefreshSuccess {
    pub(crate) access_token: String,
}

//-------------------------
// Google Auth2
//-------------------------
#[derive(Debug, Deserialize)]
pub(crate) struct QueryCode {
    pub(crate) code: String,
    pub(crate) state: String,
}

#[derive(Deserialize)]
pub(crate) struct OAuthParams {
    pub(crate) code: String,
    pub(crate) state: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RegisterUserSchema {
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) password: String,
}

/// Schema for Google OAuth register/login requests
#[derive(Debug, Clone)]
pub(crate) struct GoogleAuthRequestSchema {
    pub(crate) id_token: String,
}

/// The minimal data we need from Google to identify the user
#[derive(Debug, Clone)]
pub(crate) struct GoogleTokenData {
    pub(crate) email: String,
}

//--------------------------------------------------------------------------------------------------
// Service
//--------------------------------------------------------------------------------------------------
/// Response from Google's OAuth token endpoint
#[derive(Debug, Deserialize)]
pub(crate) struct TokenResponse {
    pub(crate) access_token: String,
    pub(crate) id_token: String,
    expires_in: Option<u64>,
    token_type: Option<String>,
}

/// Public profile info from Google
#[derive(Debug, Deserialize)]
pub(crate) struct GoogleUser {
    pub(crate) id: String,
    pub(crate) email: String,
    pub(crate) verified_email: bool,
    pub(crate) name: String,
    pub(crate) picture: String,
}
