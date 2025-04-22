use std::sync::Arc;
use crate::routes::auth::models::{AuthError, UserDb};
use crate::routes::auth::services::AuthServiceImpl;
use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use axum_extra::extract::cookie::CookieJar;
// NOTE: the `E` in `Result<Response, E>` must implement `IntoResponse`.
// pub async fn auth(
//     jar: CookieJar,
//     State(state): State<AppState>,
//     mut req: Request<Body>,
//     next: Next,
// ) -> Result<Response, (StatusCode, Json<AuthError>)> {
//     tracing::debug!("auth middleware: {:?}", req);
//     let token = req
//         .headers()
//         .get(header::AUTHORIZATION)
//         .and_then(|h| h.to_str().ok())
//         .and_then(|v| v.strip_prefix("Bearer ").map(str::to_owned))
//         .ok_or_else(|| {
//             tracing::error!("Missing bearer token");
//             (
//                 StatusCode::UNAUTHORIZED,
//                 Json(AuthError {
//                     message: "You are not logged in, please provide a token".into(),
//                 }),
//             )
//         })?;
//
//     let claims: TokenClaims = decode::<TokenClaims>(
//         &token,
//         &DecodingKey::from_secret(state.settings.jwt_secret.as_bytes()),
//         &Validation::default(),
//     )
//     .map_err(|_| {
//         tracing::error!("Decoding jwt failed");
//         (
//             StatusCode::UNAUTHORIZED,
//             Json(AuthError {
//                 message: "Invalid token".into(),
//             }),
//         )
//     })?
//     .claims;
//
//     let user_id: i32 = claims.sub.parse().map_err(|_| {
//         tracing::error!("Token sub is not a valid user id");
//         (
//             StatusCode::UNAUTHORIZED,
//             Json(AuthError {
//                 message: "Invalid token".into(),
//             }),
//         )
//     })?;
//
//     let user = sqlx::query_as!(UserDb, "SELECT * FROM users WHERE id = $1", user_id)
//         .fetch_optional(&state.db)
//         .await
//         .map_err(|e| {
//             tracing::error!("Database error: {e}");
//             (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 Json(AuthError {
//                     message: format!("Database error: {e}"),
//                 }),
//             )
//         })?
//         .ok_or_else(|| {
//             tracing::error!("No user with id {}", claims.sub);
//             (
//                 StatusCode::UNAUTHORIZED,
//                 Json(AuthError {
//                     message: "The user belonging to this token no longer exists".into(),
//                 }),
//             )
//         })?;
//
//     req.extensions_mut().insert(user);
//     tracing::debug!("Authenticated user: {:?}", req.extensions().get::<UserDb>());
//     Ok(next.run(req).await)
// }

pub async fn auth<S>(
    jar: CookieJar,
    State(service): State<Arc<S>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, Json<AuthError>)>
where
    S: AuthServiceImpl,
{
    // 1) extract Bearer token
    let token = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer ").map(str::to_owned))
        .ok_or_else(|| {
            let err = AuthError::new("Missing Authorization Bearer token");
            (StatusCode::UNAUTHORIZED, Json(err))
        })?;

    // 2) validate & fetch user
    let user: UserDb = service.validate_token(&token).await?;

    // 3) stash in request extensions
    req.extensions_mut().insert(user);

    // 4) forward
    Ok(next.run(req).await)
}
