// use std::sync::Arc;
use argon2::{PasswordHasher, PasswordVerifier};
use axum::{
    extract::{Json, State},
    http::{HeaderMap, Response, StatusCode},
    response::IntoResponse,
};
use serde_json::json;

use serde::Serialize;

use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::shared::models::AppState;

// -------------------------------------------------------------------------------------------------
// Routes
// -------------------------------------------------------------------------------------------------
// SIGNUP handler

// use axum::{Json, response::IntoResponse};
use crate::routes::auth::models::{
    LoginError, LoginSuccess, LoginUser, LoginUserSchema, RefreshError, RefreshSuccess,
    RegisterError, RegisterSuccess, RegisterUserRequestSchema, TokenClaims, User,
    UserRegisterResponse,
};
use crate::routes::auth::utils::create_token;
use utoipa::ToSchema;

use crate::routes::auth::service::{AuthService, PgAuthService};

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
pub async fn register(
    State(service): State<PgAuthService>,
    Json(body): Json<RegisterUserRequestSchema>,
) -> Result<(StatusCode, Json<RegisterSuccess>), (StatusCode, Json<RegisterError>)> {
    let user = service.register(body).await.map_err(|e| {
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

fn filter_user_record(user: &User) -> UserRegisterResponse {
    UserRegisterResponse {
        id: user.id,
        email: user.email.to_owned(),
        created_at: user.created_at,
        updated_at: user.updated_at,
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
pub async fn login(
    State(service): State<PgAuthService>,
    Json(body): Json<LoginUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<LoginError>)> {
    let user = service.login(body).await.map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(LoginError {
                status: "fail".to_string(),
                message: e.to_string(),
            }),
        )
    })?;

    let now = chrono::Utc::now();
    let access_token = create_token(
        &user.id.to_string(),
        (now + chrono::Duration::minutes(60)).timestamp() as usize,
        service.settings.jwt_secret.as_ref(),
    );

    let refresh_token = create_token(
        &user.id.to_string(),
        (now + chrono::Duration::days(30)).timestamp() as usize,
        service.settings.jwt_secret.as_ref(),
    );

    let mut response = Response::new(
        json!(LoginSuccess {
            status: "success".to_string(),
            access_token,
            refresh_token,
        })
            .to_string(),
    );
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
pub async fn refresh(
    State(state): State<PgAuthService>,
    header: HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, Json<RefreshError>)> {
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

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(state.settings.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| {
        let error_response = RefreshError {
            status: "fail".to_string(),
            message: "Invalid token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(error_response))
    })?
    .claims;

    let user_id: i32 = claims.sub.parse().map_err(|_| {
        let error_response = RefreshError {
            status: "fail".to_string(),
            message: "Invalid token subject format".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(error_response))
    })?;

    let service = PgAuthService::new(state.db.clone(), state.settings.clone());
    let user = service.refresh(user_id).await.map_err(|e| {
        let error_response = RefreshError {
            status: "fail".to_string(),
            message: e.to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(error_response))
    })?;

    let now = chrono::Utc::now();
    let new_access_token = create_token(
        &user.id.to_string(),
        (now + chrono::Duration::minutes(60)).timestamp() as usize,
        state.settings.jwt_secret.as_ref(),
    );

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

// #[utoipa::path(
//     post,
//     path = "/auth/logout",
//     responses(
//         (status = axum::http::StatusCode::OK, body=LogoutSuccess, description = "Success", content_type = "text/plain"),
//         (status = axum::http::StatusCode::BAD_REQUEST, body=LogoutError, description = "Error", content_type = "text/plain")
//     )
// )]
// pub async fn logout() -> Result<impl IntoResponse, (StatusCode, Json<LogoutError>)> {
//     let cookie = Cookie::build("refresh_token")
//         .path("/")
//         .max_age(time::Duration::days(-360))
//         .same_site(SameSite::Lax)
//         .http_only(true)
//         .build();
//
//     let mut response = Response::new(json!(LogoutSuccess {}).to_string());
//     response
//         .headers_mut()
//         .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
//     response
//         .headers_mut()
//         .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
//     let (mut parts, body) = response.into_parts();
//     parts.status = StatusCode::OK; //
//     response = Response::from_parts(parts, body);
//     Ok(response)
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::WapSettings;
    use axum::body::Body;
    use axum::http;
    use axum::http::Request;
    use http_body_util::BodyExt;
    use tower::ServiceExt;
    // for `collect`

    #[sqlx::test]
    async fn test_register_and_login(pool: sqlx::PgPool) {
        // Test the login function here
        let app = crate::tests::tests::init_app_state(pool.clone()).await;
        
        // Prepare router
        let (router, _) = crate::routes::auth::router(app.clone()).split_for_parts();

        // Register user
        let register_request = RegisterUserRequestSchema {
            email: "a@a.com".to_string(),
            password: "123456".to_string(),
        };
        let register_response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/auth/register")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_vec(&register_request).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(register_response.status(), StatusCode::CREATED);
        let body = register_response
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes();
        let register_body: RegisterSuccess = serde_json::from_slice(&body).expect("error");

        println!("register_body: {:?}", register_body);

        // Login user
        let login_request = LoginUser {
            email: "a@a.com".to_string(),
            password: "123456".to_string(),
        };
        let login_response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/auth/login")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_vec(&login_request).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(login_response.status(), StatusCode::CREATED);
        let body = login_response
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes();
        let login_body: LoginSuccess = serde_json::from_slice(&body).expect("error");

        // deserialize access token
        // let decodeing_key = DecodingKey::from_secret(&app.env.jwt_secret);
        // let access_token = decode(&login_body.access_token, &app.env.jwt_secret).unwrap();
        let claims = decode::<TokenClaims>(
            &login_body.access_token,
            &DecodingKey::from_secret(app.clone().settings.jwt_secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "Invalid token",
            });
            (StatusCode::UNAUTHORIZED, Json(error_response))
        })
        .unwrap();

        println!("claims: {:?}", claims);
        assert_eq!(
            register_body.data.id,
            claims.claims.sub.parse::<i32>().unwrap()
        );

        // Logout user - try failed - authorization header is missing
        // let logout_response = router
        //     .clone()
        //     .oneshot(
        //         Request::builder()
        //             .method(http::Method::POST)
        //             .uri("/auth/logout")
        //             .header("Content-Type", "application/json")
        //             .body(Body::empty())
        //             .unwrap(),
        //     )
        //     .await
        //     .unwrap();
        //
        // assert_eq!(logout_response.status(), StatusCode::UNAUTHORIZED);

        // let logout_response = router
        //     .clone()
        //     .oneshot(
        //         Request::builder()
        //             .method(http::Method::POST)
        //             .uri("/auth/logout")
        //             .header("Content-Type", "application/json")
        //             .header(
        //                 "Authorization",
        //                 format!("Bearer {}", login_body.access_token),
        //             )
        //             .body(Body::empty())
        //             .unwrap(),
        //     )
        //     .await
        //     .unwrap();
        //
        // assert_eq!(logout_response.status(), StatusCode::OK);
        // println!("logout_response: {:?}", &logout_response);

        // Refresh
        let refresh_response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/auth/refresh")
                    .header("Content-Type", "application/json")
                    .header(
                        "Authorization",
                        format!("Bearer {}", login_body.refresh_token),
                    )
                    // .header("Authorization", format!("Bearer {}", login_body.access_token))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(refresh_response.status(), StatusCode::CREATED);

        // tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let body = refresh_response
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes();
        let refresh_body: RefreshSuccess = serde_json::from_slice(&body).expect("error");

        // assert_ne!(
        //     refresh_body.access_token,
        //     login_body.access_token
        // );

        // TODO: how can I mock now.timestamp so the access token will be different for each request

        // Logout
    }
}
