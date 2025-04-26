use crate::routes::weather_locations::models::{CreateWeatherLocationRequest, WeatherLocation};
use crate::shared::models::DatabaseId;
use anyhow::Result;
use async_trait::async_trait;

/// Defines the core CRUD operations for managing weather‐location records.
///
/// Implementors must handle creation, retrieval, updates, and deletion
/// of `WeatherLocation` entries, scoped to a specific user.
#[async_trait]
pub trait WeatherLocationServiceImpl: Clone + Send + Sync + 'static {
    /// Insert a new weather location for the given user.
    ///
    /// If `location.is_default` is `true`, any existing default for the user must be unset.
    ///
    /// Returns the freshly‐created `WeatherLocation` with all its fields populated.
    async fn create(&self, location: &CreateWeatherLocationRequest) -> Result<WeatherLocation>;

    /// Fetch all weather locations belonging to `user_id`.
    ///
    /// Returns a vector of `WeatherLocation`. If none exist, returns an empty `Vec`.
    async fn get_all(&self, user_id: &DatabaseId) -> Result<Vec<WeatherLocation>>;

    /// Fetch a single weather location by its `id` for the given user.
    ///
    /// Returns `Ok(WeatherLocation)` if found, or an error if not found or on failure.
    async fn get_by_id(&self, user_id: &DatabaseId, id: &DatabaseId) -> Result<WeatherLocation>;

    /// Update an existing weather location.
    ///
    /// The provided `location` struct must contain the `id` and `user_id` of the record
    /// to update. If `location.is_default` is `true`, any previous default for that user
    /// will be unset.
    ///
    /// Returns the updated `WeatherLocation`.
    async fn update(&self, location: &WeatherLocation) -> Result<WeatherLocation>;

    /// Delete the weather location with the given `id` for the specified user.
    ///
    /// Returns `Ok(())` on success, or an error if the record did not exist or the
    /// deletion failed.
    async fn delete(&self, user_id: &DatabaseId, id: &DatabaseId) -> Result<()>;
}

/// Postgres‐backed implementation of `WeatherLocationServiceImpl`.
///
/// Uses SQLx to run all queries in a `PgPool`. Handles transactional logic
/// when toggling the `is_default` flag.
#[derive(Clone)]
pub struct WeatherLocationService {
    /// The SQLx Postgres connection pool.
    pub db: sqlx::PgPool,
}

#[async_trait]
impl WeatherLocationServiceImpl for WeatherLocationService {
    async fn create(&self, location: &CreateWeatherLocationRequest) -> Result<WeatherLocation> {
        let mut tx = self.db.begin().await?;

        if location.is_default {
            sqlx::query!(
                "UPDATE weather_locations SET is_default = false WHERE user_id = $1",
                location.user_id.0
            )
            .execute(&mut *tx)
            .await?;
        }

        // This query now RETURNING all the columns that map to WeatherLocation
        let rec = sqlx::query_as!(
            WeatherLocation,
            r#"
            INSERT INTO weather_locations (user_id, name, latitude, longitude, is_default, description)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
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
        Ok(rec)
    }

    async fn get_all(&self, user_id: &DatabaseId) -> Result<Vec<WeatherLocation>> {
        let locations = sqlx::query_as!(
            WeatherLocation,
            r#"
            SELECT *
            FROM weather_locations
            WHERE user_id = $1
            "#,
            user_id.0
        )
        .fetch_all(&self.db)
        .await?;

        Ok(locations)
    }

    async fn get_by_id(&self, user_id: &DatabaseId, id: &DatabaseId) -> Result<WeatherLocation> {
        let rec = sqlx::query_as!(
            WeatherLocation,
            r#"
            SELECT *
            FROM weather_locations
            WHERE id = $1 AND user_id = $2
            "#,
            id.0,
            user_id.0
        )
        .fetch_one(&self.db)
        .await?;

        Ok(rec)
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

        let rec = sqlx::query_as!(
            WeatherLocation,
            r#"
            UPDATE weather_locations
            SET name = $1, latitude = $2, longitude = $3, is_default = $4, description = $5
            WHERE id = $6 AND user_id = $7
            RETURNING *
            "#,
            location.name,
            location.latitude,
            location.longitude,
            location.is_default,
            location.description,
            location.id.0,
            location.user_id.0,
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(rec)
    }

    async fn delete(&self, user_id: &DatabaseId, id: &DatabaseId) -> Result<()> {
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
    use crate::routes::weather_locations::services::WeatherLocationService;
    use crate::shared::models::DatabaseId;
    use crate::tests::tests::TestApp;
    use sqlx::{Executor, PgPool};

    #[sqlx::test]
    async fn test_weather_location_service(pool: PgPool) {
        let test_app = TestApp::new(pool.clone()).await;

        // Clean slate: weather_locations, settings, users
        test_app
            .app
            .db
            .execute("DELETE FROM weather_locations")
            .await
            .unwrap();
        test_app
            .app
            .db
            .execute("DELETE FROM settings")
            .await
            .unwrap();
        test_app.app.db.execute("DELETE FROM users").await.unwrap();

        // Create a fake user
        let user_rec = sqlx::query!(
            "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id",
            "user@example.com",
            "pass"
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        let user_id = DatabaseId(user_rec.id);

        let svc = WeatherLocationService {
            db: test_app.app.db.clone(),
        };

        // 1) create non-default location
        let location1 = CreateWeatherLocationRequest {
            user_id: user_id.clone(),
            name: "Home".into(),
            latitude: 10.0,
            longitude: 20.0,
            is_default: false,
            description: "Home sweet home".into(),
        };
        let created1 = svc.create(&location1).await.unwrap();
        assert_eq!(created1.name, location1.name);
        assert!(!created1.is_default);
        let id1 = created1.id;

        // 2) get_all should return exactly one
        let all1 = svc.get_all(&user_id).await.unwrap();
        assert_eq!(all1.len(), 1);
        assert_eq!(all1[0].id, id1);

        // 3) get_by_id should return the same
        let fetched1 = svc.get_by_id(&user_id, &id1).await.unwrap();
        assert_eq!(fetched1.id, id1);

        // 4) create a default location, which should unset previous defaults
        let location2 = CreateWeatherLocationRequest {
            user_id: user_id.clone(),
            name: "Office".into(),
            latitude: 30.0,
            longitude: 40.0,
            is_default: true,
            description: "Workplace".into(),
        };
        let created2 = svc.create(&location2).await.unwrap();
        assert!(created2.is_default);
        let id2 = created2.id;

        let all2 = svc.get_all(&user_id).await.unwrap();
        // Only one default location should exist
        let defaults: Vec<_> = all2.iter().filter(|l| l.is_default).collect();
        assert_eq!(defaults.len(), 1);
        assert_eq!(defaults[0].id, id2);

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
        assert_eq!(defaults3[0].id, updated1.id);

        // 6) delete the updated location
        svc.delete(&user_id, &updated1.id).await.unwrap();
        let all4 = svc.get_all(&user_id).await.unwrap();
        assert_eq!(all4.len(), 1);
        assert_eq!(all4[0].id, id2);

        // 7) get_by_id for non-existent should error
        let err = svc.get_by_id(&user_id, &DatabaseId(9999)).await;
        assert!(err.is_err());
    }
}
