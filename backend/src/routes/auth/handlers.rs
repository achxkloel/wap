// use std::sync::Arc;
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{
    extract::{Json, State},
    http::{header, Response, StatusCode, HeaderMap},
    response::IntoResponse,
    routing::post,
};
use serde_json::json;

use axum_extra::extract::cookie::{Cookie, SameSite, CookieJar};
use bcrypt::{hash, verify, DEFAULT_COST};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use chrono::{Duration, Utc};
use jsonwebtoken::{encode, decode, EncodingKey, Header, Validation, DecodingKey};
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

fn create_token(user_id: &str, exp: usize, secret: &str) -> String {
    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user_id.to_string(),
        exp,
        iat,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
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

    let access_token = create_token(
        &user.id.to_string(),
        (now + chrono::Duration::minutes(60)).timestamp() as usize,
        data.env.jwt_secret.as_ref(),
    );

    let refresh_token = create_token(
        &user.id.to_string(),
        (now + chrono::Duration::days(30)).timestamp() as usize,
        data.env.jwt_secret.as_ref(),
    );

    let access_cookie = Cookie::build(("access_token", access_token.to_owned()))
        .path("/")
        .max_age(time::Duration::minutes(60))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    let refresh_cookie = Cookie::build(("refresh_token", refresh_token.to_owned()))
        .path("/")
        .max_age(time::Duration::days(30))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    let mut response = Response::new(json!({"status": "success", "access_token": access_token, "refresh_token": refresh_token}).to_string());
    response
        .headers_mut()
        .append(header::SET_COOKIE, access_cookie.to_string().parse().unwrap());
    response
        .headers_mut()
        .append(header::SET_COOKIE, refresh_cookie.to_string().parse().unwrap());
    println!("response: {:?}", &response);
    Ok(response)
}

#[utoipa::path(
    method(post),
    path = "/auth/refresh",
    responses(
        (status = axum::http::StatusCode::OK, description = "Success", content_type = "text/plain"),
        (status = axum::http::StatusCode::BAD_REQUEST, description = "Error", content_type = "text/plain")
    )
)]
pub async fn refresh(
    State(data): State<Arc<AppState>>,
    cookie_jar: CookieJar,
    header: HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let token = cookie_jar
        .get("refresh_token")
        .map(|cookie| {
            println!("token cookie");
            cookie.value().to_string()
        })
        .or_else(|| {
            println!("token header");
            header
                .get("Authorization")
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        });

    println!("token: {:?}", token);
    let token = token.ok_or_else(|| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "You are not logged in, please provide token",
        });
        (StatusCode::UNAUTHORIZED, Json(error_response))
    })?;

    println!("token ok: {:?}", token);
    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(data.env.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Invalid token",
        });
        (StatusCode::UNAUTHORIZED, Json(error_response))
    })?
    .claims;

    // Assuming `users.id` is i32:
    let user_id: i32 = claims.sub.parse().map_err(|_| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Invalid token subject format",
        });
        (StatusCode::UNAUTHORIZED, Json(error_response))
    })?;

    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_optional(&data.db)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Error fetching user from database: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let user = user.ok_or_else(|| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "The user belonging to this token no longer exists",
        });
        (StatusCode::UNAUTHORIZED, Json(error_response))
    })?;

    let now = chrono::Utc::now();
    let new_access_token = create_token(
        &user.id.to_string(),
        (now + chrono::Duration::minutes(60)).timestamp() as usize,
        data.env.jwt_secret.as_ref(),
    );

    let new_access_cookie = Cookie::build(("access_token", new_access_token.to_owned()))
        .path("/")
        .max_age(time::Duration::minutes(60))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    let mut response = Response::new(json!({"status": "success", "access_token": new_access_token}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, new_access_cookie.to_string().parse().unwrap());
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
    let cookie = Cookie::build("access_token")
        .path("/")
        .max_age(time::Duration::days(-360))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    let cookie = Cookie::build("refresh_token")
        .path("/")
        .max_age(time::Duration::days(-360))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    let mut response = Response::new(json!({"status": "success"}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}
