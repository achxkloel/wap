use crate::routes::weather_location::handlers::{
    UserId, WeatherLocation, WeatherLocationId, WeatherLocationService,
};
use sqlx::PgPool;

pub struct PgWeatherService {
    pool: PgPool,
}

impl PgWeatherService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl WeatherLocationService for PgWeatherService {
    async fn create(&self, location: WeatherLocation) -> anyhow::Result<WeatherLocationId> {
        let mut tx = self.pool.begin().await?;

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
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
