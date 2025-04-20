use crate::shared::models::{AppState};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json as AxumJson,
};
use std::sync::Arc;
use crate::routes::auth::models::User;
use crate::routes::weather_location::models::Settings;

#[utoipa::path(
    method(put),
    path = "/user/settings",
    request_body = Settings,
    responses(
        (status = 200, description = "Settings updated"),
        (status = 400, description = "Bad request")
    )
)]
pub async fn put_settings(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Json(payload): Json<Settings>,
) -> Result<impl IntoResponse, (StatusCode, AxumJson<serde_json::Value>)> {
    // Simulated user ID – in real apps this should come from auth middleware
    println!("{:?}", payload);
    let user_id = user.id;
    println!("{:#?} | user: {:#?}", payload, user);

    // Update settings
    sqlx::query!(
            "UPDATE settings SET theme = $1, notifications_enabled = $2, radius = $3, updated_at = NOW() WHERE user_id = $4",
            payload.theme as _,
            payload.notifications_enabled,
            payload.radius,
            user_id
        )
            .execute(&state.db)
            .await
            .map_err(|err| {
                let msg = serde_json::json!({ "error": format!("Failed to update settings: {}", err) });
                (StatusCode::INTERNAL_SERVER_ERROR, AxumJson(msg))
            })?;

    Ok((StatusCode::OK, "Settings saved successfully"))
}

#[utoipa::path(
    get,
    path = "/user/settings",
    responses(
        (status = 200, description = "User settings returned", body = Settings),
        (status = 404, description = "No settings found for user"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_settings(
    Extension(user): Extension<User>, // Get user injected by middleware
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_id = user.id;
    println!("user: {:#?}", user);

    let settings = sqlx::query_as::<_, Settings>(
        r#"
        SELECT theme, notifications_enabled, radius
        FROM settings
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|err| {
        let json = serde_json::json!({ "error": format!("Database error: {}", err) });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json))
    })?;

    match settings {
        Some(s) => Ok((StatusCode::OK, Json(s))),
        None => {
            let json = serde_json::json!({
                "status": "fail",
                "message": "Settings not found for user"
            });
            Err((StatusCode::NOT_FOUND, Json(json)))
        }
    }
}
