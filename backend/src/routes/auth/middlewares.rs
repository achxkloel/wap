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
use std::sync::Arc;

///
///
/// # Arguments
///
/// * `jar`:
/// * `State(service)`:
/// * `req`:
/// * `next`:
///
/// returns: Result<Response<Body>, (StatusCode, Json<AuthError>)>
pub(crate) async fn auth<S>(
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
