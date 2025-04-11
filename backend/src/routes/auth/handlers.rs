// use std::sync::Arc;
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{
    extract::{Json, State},
    http::{header, Response, StatusCode},
    response::IntoResponse,
    routing::post,
};
use serde_json::json;

use axum_extra::extract::cookie::{Cookie, SameSite};
use bcrypt::{hash, verify, DEFAULT_COST};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;
use utoipa_axum::{router::OpenApiRouter, routes, PathItemExt};

use crate::model::{Theme, RegisterUserRequestSchema, AppState, CreateUser, LoginUser, LoginUserSchema, TokenClaims, User, LoginResponse, RegisterResponse, UserRegisterResponse};

// -------------------------------------------------------------------------------------------------
// Routes
// -------------------------------------------------------------------------------------------------
// SIGNUP handler

// use axum::{Json, response::IntoResponse};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct SignupResponse {
    pub message: String,
}

#[utoipa::path(
    method(post),
    path = "/auth/register",
    request_body(content = RegisterUserRequestSchema, content_type = "application/json"),
    responses(
        (status = axum::http::StatusCode::OK, description = "Success", body = RegisterResponse, content_type = "text/plain"),
        (status = axum::http::StatusCode::BAD_REQUEST, description = "Error", content_type = "text/plain")
    )
)]
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterUserRequestSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Error while hashing password: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })
        .map(|hash| hash.to_string())?;

    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (email,password) VALUES ($1, $2) RETURNING id, email, password, created_at, updated_at",
        body.email.to_string().to_ascii_lowercase(),
        hashed_password
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
        "user": filter_user_record(&user)
    })});

    // Insert new settings
    sqlx::query!(
        "INSERT INTO settings (user_id) VALUES ($1)",
        user.id,
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Error inserting settings: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    Ok(Json(user_response))
}

fn filter_user_record(user: &User) -> UserRegisterResponse {
    UserRegisterResponse {
        id: user.id,
        email: user.email.to_owned(),
        created_at: user.created_at,
        updated_at: user.updated_at,
    }
}

#[utoipa::path(
    method(post),
    path = "/auth/login",
    request_body(content = LoginUser, content_type = "application/json"),
    responses(
        (status = axum::http::StatusCode::OK, description = "Success", content_type = "text/plain"),
        (status = axum::http::StatusCode::BAD_REQUEST, description = "Error", content_type = "text/plain")
    )
)]
pub async fn login(
    State(data): State<Arc<AppState>>,
    Json(body): Json<LoginUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let email = body.email.to_ascii_lowercase();
    println!("email: {}", email);
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
        .fetch_optional(&data.db)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Database error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?
        .ok_or_else(|| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "Invalid email or password",
            });
            println!("Something goes wrong");
            (StatusCode::BAD_REQUEST, Json(error_response))
        })?;

    let is_valid = match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };
    println!("user: {:?}", user);

    if !is_valid {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Invalid email or password"
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build(("token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    let mut response = Response::new(json!({"status": "success", "token": token}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    println!("response: {:?}", &response);
    Ok(response)
}

#[utoipa::path(
    method(post),
    path = "/auth/logout",
    responses(
        (status = axum::http::StatusCode::OK, description = "Success", content_type = "text/plain"),
        (status = axum::http::StatusCode::BAD_REQUEST, description = "Error", content_type = "text/plain")
    )
)]
pub async fn logout() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let cookie = Cookie::build("token")
        .path("/")
        .max_age(time::Duration::weeks(-1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    let mut response = Response::new(json!({"status": "success"}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}
