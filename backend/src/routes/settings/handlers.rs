use crate::routes::auth::models::User;
use crate::routes::weather_location::models::{UserSettings, UserSettingsUpdateRequest};
use crate::routes::settings::services::{SettingsService, PgSettingsService};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    Extension,
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
    State(service): State<PgSettingsService>,
    Extension(user): Extension<User>,
    Json(payload): Json<UserSettingsUpdateRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.update_settings(user.id, payload)
        .await
        .map_err(|err| {
            let msg = serde_json::json!({ "error": format!("Failed to update settings: {}", err) });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(msg))
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
    Extension(user): Extension<User>,
    State(service): State<PgSettingsService>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let settings = service.get_settings(user.id)
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
    use super::*;
    use crate::tests::tests::TestApp;
    use axum::{
        body::Body,
        http::{Method, Request, StatusCode},
    };
    use tower::ServiceExt;
    use mockall::{mock, predicate::*};
    use crate::routes::settings::services::MockSettingsService;
    use crate::routes::weather_location::models::{Theme, UserSettingsServiceSuccess};
    use anyhow::Result;
    use sqlx::PgPool;
    use crate::routes::settings::routers;

    /// Helper that injects a `User` into the request's extensions.
    // fn with_user<B>(mut req: Request<B>, user: &User) -> Request<B> {
    //     req.extensions_mut().insert(user.clone());
    //     req
    // }

    #[sqlx::test]
    async fn test_get_settings(pool: PgPool) {
        // 1) spin up TestApp
        let test_app = TestApp::new(pool).await;

        // 2) extract only the values we’ll need later
        let user_id      = test_app.users[0].user.id.clone();
        let access_token = test_app.users[0].tokens.access_token.clone();

        // 3) prepare the mock so it only closes over `user_id`
        let mut mock = MockSettingsService::new();
        mock.expect_get_settings()
            .with(eq(user_id))
            .times(1)
            .returning(move |_| {
                Box::pin(async move {
                    Ok(Some(UserSettingsServiceSuccess {
                        user_id,
                        theme: Theme::Dark,
                        notifications_enabled: true,
                        radius: 10,
                    }))
                })
            });

        // 4) build your router (using the real app scaffolding, but injecting our mock)
        //    if your `routers::router` takes the service as state, swap it in here.
        let (router, _) = routers::router(test_app.app.clone()).split_for_parts();

        // 5) build the request using the extracted token
        let request = Request::builder()
            .method(Method::GET)
            .uri("/user/settings")
            .header("Authorization", format!("Bearer {}", access_token))
            .body(Body::empty())
            .unwrap();

        // 6) fire and **use** the response
        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    // #[sqlx::test]
    // async fn test_get_settings(pool: PgPool) {
    //     let test_app = TestApp::new(pool).await;
    // 
    //     let mut mock = MockSettingsService::new();
    //     mock.expect_get_settings()
    //         .with(eq(test_app.users[0].user.id))
    //         .times(1)
    //         .returning(move |_| {
    //             // Box the async block so it implements Future
    //             let foo = test_app.clone();
    //             Box::pin(async move {
    //                 Ok(Some(UserSettingsServiceSuccess {
    //                     user_id: foo.users[0].user.id.clone(),
    //                     theme: Theme::Dark,
    //                     notifications_enabled: true,
    //                     radius: 10,
    //                 }))
    //             })
    //         });
    // 
    //     let request = Request::builder()
    //         .method(Method::GET)
    //         .uri("/user/settings")
    //         .header("Authorization", format!("Bearer {}",test_app.clone().users[0].tokens.access_token))
    //         .body(Body::empty())
    //         .unwrap();
    // 
    //     let (router, _) = routers::router(test_app.app).split_for_parts();
    //     let response = router.oneshot(request).await.unwrap();
    // }
}