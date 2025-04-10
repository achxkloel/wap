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

use crate::model::RegisterUserRequestSchema;
use crate::model::{AppState, CreateUser, LoginUser, LoginUserSchema, TokenClaims, User, };
use crate::response::{ResponseUser,RegisterResponse};

// -------------------------------------------------------------------------------------------------
// Routes
// -------------------------------------------------------------------------------------------------
// SIGNUP handler

// use axum::{Json, response::IntoResponse};
use utoipa::ToSchema;

pub async fn logout_handler() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let cookie = Cookie::build("token")
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    let mut response = Response::new(json!({"status": "success"}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}
