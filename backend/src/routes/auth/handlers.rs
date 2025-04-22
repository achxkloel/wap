// use std::sync::Arc;
use argon2::{PasswordHasher, PasswordVerifier};
use axum::extract::Query;
use axum::http::{header, HeaderValue};
use axum::{
    extract::{Json, State},
    http::{HeaderMap, Response, StatusCode},
    response::IntoResponse,
};
use chrono::Utc;
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;

use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::shared::models::{AppState, DatabaseId};

// TODO: add to response also user dat
// TODO: add endpoint to change user passwords
// TODO: add endpoint to authenticate google auth just google id check for user + googleId field to user

// -------------------------------------------------------------------------------------------------
// Routes
// -------------------------------------------------------------------------------------------------
// SIGNUP handler

// use axum::{Json, response::IntoResponse};
use crate::routes::auth::middlewares::auth;
use crate::routes::auth::models::{
    LoginError, LoginSuccess, LoginUser, LoginUserSchema, OAuthParams, QueryCode, RefreshError,
    RefreshSuccess, RegisterError, RegisterSuccess, RegisterUserRequestSchema, TokenClaims, UserDb,
    UserRegisterResponse,
};
use crate::routes::auth::services::{
    AuthService, AuthServiceImpl, GoogleAuthService, JwtConfigImpl,
};
use crate::routes::auth::{middlewares, services};
use crate::routes::settings::handlers::{get_settings, put_settings};
use crate::routes::settings::services::SettingsService;
use utoipa::ToSchema;
use utoipa_axum::router::{OpenApiRouter, UtoipaMethodRouterExt};
use utoipa_axum::routes;

#[derive(Debug, Serialize, ToSchema)]
pub struct SignupResponse {
    pub message: String,
}

#[utoipa::path(
    post,
    path = "/auth/register",
    request_body(content = RegisterUserRequestSchema, content_type = "application/json"),
    responses(
        (status = axum::http::StatusCode::OK, description = "Success", body = RegisterSuccess, content_type = "text/plain"),
        (status = axum::http::StatusCode::BAD_REQUEST, body = RegisterError, description = "Error", content_type = "text/plain")
    )
)]
pub async fn register<S>(
    State(service): State<Arc<S>>,
    Json(body): Json<RegisterUserRequestSchema>,
) -> Result<(StatusCode, Json<RegisterSuccess>), (StatusCode, Json<RegisterError>)>
where
    S: AuthServiceImpl,
{
    let user = service.register(&body).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(RegisterError {
                status: "fail".to_string(),
                message: e.to_string(),
            }),
        )
    })?;

    Ok((
        StatusCode::CREATED,
        Json(RegisterSuccess {
            status: "success".to_string(),
            data: UserRegisterResponse {
                id: user.id,
                email: user.email.to_owned(),
                created_at: user.created_at,
                updated_at: user.updated_at,
            },
        }),
    ))
}

fn filter_user_record(user: &UserDb) -> UserRegisterResponse {
    UserRegisterResponse {
        id: user.id.clone(),
        email: user.email.to_owned(),
        created_at: user.created_at,
        updated_at: user.updated_at,
    }
}

