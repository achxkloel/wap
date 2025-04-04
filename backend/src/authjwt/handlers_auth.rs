use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
};

use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::structs::{AppState, Claims, CreateUser, LoginUser};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;

use utoipa_axum::{router::OpenApiRouter, routes, PathItemExt};

// -------------------------------------------------------------------------------------------------
// Routes
// -------------------------------------------------------------------------------------------------
// SIGNUP handler

// use axum::{Json, response::IntoResponse};
use utoipa::ToSchema;


#[derive(Debug, Deserialize, ToSchema)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SignupResponse {
    pub message: String,
}

#[utoipa::path(
    method(post),
    path = "/auth/signup",
    request_body(content = SignupRequest, content_type = "application/json"),
    responses(
        (status = axum::http::StatusCode::OK, description = "Success", body = str, content_type = "text/plain"),
        (status = axum::http::StatusCode::BAD_REQUEST, description = "Error", content_type = "text/plain")
    )
)]
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SignupRequest>,
) -> impl IntoResponse {
    let hashed_password = hash(payload.password, DEFAULT_COST).unwrap();

    let result = sqlx::query!(
        "INSERT INTO users (email, password) VALUES ($1, $2)",
        payload.email,
        hashed_password
    )
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => (StatusCode::CREATED, "User created successfully"),
        Err(e) => {
            eprintln!("Signup error: {:?}", e);
            (StatusCode::BAD_REQUEST, "User already exists")
        }
    }
}




// // LOGIN handler
// #[utoipa::path(
//     method(post),
//     path = "/auth/login",
//     request_body(content = SignupRequest, content_type = "application/json"),
//     responses(
//         (status = OK, description = "Success", body = str, content_type = "text/plain")
//     )
// )]
// pub async fn login(
//     State(state): State<Arc<AppState>>,
//     Json(payload): Json<LoginUser>,
// ) -> impl IntoResponse {
//     let user = sqlx::query!(
//         "SELECT * FROM users WHERE email = $1",
//         payload.email
//     )
//         .fetch_optional(&state.db)
//         .await
//         .unwrap();
// 
//     if let Some(user) = user {
//         let valid = verify(payload.password, &user.password).unwrap();
//         if valid {
//             let expiration = Utc::now()
//                 .checked_add_signed(Duration::days(7))
//                 .expect("valid timestamp")
//                 .timestamp() as usize;
// 
//             let claims = Claims {
//                 sub: user.email,
//                 exp: expiration,
//             };
// 
//             let token = encode(
//                 &Header::default(),
//                 &claims,
//                 &EncodingKey::from_secret(state.env.jwt_secret.as_bytes()),
//             )
//                 .unwrap();
// 
//             return (StatusCode::OK, Json(serde_json::json!({ "token": token })));
//         }
//     }
//     (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Invalid username or password"})))
// }

// LOGOUT handler (client-side typically handles token discard, this is a placeholder)
// pub async fn logout(
//     TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
// ) -> impl IntoResponse {
//     println!("Token invalidated on client side: {}", auth.token());
//     (StatusCode::OK, "Logged out successfully")
// }
