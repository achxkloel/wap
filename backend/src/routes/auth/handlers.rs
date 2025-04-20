// use std::sync::Arc;
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{
    extract::{Json, State},
    http::{header, HeaderMap, Response, StatusCode},
    response::IntoResponse,
};
use serde_json::json;

use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use rand_core::OsRng;
use serde::Serialize;

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use crate::shared::models::AppState;

// -------------------------------------------------------------------------------------------------
// Routes
// -------------------------------------------------------------------------------------------------
// SIGNUP handler

// use axum::{Json, response::IntoResponse};
use crate::routes::auth::models::{
    LoginError, LoginSuccess, LoginUser, LoginUserSchema, LogoutError, LogoutSuccess, RefreshError,
    RefreshSuccess, RegisterError, RegisterSuccess, RegisterUserRequestSchema, TokenClaims, User,
    UserRegisterResponse,
};
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
    post,
    path = "/auth/register",
    request_body(content = RegisterUserRequestSchema, content_type = "application/json"),
    responses(
        (status = axum::http::StatusCode::OK, description = "Success", body = RegisterSuccess, content_type = "text/plain"),
        (status = axum::http::StatusCode::BAD_REQUEST, body = RegisterError, description = "Error", content_type = "text/plain")
    )
)]
pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<RegisterUserRequestSchema>,
) -> Result<(StatusCode, Json<RegisterSuccess>), (StatusCode, Json<RegisterError>)> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Error while hashing password: {}", e),
            });
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(RegisterError {
                    status: "fail".to_string(),
                    message: format!("Error while hashing password: {}", e),
                }),
            )
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
            (StatusCode::INTERNAL_SERVER_ERROR, Json(RegisterError {
                status: "fail".to_string(),
                message: format!("Database error: {}", e),
            }))
        })?;

    // Insert new settings
    sqlx::query!("INSERT INTO settings (user_id) VALUES ($1)", user.id,)
        .execute(&state.db)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Error inserting settings: {}", e),
            });
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(RegisterError {
                    status: "fail".to_string(),
                    message: format!("Error inserting settings: {}", e),
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
    State(data): State<AppState>,
    Json(body): Json<LoginUserSchema>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let email = body.email.to_ascii_lowercase();
    tracing::debug!("email: {}", email);
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
        .fetch_optional(&data.db)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Database error: {}", e),
            });
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(LoginError {
                    status: "error".to_string(),
                    message: format!("Database error: {}", e),
                }),
            )
        })?
        .ok_or_else(|| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "Invalid email or password",
            });
            tracing::error!("Something goes wrong");
            (
                StatusCode::BAD_REQUEST,
                Json(LoginError {
                    status: "fail".to_string(),
                    message: "Invalid email or password".to_string(),
                }),
            )
        })?;

    let is_valid = match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };
    tracing::debug!("user: {:?}", user);

    if !is_valid {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Invalid email or password"
        });
        return Err((
            StatusCode::BAD_REQUEST,
            Json(LoginError {
                status: "fail".to_string(),
                message: "Invalid email or password".to_string(),
            }),
        ));
    }

    let now = chrono::Utc::now();

    let access_token = create_token(
        &user.id.to_string(),
        (now + chrono::Duration::minutes(60)).timestamp() as usize,
        data.settings.jwt_secret.as_ref(),
    );

    let refresh_token = create_token(
        &user.id.to_string(),
        (now + chrono::Duration::days(30)).timestamp() as usize,
        data.settings.jwt_secret.as_ref(),
    );

    // TODO: Better to remove token from cookies, when logging out it need to be removed form two places
    // let access_cookie = Cookie::build(("access_token", access_token.to_owned()))
    //     .path("/")
    //     .max_age(time::Duration::minutes(60))
    //     .same_site(SameSite::Lax)
    //     .http_only(true)
    //     .build();
    //
    // let refresh_cookie = Cookie::build(("refresh_token", refresh_token.to_owned()))
    //     .path("/")
    //     .max_age(time::Duration::days(30))
    //     .same_site(SameSite::Lax)
    //     .http_only(true)
    //     .build();

    let mut response = Response::new(
        json!(LoginSuccess {
            status: "success".to_string(),
            access_token: access_token,
            refresh_token: refresh_token
        })
        .to_string(),
    );
    let (mut parts, body) = response.into_parts();
    parts.status = StatusCode::CREATED;
    response = Response::from_parts(parts, body);

    // response.headers_mut().append(
    //     header::SET_COOKIE,
    //     access_cookie.to_string().parse().unwrap(),
    // );
    // response.headers_mut().append(
    //     header::SET_COOKIE,
    //     refresh_cookie.to_string().parse().unwrap(),
    // );

    tracing::debug!("response from login handler: {:?}", &response);
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
    State(data): State<AppState>,
    cookie_jar: CookieJar,
    header: HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, Json<RefreshError>)> {
    // let token = cookie_jar
    //     .get("refresh_token")
    //     .map(|cookie| {
    //         println!("token cookie");
    //         cookie.value().to_string()
    //     })
    //     .or_else(|| {
    //         println!("token header");
    //         header
    //             .get("Authorization")
    //             .and_then(|auth_header| auth_header.to_str().ok())
    //             .and_then(|auth_value| {
    //                 if auth_value.starts_with("Bearer ") {
    //                     Some(auth_value[7..].to_owned())
    //                 } else {
    //                     None
    //                 }
    //             })
    //     });

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
            let error_response = RefreshError{
                status: "fail".to_string(),
                message: "You are not logged in, please provide token".to_string(),
            };
            (StatusCode::UNAUTHORIZED, Json(error_response))
        })?;

    // println!("token: {:?}", token);
    // let token = token.ok_or_else(|| {
    //     let error_response = serde_json::json!({
    //         "status": "fail",
    //         "message": "You are not logged in, please provide token",
    //     });
    //     (StatusCode::UNAUTHORIZED, Json(error_response))
    // })?;

    println!("token ok: {:?}", token);
    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(data.settings.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| {
        let error_response = RefreshError{
            status: "fail".to_string(),
            message: "Invalid token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(error_response))
    })?
    .claims;

    // Assuming `users.id` is i32:
    let user_id: i32 = claims.sub.parse().map_err(|_| {
        let error_response = RefreshError{
            status: "fail".to_string(),
            message: "Invalid token subject format". to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(error_response))
    })?;

    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_optional(&data.db)
        .await
        .map_err(|e| {
            let error_response = RefreshError{
                status: "fail".to_string(),
                message: format!("Error fetching user from database: {}", e),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let user = user.ok_or_else(|| {
        let error_response = RefreshError {
            status: "fail".to_string(),
            message: "The user belonging to this token no longer exists".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(error_response))
    })?;

    let now = chrono::Utc::now();
    let new_access_token = create_token(
        &user.id.to_string(),
        (now + chrono::Duration::minutes(60)).timestamp() as usize,
        data.settings.jwt_secret.as_ref(),
    );

    // let new_access_cookie = Cookie::build(("access_token", new_access_token.to_owned()))
    //     .path("/")
    //     .max_age(time::Duration::minutes(60))
    //     .same_site(SameSite::Lax)
    //     .http_only(true)
    //     .build();

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

    // response.headers_mut().insert(
    //     header::SET_COOKIE,
    //     new_access_cookie.to_string().parse().unwrap(),
    // );

    println!("response: {:?}", &response);

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
    use std::thread::sleep;
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
        let app = AppState {
            db: pool.clone(),
            settings: WapSettings {
                database_url: "".to_string(),
                jwt_secret: "aaaaa".to_string(),
                jwt_expires_in: "".to_string(),
                jwt_maxage: 0,
            },
        };

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
