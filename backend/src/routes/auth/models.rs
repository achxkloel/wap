use crate::shared::models::DatabaseId;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Pointer};
use std::str::FromStr;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use mockall::Any;
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Serialize, Deserialize, ToSchema)]
pub struct UserDb {
    /// Primary key
    pub id: DatabaseId,

    /// Unique email address
    pub email: String,

    /// Argon2 (or bcrypt, etc.) hash of the user's password
    pub password_hash: String,

    /// Optional first name
    pub first_name: Option<String>,

    /// Optional last name
    pub last_name: Option<String>,

    /// Optional URL to the user's avatar/image
    pub image_url: Option<String>,

    /// Optional Provider
    pub provider: Option<String>,

    /// Optional Google OAuth ID
    pub google_id: Option<String>,

    /// When the row was created
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// When the row was last updated
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Serialize, Deserialize, ToSchema)]
pub struct UserData {
    /// Primary key
    pub id: DatabaseId,

    /// Unique email address
    pub email: String,

    /// Optional first name
    pub first_name: Option<String>,

    /// Optional last name
    pub last_name: Option<String>,

    /// Optional URL to the user's avatar/image
    pub image_url: Option<String>,

    /// Optional Provider
    pub provider: Option<String>,

    /// Optional Google OAuth ID
    pub google_id: Option<String>,

    /// When the row was created
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// When the row was last updated
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, ToSchema, Serialize, Deserialize, sqlx::Type, sqlx::FromRow,
)]
pub struct JwtToken(pub String);

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct LoginSuccess {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct LoginError {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct RegisterUserRequestSchema {
    pub email: String,
    pub password: String,
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
    pub id: DatabaseId,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct RegisterResponseSuccess {
    pub data: UserRegisterResponse,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct LogoutSuccess {}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct LogoutError {
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct RegisterError {
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone, ToSchema)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
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
pub struct AuthError {
    /// A human‑readable message
    pub message: String,
}

impl AuthError {
    /// Create a new AuthError with the given HTTP status code and message.
    pub fn new<S>(message: S) -> Self
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
pub struct RefreshSuccess {
    pub access_token: String,
    pub refresh_token: String,
}

//-------------------------
// Google Auth2
//-------------------------
#[derive(Debug, Deserialize)]
pub struct QueryCode {
    pub code: String,
}

#[derive(Deserialize)]
pub struct OAuthParams {
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}

/// Schema for Google OAuth register/login requests
#[derive(Debug, Clone)]
pub struct GoogleAuthRequestSchema {
    pub id_token: String,
}

/// The minimal data we need from Google to identify the user
#[derive(Debug, Clone)]
pub struct GoogleTokenData {
    pub email: String,
}

//--------------------------------------------------------------------------------------------------
// Service
//--------------------------------------------------------------------------------------------------
/// Response from Google's OAuth token endpoint
#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub id_token: String,
    expires_in: Option<u64>,
    token_type: Option<String>,
}

/// Public profile info from Google
#[derive(Debug, Deserialize)]
pub struct GoogleUser {
    pub sub: String,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub email: String,
    pub email_verified: bool,
}

#[derive(Debug, ToSchema, PartialEq, Eq)]
pub enum AuthSuccessKind<S> {
    UserCreated(StatusCode, S),
    Created(StatusCode, S),
}


// replace the old single‐type impl with this generic one:
impl<S> IntoResponse for AuthSuccessKind<S>
where
    S: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let (status, body) = match self {
            AuthSuccessKind::UserCreated(s, b) | AuthSuccessKind::Created(s, b) => (s, b),
        };
        let json = serde_json::to_string(&body).unwrap_or_else(|_| {;
            // if we can't serialize the body, just return a generic error
            serde_json::to_string(&AuthError::new("Internal server error - Can not create body")).unwrap()
        });
        let mut response = axum::response::Response::new(json.into());
        *response.status_mut() = status;
        response
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(tag = "type", content = "data")]
pub enum AuthErrorKind {
    UserCreate(String),
    UserAlreadyExists,
    DatabaseError,
    HashingError,
    SettingsCreate,
    MissingCode,
    TokenExchangeError(String),
    GoogleUserFetchError(String),
}

impl Display for AuthErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthErrorKind::UserCreate(msg) => write!(f, "{}", msg),
            AuthErrorKind::UserAlreadyExists => write!(f, "User already exists"),
            AuthErrorKind::DatabaseError => write!(f, "Database error"),
            AuthErrorKind::HashingError => write!(f, "Hashing error"),
            AuthErrorKind::SettingsCreate => write!(f, "Settings error"),
            AuthErrorKind::MissingCode => write!(f, "Missing code"),
            AuthErrorKind::TokenExchangeError(msg) => write!(f, "Token exchange error: {}", msg),
            AuthErrorKind::GoogleUserFetchError(msg) => write!(f, "Google user fetch error: {}", msg),
        }
    }
}

/// Request body for changing a user's password
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ChangePasswordRequest {
    /// The user's current password
    pub current_password: String,
    /// The new password to set
    pub new_password: String,
}
