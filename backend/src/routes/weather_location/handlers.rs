// domain/user.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ToSchema)]
pub struct UserId(pub i32);

// domain/weather_location.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ToSchema)]
pub struct WeatherLocationId(pub i32);

#[derive(Debug, Clone, ToSchema)]
pub struct WeatherLocation {
    pub id: Option<WeatherLocationId>, // None for new entities
    pub user_id: UserId,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub is_default: bool,
    pub description: Option<String>,
}

struct WeatherLocationAdapter {
    pub db: sqlx::PgPool,
}

#[async_trait::async_trait]
pub trait WeatherLocationService: Send + Sync + 'static {
    async fn new(db: sqlx::PgPool) -> Self;
    async fn create(&self, location: WeatherLocation) -> anyhow::Result<WeatherLocationId>;
    async fn delete(&self, user_id: UserId, id: WeatherLocationId) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
impl WeatherLocationService for WeatherLocationAdapter {
    async fn new(db: sqlx::PgPool) -> Self
    where
        Self: Sized,
    {
        Self { db }
    }

    async fn create(&self, location: WeatherLocation) -> anyhow::Result<WeatherLocationId> {
        let mut tx = self.db.begin().await?;

        // If setting this one as default, unset previous default for the user
        if location.is_default {
            sqlx::query!(
                "UPDATE weather_locations SET is_default = false WHERE user_id = $1",
                location.user_id.0
            )
            .execute(&mut *tx)
            .await?;
        }

        let record = sqlx::query!(
            r#"
            INSERT INTO weather_locations (user_id, name, latitude, longitude, is_default, description)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#,
            location.user_id.0,
            location.name,
            location.latitude,
            location.longitude,
            location.is_default,
            location.description,
        )
            .fetch_one(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(WeatherLocationId(record.id))
    }

    async fn delete(&self, user_id: UserId, id: WeatherLocationId) -> anyhow::Result<()> {
        sqlx::query!(
            "DELETE FROM weather_locations WHERE id = $1 AND user_id = $2",
            id.0,
            user_id.0
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }
}

use crate::shared::models::AppState;
use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use utoipa::ToSchema;

#[derive(Debug, Deserialize)]
pub struct CreateWeatherLocationRequest {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub is_default: bool,
    pub description: Option<String>,
}

#[utoipa::path(
    post,
    path = "/weather_locations",
    responses(
        (status = 201, description = "Location created", body = WeatherLocation),
    )
)]
pub async fn create_weather_location(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
    Json(payload): Json<CreateWeatherLocationRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let location = WeatherLocation {
        id: None,
        user_id: UserId(user_id),
        name: payload.name,
        latitude: payload.latitude,
        longitude: payload.longitude,
        is_default: payload.is_default,
        description: payload.description,
    };

    WeatherLocationAdapter::new(state.db.clone())
        .await
        .create(location)
        .await
        .map(|_| (StatusCode::CREATED, "Location created"))
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
        })
}

#[utoipa::path(
    method(delete),
    path = "/weather_location/{id}",
    responses(
        (status = 204, description = "Weather location deleted", body = str)
    ),
    params(
        ("id" = i32, Path, description = "Weather Location id")
    ),
)]
pub async fn delete_weather_location(
    State(state): State<Arc<AppState>>,
    Path((user_id, location_id)): Path<(i32, i32)>,
) -> Result<StatusCode, StatusCode> {
    WeatherLocationAdapter::new(state.db.clone())
        .await
        .delete(UserId(user_id), WeatherLocationId(location_id))
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
