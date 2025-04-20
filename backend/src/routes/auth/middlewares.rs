use crate::routes::auth::models::{AuthError, TokenClaims, User};
use crate::shared::models::AppState;
use axum::{
    body::Body,
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;


// NOTE: the `E` in `Result<Response, E>` must implement `IntoResponse`.
pub async fn auth(
    jar: CookieJar,
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, Json<AuthError>)> {
    /* ─────────────────── 1. extract the token ─────────────────── */
    // let token = jar
    //     .get("access_token")
    //     .map(|c| c.value().to_owned())
    //     .or_else(|| {
    //         req.headers()
    //             .get(header::AUTHORIZATION)
    //             .and_then(|h| h.to_str().ok())
    //             .and_then(|v| v.strip_prefix("Bearer ").map(str::to_owned))
    //     })
    //     .ok_or_else(|| {
    //         (
    //             StatusCode::UNAUTHORIZED,
    //             Json(ErrorResponse {
    //                 status: "fail",
    //                 message: "You are not logged in, please provide a token".into(),
    //             }),
    //         )
    //     })?;
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer ").map(str::to_owned))
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(AuthError {
                    status: "fail",
                    message: "You are not logged in, please provide a token".into(),
                }),
            )
        })?;

    /* ─────────────────── 2. validate & decode ─────────────────── */
    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(state.settings.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(AuthError {
                status: "fail",
                message: "Invalid token".into(),
            }),
        )
    })?
    .claims;

    /* ─────────────────── 3. fetch user ─────────────────────────── */
    let user_id: i32 = claims.sub.parse().map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(AuthError {
                status: "fail",
                message: "Invalid token subject format".into(),
            }),
        )
    })?;

    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuthError {
                    status: "fail",
                    message: format!("Database error: {e}"),
                }),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(AuthError {
                    status: "fail",
                    message: "The user belonging to this token no longer exists".into(),
                }),
            )
        })?;

    /* ─────────────────── 4. stash user & continue ─────────────── */
    req.extensions_mut().insert(user);
    // let req: Request<Body> = req.map(|b| Body::from_stream(b));
    Ok(next.run(req).await)

    // Ok(next.run(req_body).await)
}
