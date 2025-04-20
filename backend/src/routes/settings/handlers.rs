use crate::routes::auth::models::User;
use crate::routes::weather_location::models::{UserSettings, UserSettingsUpdateRequest};
use crate::shared::models::AppState;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json as AxumJson,
};

#[utoipa::path(
    method(put),
    path = "/user/settings",
    request_body = UserSettingsUpdateRequest,
    responses(
        (status = 200, description = "Settings updated"),
        (status = 400, description = "Bad request")
    )
)]
pub async fn put_settings(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Json(payload): Json<UserSettingsUpdateRequest>,
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
        (status = 200, description = "User settings returned", body = UserSettingsUpdateRequest),
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

    let settings = sqlx::query_as::<_, UserSettings>(
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

#[cfg(test)]
mod tests {
    use crate::tests::tests::TestApp;
    use axum::{
        body::Body,
        http::{Method, Request},
    };
    use tower::ServiceExt;

    /// Helper that injects a `User` into the request’s extensions.
    // fn with_user<B>(mut req: Request<B>, user: &User) -> Request<B> {
    //     req.extensions_mut().insert(user.clone());
    //     req
    // }

    #[sqlx::test]
    async fn settings_round_trip(pool: sqlx::PgPool) {
        /* ─────────────── Arrange ─────────────── */
        let test_app = TestApp::new(pool.clone()).await;

        let (router, _) = crate::routes::settings::router(test_app.app.clone()).split_for_parts();
        
        // let settings = crate::routes::weather_location::services::
        //     theme: 0,
        //     notifications_enabled: true,
        //     radius: 10,
        // };
        // 
        // /* ─────────────── GET (default row) ─────────────── */
        // let res = router
        //     .clone()
        //     .oneshot(
        //         Request::builder()
        //             .method(Method::GET)
        //             .uri("/user/settings")
        //             .header("content-type", "application/json")
        //             .header("accept", "application/json")
        //             .body(Body::empty())
        //             .unwrap(),
        //     )
        //     .await
        //     .unwrap();
        // assert_eq!(res.status(), StatusCode::OK);
        // 
        //     let body = res.into_body().collect().await.unwrap().to_bytes();
        //     let settings: WapSettings = serde_json::from_slice(&body).unwrap();
        //     assert_eq!(settings.theme, 0); // default enum value
        //     assert_eq!(settings.radius, 10); // whatever your default is
        //     assert!(settings.notifications_enabled);
        // 
        //     /* ─────────────── PUT (update settings) ─────────────── */
        //     let payload = WapSettings {
        //         theme: 1,
        //         notifications_enabled: false,
        //         radius: 42,
        //     };
        //     let res = router
        //         .clone()
        //         .oneshot(
        //             Request::builder()
        //                 .method(Method::PUT)
        //                 .uri("/user/settings")
        //                 .header("content-type", "application/json")
        //                 .body(Body::from(serde_json::to_vec(&payload).unwrap()))
        //                 .unwrap(),
        //         )
        //         .await
        //         .unwrap();
        //     assert_eq!(res.status(), StatusCode::OK);
        // 
        //     /* ─────────────── GET again (should be updated) ─────────────── */
        //     let res = router
        //         .clone()
        //         .oneshot(with_user(
        //             Request::get("/user/settings").body(Body::empty()).unwrap(),
        //             &user,
        //         ))
        //         .await
        //         .unwrap();
        //     let body = res.into_body().collect().await.unwrap().to_bytes();
        //     let updated: WapSettings = serde_json::from_slice(&body).unwrap();
        //     assert_eq!(updated, payload);
        //
        //     /* ─────────────── GET for a user with no row (404) ─────────────── */
        //     let other_user = sqlx::query_as!(
        //         User,
        //         r#"INSERT INTO users (email, password)
        //            VALUES ('no_settings@example.com','hashed')
        //            RETURNING id, email, password, created_at, updated_at"#,
        //     )
        //     .fetch_one(&pool)
        //     .await
        //     .unwrap();
        //
        //     let res = router
        //         .oneshot(with_user(
        //             Request::get("/user/settings").body(Body::empty()).unwrap(),
        //             &other_user,
        //         ))
        //         .await
        //         .unwrap();
        //     assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }
}
