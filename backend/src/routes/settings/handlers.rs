use crate::routes::auth::models::UserDb;
use crate::routes::settings::models::{UserSettingsServiceSuccess, UserSettingsUpdateRequest};
use crate::routes::settings::services::{PgSettingsService, SettingsService};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    Extension,
};
use std::sync::Arc;
use tracing::error;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use crate::routes::auth::middlewares::auth;
use crate::routes::auth::services::AuthService;
use crate::shared::models::AppState;

#[utoipa::path(
    method(put),
    path = "/user/settings",
    request_body = UserSettingsUpdateRequest,
    responses(
        (status = 200, description = "Settings updated"),
        (status = 400, description = "Bad request")
    )
)]
pub async fn put_settings<S>(
    State(service): State<Arc<S>>,
    Extension(user): Extension<UserDb>,
    Json(payload): Json<UserSettingsUpdateRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
where
    S: SettingsService,
{
    service
        .update_settings(&user.id, &payload)
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
        (status = 200, description = "User settings returned", body = UserSettingsServiceSuccess),
        (status = 404, description = "No settings found for user"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_settings<S>(
    Extension(user): Extension<UserDb>,
    State(service): State<Arc<S>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
where
    S: SettingsService,
{
    let settings = service.get_settings(&user.id).await.map_err(|err| {
        tracing::error!("Error in service.get_settings: {:?}", err);
        let json =
            serde_json::json!({ "error": format!("Error in service.get_settings: {}", err) });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json))
    })?;

    match settings {
        Some(s) => {
            tracing::debug!("User settings: {:?}", s);
            Ok((StatusCode::OK, Json(s)))
        }
        None => {
            tracing::error!("Settings not found for user: {:?}", user.id);
            let json = serde_json::json!({
                "status": "fail",
                "message": "Settings not found for user"
            });
            Err((StatusCode::NOT_FOUND, Json(json)))
        }
    }
}

/// Fully‐generic: you supply the `service: S`.
pub fn router_with_service<S>(app: AppState, service: Arc<S>) -> OpenApiRouter
where
    S: SettingsService + Send + Sync + 'static,
{
    let auth_service = Arc::new(AuthService { db: app.db.clone(), settings: app.settings.clone(), http: Default::default() });
    OpenApiRouter::new()
        .routes(routes!(get_settings))
        .routes(routes!(put_settings))
        .layer(axum::middleware::from_fn_with_state(auth_service, auth))
        .with_state(service)
}

/// A convenience wrapper that uses the real Postgres implementation.
pub fn router(app: AppState) -> OpenApiRouter {
    let settings_service = Arc::new(PgSettingsService::new(app.db.clone(), app.settings.clone()));
    router_with_service(app.clone(), settings_service)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routes::settings::models::{Theme, UserSettingsServiceSuccess};
    use crate::routes::settings::handlers;
    use crate::routes::settings::services::MockSettingsService;
    use crate::shared::models::DatabaseId;
    use crate::tests::tests::TestApp;
    use axum::{
        body::Body,
        http,
        http::{Method, Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use jsonwebtoken::Header;
    use mockall::mock;
    use mockall::predicate::*;
    use sqlx::PgPool;
    use std::sync::Arc;
    use tower::ServiceExt;
    use tracing_test::traced_test;

    #[sqlx::test]
    #[traced_test]
    async fn test_get_settings(pool: PgPool) {
        let test_app = TestApp::new(pool).await;
        let user_id: DatabaseId = test_app.users[0].user.id; // no `.clone()` needed
        let access_token = test_app.users[0].tokens.access_token.clone();
        tracing::debug!("access_token: {:?}", access_token);

        let payload = UserSettingsServiceSuccess {
            user_id,
            theme: Theme::Dark,
            notifications_enabled: true,
            radius: 10,
        };

        // let mut mock = MockSettingsService::new();
        let mut mock = MockSettingsService::new();
        mock.expect_get_settings()
            .with(eq(user_id))
            .times(1)
            .returning(move |_| {
                // `user_id` is `Copy`, so this copies, doesn't consume:
                let id = user_id;
                Box::pin(async move { Ok(Some(payload.clone())) })
            });

        // let (router, _) = routers::router_with_service(test_app.app.clone(), mock).split_for_parts();
        let arc_mocked_service = Arc::new(mock);
        let (router, _) = router_with_service(test_app.app.clone(), arc_mocked_service)
            .split_for_parts();
        let request = Request::builder()
            .method(Method::GET)
            .uri("/user/settings")
            .header(
                http::header::AUTHORIZATION,
                format!("Bearer {}", access_token),
            )
            .body(Body::empty())
            .unwrap();

        tracing::debug!("request: {:?}", request);

        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // parse the response body and check content against the mocked data
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let refresh_body: UserSettingsServiceSuccess =
            serde_json::from_slice(&body).expect("error");

        assert_eq!(refresh_body.user_id, user_id);
        assert_eq!(refresh_body.theme, Theme::Dark);
        assert_eq!(refresh_body.notifications_enabled, true);
        assert_eq!(refresh_body.radius, 10);
        tracing::debug!("response: {:?}", refresh_body);
    }

    #[sqlx::test]
    #[traced_test]
    async fn test_put_settings(pool: PgPool) {
        // 1) spin up the app & extract user info/token
        let test_app = TestApp::new(pool).await;
        let user_id: DatabaseId = test_app.users[0].user.id; // Copy newtype
        let token = test_app.users[0].tokens.access_token.clone();

        // 2) prepare a dummy payload
        let payload = UserSettingsUpdateRequest {
            theme: Theme::Light,
            notifications_enabled: false,
            radius: 20,
        };

        // 3) mock & expect the service.update_settings call
        let mut mock = MockSettingsService::new();
        mock.expect_update_settings()
            .with(eq(user_id), eq(payload))
            .times(1)
            .returning(move |_, _| Box::pin(async move { Ok(()) }));

        // 4) build a router that uses our mock
        let service = Arc::new(mock);
        let (router, _) =
            router_with_service(test_app.app.clone(), service).split_for_parts();

        // 5) serialize payload and assemble the PUT request
        let body = serde_json::to_string(&payload).unwrap();
        let request = Request::builder()
            .method(Method::PUT)
            .uri("/user/settings")
            .header(http::header::AUTHORIZATION, format!("Bearer {}", token))
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(Body::from(body))
            .unwrap();

        // 6) dispatch and inspect the response
        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // (optionally) verify the response body
        let txt = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&txt[..], b"Settings saved successfully");
    }
}