/// Pull all of your JWT logic into a single async helper.
async fn create_login_response<S>(user: UserDb, state: &S) -> LoginSuccess
where
    S: JwtConfigImpl, // now jwt_secret is async
{
    // 1) grab the secret and expiries asynchronously
    let secret = state.jwt_secret().await;
    let access_minutes = state.access_expires_minutes().await;
    let refresh_days = state.refresh_expires_days().await;

    // 2) compute timestamps
    let now = chrono::Utc::now();
    let access_exp = (now + chrono::Duration::minutes(access_minutes)).timestamp() as usize;
    let refresh_exp = (now + chrono::Duration::days(refresh_days)).timestamp() as usize;

    // 3) sign tokens
    let access_token = state
        .create_jwt_token(&user.id.0.to_string(), access_exp)
        .await;
    let refresh_token = state
        .create_jwt_token(&user.id.0.to_string(), refresh_exp)
        .await;

    LoginSuccess {
        status: "success".into(),
        access_token,
        refresh_token,
    }
}

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body(content = LoginUser, content_type = "application/json"),
    responses(
        (status = axum::http::StatusCode::OK, body=LoginSuccess, description = "Success", content_type = "text/plain"),
        (status = axum::http::StatusCode::BAD_REQUEST, body=LoginError, description = "Error", content_type = "text/plain")
    )
)]
pub async fn login<S>(
    State(service): State<Arc<S>>,
    Json(body): Json<LoginUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<LoginError>)>
where
    S: AuthServiceImpl,
{
    let user = service.login(body).await.map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(LoginError {
                status: "fail".to_string(),
                message: e.to_string(),
            }),
        )
    })?;

    let data = create_login_response(user.clone(), &*service).await;
    let mut response = Response::new(json!(data).to_string());
    *response.status_mut() = StatusCode::CREATED;
    Ok(response)
}

#[utoipa::path(
    post,
    path = "/auth/refresh",
    responses(
        (status = axum::http::StatusCode::OK, body=RefreshSuccess, description = "Success", content_type = "text/plain"),
        (status = axum::http::StatusCode::BAD_REQUEST, body=RefreshError, description = "Error", content_type = "text/plain")
    )
)]
pub async fn refresh<S>(
    State(state): State<Arc<S>>,
    header: HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, Json<RefreshError>)>
where
    S: AuthServiceImpl,
{
    let token = header
        .get("Authorization")
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        })
        .ok_or_else(|| {
            let error_response = RefreshError {
                status: "fail".to_string(),
                message: "You are not logged in, please provide token".to_string(),
            };
            (StatusCode::UNAUTHORIZED, Json(error_response))
        })?;

    let claims = state.token_claim(&token).await?;

    let user_id: i32 = claims.sub.parse().map_err(|_| {
        let error_response = RefreshError {
            status: "fail".to_string(),
            message: "Invalid token subject format".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(error_response))
    })?;

    let user = state.refresh(user_id).await?;

    let now = chrono::Utc::now();
    let new_access_token = state
        .create_jwt_token(
            &user.id.0.to_string(),
            (now + chrono::Duration::minutes(60)).timestamp() as usize,
        )
        .await;

    let mut response = Response::new(
        json!(RefreshSuccess {
            status: "success".to_string(),
            access_token: new_access_token
        })
        .to_string(),
    );

    let (mut parts, body) = response.into_parts();
    parts.status = StatusCode::CREATED;
    response = Response::from_parts(parts, body);

    Ok(response)
}

