use crate::shared::models::DatabaseId;
use anyhow::Result;
use async_trait::async_trait;
use crate::routes::weather_location::models::{WeatherLocation, WeatherLocationId};

#[async_trait]
pub trait WeatherLocationServiceImpl: Clone + Send + Sync + 'static {
    async fn create(&self, location: WeatherLocation) -> Result<WeatherLocation>;
    async fn get_all(&self, user_id: &DatabaseId) -> Result<Vec<WeatherLocation>>;
    async fn get_by_id(
        &self,
        user_id: &DatabaseId,
        id: &WeatherLocationId,
    ) -> Result<WeatherLocation>;
    async fn update(&self, location: &WeatherLocation) -> Result<WeatherLocation>;
    async fn delete(&self, user_id: &DatabaseId, id: &WeatherLocationId) -> Result<()>;
}

#[derive(Clone)]
pub struct WeatherLocationService {
    pub db: sqlx::PgPool,
}

#[async_trait]
impl WeatherLocationServiceImpl for WeatherLocationService {
    async fn create(&self, location: WeatherLocation) -> Result<WeatherLocation> {
        let mut tx = self.db.begin().await?;

        if location.is_default {
            sqlx::query!(
                "UPDATE weather_locations SET is_default = false WHERE user_id = $1",
                location.user_id.0
            )
            .execute(&mut *tx)
            .await?;
        }

        let rec = sqlx::query!(
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
        Ok(WeatherLocation {
            id: Some(WeatherLocationId(rec.id)),
            ..location
        })
    }

    async fn get_all(&self, user_id: &DatabaseId) -> Result<Vec<WeatherLocation>> {
        let locations = sqlx::query!(
            r#"
            SELECT id, user_id, name, latitude, longitude, is_default, description
            FROM weather_locations
            WHERE user_id = $1
            "#,
            user_id.0
        )
        .fetch_all(&self.db)
        .await?
        .into_iter()
        .map(|rec| WeatherLocation {
            id: Some(WeatherLocationId(rec.id)),
            user_id: DatabaseId(rec.user_id),
            name: rec.name,
            latitude: rec.latitude,
            longitude: rec.longitude,
            is_default: rec.is_default,
            description: rec.description.into(),
        })
        .collect();

        Ok(locations)
    }

    async fn get_by_id(
        &self,
        user_id: &DatabaseId,
        id: &WeatherLocationId,
    ) -> Result<WeatherLocation> {
        let rec = sqlx::query!(
            r#"
            SELECT id, user_id, name, latitude, longitude, is_default, description
            FROM weather_locations
            WHERE id = $1 AND user_id = $2
            "#,
            id.0,
            user_id.0
        )
        .fetch_one(&self.db)
        .await?;

        Ok(WeatherLocation {
            id: Some(WeatherLocationId(rec.id)),
            user_id: DatabaseId(rec.user_id),
            name: rec.name,
            latitude: rec.latitude,
            longitude: rec.longitude,
            is_default: rec.is_default,
            description: rec.description.into(),
        })
    }

    async fn update(&self, location: &WeatherLocation) -> Result<WeatherLocation> {
        let mut tx = self.db.begin().await?;

        if location.is_default {
            sqlx::query!(
                "UPDATE weather_locations SET is_default = false WHERE user_id = $1",
                location.user_id.0
            )
            .execute(&mut *tx)
            .await?;
        }

        let rec = sqlx::query!(
            r#"
            UPDATE weather_locations
            SET name = $1, latitude = $2, longitude = $3, is_default = $4, description = $5
            WHERE id = $6 AND user_id = $7
            RETURNING id, user_id, name, latitude, longitude, is_default, description
            "#,
            location.name,
            location.latitude,
            location.longitude,
            location.is_default,
            location.description,
            location.id.unwrap().0,
            location.user_id.0,
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(WeatherLocation {
            id: Some(WeatherLocationId(rec.id)),
            user_id: DatabaseId(rec.user_id),
            name: rec.name,
            latitude: rec.latitude,
            longitude: rec.longitude,
            is_default: rec.is_default,
            description: rec.description.into(),
        })
    }

    async fn delete(&self, user_id: &DatabaseId, id: &WeatherLocationId) -> Result<()> {
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
