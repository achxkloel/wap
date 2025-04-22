use crate::routes::weather_locations::models::{WeatherLocation, WeatherLocationId};
use crate::shared::models::DatabaseId;
use anyhow::Result;
use async_trait::async_trait;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routes::weather_locations::models::{WeatherLocation, WeatherLocationId};
    use crate::routes::weather_locations::services::WeatherLocationService;
    use crate::shared::models::DatabaseId;
    use crate::tests::tests::TestApp;
    use sqlx::{Executor, PgPool};

    #[sqlx::test]
    async fn test_weather_location_service(pool: PgPool) {
        let test_app = TestApp::new(pool.clone()).await;

        // Clean slate: weather_locations, settings, users
        test_app.app.db.execute("DELETE FROM weather_locations").await.unwrap();
        test_app.app.db.execute("DELETE FROM settings").await.unwrap();
        test_app.app.db.execute("DELETE FROM users").await.unwrap();

        // Create a dummy user
        let user_rec = sqlx::query!(
            "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id",
            "user@example.com",
            "pass"
        )
            .fetch_one(&pool)
            .await
            .unwrap();
        let user_id = DatabaseId(user_rec.id);

        let svc = WeatherLocationService { db: test_app.app.db.clone() };

        // 1) create non-default location
        let location1 = WeatherLocation {
            id: None,
            user_id: user_id.clone(),
            name: "Home".into(),
            latitude: 10.0,
            longitude: 20.0,
            is_default: false,
            description: Some("Home sweet home".into()),
        };
        let created1 = svc.create(location1.clone()).await.unwrap();
        assert_eq!(created1.name, location1.name);
        assert!(!created1.is_default);
        let id1 = created1.id.unwrap();

        // 2) get_all should return exactly one
        let all1 = svc.get_all(&user_id).await.unwrap();
        assert_eq!(all1.len(), 1);
        assert_eq!(all1[0].id.unwrap(), id1);

        // 3) get_by_id should return the same
        let fetched1 = svc.get_by_id(&user_id, &id1).await.unwrap();
        assert_eq!(fetched1.id.unwrap(), id1);

        // 4) create a default location, which should unset previous defaults
        let location2 = WeatherLocation {
            id: None,
            user_id: user_id.clone(),
            name: "Office".into(),
            latitude: 30.0,
            longitude: 40.0,
            is_default: true,
            description: Some("Workplace".into()),
        };
        let created2 = svc.create(location2.clone()).await.unwrap();
        assert!(created2.is_default);
        let id2 = created2.id.unwrap();

        let all2 = svc.get_all(&user_id).await.unwrap();
        // Only one default location should exist
        let defaults: Vec<_> = all2.iter().filter(|l| l.is_default).collect();
        assert_eq!(defaults.len(), 1);
        assert_eq!(defaults[0].id.unwrap(), id2);

        // 5) update the first location to become default
        let mut to_update = created1.clone();
        to_update.name = "Home Updated".into();
        to_update.is_default = true;
        let updated1 = svc.update(&to_update).await.unwrap();
        assert_eq!(updated1.name, "Home Updated");
        assert!(updated1.is_default);

        let all3 = svc.get_all(&user_id).await.unwrap();
        let defaults3: Vec<_> = all3.iter().filter(|l| l.is_default).collect();
        assert_eq!(defaults3.len(), 1);
        assert_eq!(defaults3[0].id.unwrap(), updated1.id.unwrap());

        // 6) delete the updated location
        svc.delete(&user_id, &updated1.id.unwrap()).await.unwrap();
        let all4 = svc.get_all(&user_id).await.unwrap();
        assert_eq!(all4.len(), 1);
        assert_eq!(all4[0].id.unwrap(), id2);

        // 7) get_by_id for non-existent should error
        let err = svc.get_by_id(&user_id, &WeatherLocationId(9999)).await;
        assert!(err.is_err());
    }
}