#[utoipa::path(
    post,
    path = "/auth/google",
    responses(
        (status = axum::http::StatusCode::OK, body=RefreshSuccess, description = "Success", content_type = "text/plain"),
        (status = axum::http::StatusCode::BAD_REQUEST, body=RefreshError, description = "Error", content_type = "text/plain")
    )
)]
pub async fn google_oauth_handler<S>(
    State(service): State<Arc<S>>,
    Query(params): Query<OAuthParams>,
) -> Result<impl IntoResponse, (StatusCode, Json<RefreshError>)>
where
    S: GoogleAuthService,
{
    // 1) missing code → 400
    if params.code.trim().is_empty() {
        let err = RefreshError {
            status: "fail".into(),
            message: "Authorization code not provided!".into(),
        };
        return Err((StatusCode::BAD_REQUEST, Json(err)));
    }

    // 2) exchange code → token_response or 502
    let token_resp = service
        .request_token(&params.code, &params.state)
        .await
        .map_err(|e| {
            let msg = e.to_string();
            tracing::error!("request_token error: {}", msg);
            let err = RefreshError {
                status: "fail".into(),
                message: msg,
            };
            (StatusCode::BAD_GATEWAY, Json(err))
        })?;

    // 3) fetch Google user → or 502
    let google_user = service
        .get_google_user(&token_resp.access_token, &token_resp.id_token)
        .await
        .map_err(|e| {
            let msg = e.to_string();
            tracing::error!("get_google_user error: {}", msg);
            let err = RefreshError {
                status: "fail".into(),
                message: msg,
            };
            (StatusCode::BAD_GATEWAY, Json(err))
        })?;

    // 4) upsert into your DB & get back a user_id or 500
    let user: UserDb = service
        .upsert_google_user(&google_user)
        .await
        .map_err(|e| {
            let msg = e.to_string();
            tracing::error!("upsert_google_user error: {}", msg);
            let err = RefreshError {
                status: "error".into(),
                message: msg,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(err))
        })?;

    // TODO use already the setup form login to creat access and refrsshh tokens

    let data = create_login_response(user.clone(), &*service).await;
    let mut response = Response::new(json!(data).to_string());
    *response.status_mut() = StatusCode::CREATED;
    Ok(response)
}

/// Fully‐generic: you supply the `service: S`.
pub fn router_with_service<S>(app: AppState, normal_service: Arc<S>) -> OpenApiRouter
where
    S: AuthServiceImpl + GoogleAuthService,
{
    let register = register::<S>;
    let login = login::<S>;
    let refresh = refresh::<S>;
    let google_oauth_handler = google_oauth_handler::<S>;

    let router = utoipa_axum::router::OpenApiRouter::new()
        .routes(routes!(register))
        .routes(routes!(login))
        .routes(routes!(refresh).layer(axum::middleware::from_fn_with_state(
            app.clone(),
            middlewares::auth,
        )))
        .routes(routes!(google_oauth_handler))
        .with_state(normal_service);

    router
}

/// A convenience wrapper that uses the real Postgres implementation.
pub fn router(app: AppState) -> OpenApiRouter {
    let auth_service = services::AuthService::new(app.db.clone(), &app.settings.clone());
    let arc_service = Arc::new(auth_service);
    router_with_service(app.clone(), arc_service.clone())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::shared::models::DatabaseId;
//     use axum::body::Body;
//     use axum::http;
//     use axum::http::Request;
//     use http_body_util::BodyExt;
//     use tower::ServiceExt;
//
//     #[sqlx::test]
//     async fn test_register_and_login(pool: sqlx::PgPool) {
//         // Test the login function here
//         let app = crate::tests::tests::init_app_state(pool.clone()).await;
//
//         // Prepare router
//         let (router, _) = crate::routes::auth::handlers(app.clone()).split_for_parts();
//
//         // Register user
//         let register_request = RegisterUserRequestSchema {
//             email: "a@a.com".to_string(),
//             password: "123456".to_string(),
//         };
//         let register_response = router
//             .clone()
//             .oneshot(
//                 Request::builder()
//                     .method(http::Method::POST)
//                     .uri("/auth/register")
//                     .header("Content-Type", "application/json")
//                     .body(Body::from(serde_json::to_vec(&register_request).unwrap()))
//                     .unwrap(),
//             )
//             .await
//             .unwrap();
//
//         assert_eq!(register_response.status(), StatusCode::CREATED);
//         let body = register_response
//             .into_body()
//             .collect()
//             .await
//             .unwrap()
//             .to_bytes();
//         let register_body: RegisterSuccess = serde_json::from_slice(&body).expect("error");
//
//         println!("register_body: {:?}", register_body);
//
//         // Login user
//         let login_request = LoginUser {
//             email: "a@a.com".to_string(),
//             password: "123456".to_string(),
//         };
//         let login_response = router
//             .clone()
//             .oneshot(
//                 Request::builder()
//                     .method(http::Method::POST)
//                     .uri("/auth/login")
//                     .header("Content-Type", "application/json")
//                     .body(Body::from(serde_json::to_vec(&login_request).unwrap()))
//                     .unwrap(),
//             )
//             .await
//             .unwrap();
//
//         assert_eq!(login_response.status(), StatusCode::CREATED);
//         let body = login_response
//             .into_body()
//             .collect()
//             .await
//             .unwrap()
//             .to_bytes();
//         let login_body: LoginSuccess = serde_json::from_slice(&body).expect("error");
//
//         // deserialize access token
//         // let decodeing_key = DecodingKey::from_secret(&app.env.jwt_secret);
//         // let access_token = decode(&login_body.access_token, &app.env.jwt_secret).unwrap();
//         let claims = decode::<TokenClaims>(
//             &login_body.access_token,
//             &DecodingKey::from_secret(app.clone().settings.jwt_secret.as_ref()),
//             &Validation::default(),
//         )
//         .map_err(|_| {
//             let error_response = serde_json::json!({
//                 "status": "fail",
//                 "message": "Invalid token",
//             });
//             (StatusCode::UNAUTHORIZED, Json(error_response))
//         })
//         .unwrap();
//
//         println!("claims: {:?}", claims);
//         assert_eq!(
//             register_body.data.id,
//             claims.claims.sub.parse::<DatabaseId>().unwrap()
//         );
//
//         // Logout user - try failed - authorization header is missing
//         // let logout_response = router
//         //     .clone()
//         //     .oneshot(
//         //         Request::builder()
//         //             .method(http::Method::POST)
//         //             .uri("/auth/logout")
//         //             .header("Content-Type", "application/json")
//         //             .body(Body::empty())
//         //             .unwrap(),
//         //     )
//         //     .await
//         //     .unwrap();
//         //
//         // assert_eq!(logout_response.status(), StatusCode::UNAUTHORIZED);
//
//         // let logout_response = router
//         //     .clone()
//         //     .oneshot(
//         //         Request::builder()
//         //             .method(http::Method::POST)
//         //             .uri("/auth/logout")
//         //             .header("Content-Type", "application/json")
//         //             .header(
//         //                 "Authorization",
//         //                 format!("Bearer {}", login_body.access_token),
//         //             )
//         //             .body(Body::empty())
//         //             .unwrap(),
//         //     )
//         //     .await
//         //     .unwrap();
//         //
//         // assert_eq!(logout_response.status(), StatusCode::OK);
//         // println!("logout_response: {:?}", &logout_response);
//
//         // Refresh
//         let refresh_response = router
//             .clone()
//             .oneshot(
//                 Request::builder()
//                     .method(http::Method::POST)
//                     .uri("/auth/refresh")
//                     .header("Content-Type", "application/json")
//                     .header(
//                         "Authorization",
//                         format!("Bearer {}", login_body.refresh_token),
//                     )
//                     // .header("Authorization", format!("Bearer {}", login_body.access_token))
//                     .body(Body::empty())
//                     .unwrap(),
//             )
//             .await
//             .unwrap();
//
//         assert_eq!(refresh_response.status(), StatusCode::CREATED);
//
//         // tokio::time::sleep(std::time::Duration::from_millis(100)).await;
//
//         let body = refresh_response
//             .into_body()
//             .collect()
//             .await
//             .unwrap()
//             .to_bytes();
//         let refresh_body: RefreshSuccess = serde_json::from_slice(&body).expect("error");
//
//         // assert_ne!(
//         //     refresh_body.access_token,
//         //     login_body.access_token
//         // );
//
//         // TODO: how can I mock now.timestamp so the access token will be different for each request
//
//         // Logout
//     }
//
//     #[sqlx::test]
//     #[ignore = "wip: oauth2 register/login not implemented yet"]
//     async fn test_google_oath2_register_and_login(pool: sqlx::PgPool) {
//         assert!(false);
//     }
// }
