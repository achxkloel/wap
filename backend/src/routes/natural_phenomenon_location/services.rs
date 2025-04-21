use crate::routes::natural_phenomenon_location::domains::*;
use crate::routes::natural_phenomenon_location::models::UpdateNaturalPhenomenonLocationRequestWithIds;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use crate::shared::models::DatabaseId;

#[async_trait]
pub trait NaturalPhenomenonLocationService: Send + Sync + 'static {
    async fn create(
        &self,
        location: NaturalPhenomenonLocation,
    ) -> Result<NaturalPhenomenonLocation>;
    async fn get_all(&self, user_id: DatabaseId) -> Result<Vec<NaturalPhenomenonLocation>>;
    async fn get_by_id(
        &self,
        user_id: DatabaseId,
        id: NaturalPhenomenonLocationId,
    ) -> Result<NaturalPhenomenonLocation>;
    async fn update(
        &self,
        location: UpdateNaturalPhenomenonLocationRequestWithIds,
    ) -> Result<NaturalPhenomenonLocation>;
    async fn delete(&self, user_id: DatabaseId, id: NaturalPhenomenonLocationId) -> Result<()>;
}

pub struct PgNaturalPhenomenonLocationService {
    db: PgPool,
}

impl PgNaturalPhenomenonLocationService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl NaturalPhenomenonLocationService for PgNaturalPhenomenonLocationService {
    async fn create(
        &self,
        location: NaturalPhenomenonLocation,
    ) -> Result<NaturalPhenomenonLocation> {
        let rec = sqlx::query!(
            r#"
            INSERT INTO natural_phenomenon_locations (user_id, name, latitude, longitude, description)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, user_id, name, latitude, longitude, description
            "#,
            location.user_id.0,
            location.name,
            location.latitude,
            location.longitude,
            location.description,
        )
            .fetch_one(&self.db)
            .await?;

        // Build a *fresh* domain object from the row we just got back
        Ok(NaturalPhenomenonLocation {
            id: Some(NaturalPhenomenonLocationId(rec.id)),
            user_id: DatabaseId(rec.user_id),
            name: rec.name,
            latitude: rec.latitude,
            longitude: rec.longitude,
            description: rec.description,
        })
    }

    async fn get_all(&self, user_id: DatabaseId) -> Result<Vec<NaturalPhenomenonLocation>> {
        let locations = sqlx::query!(
            r#"
                SELECT id, user_id, name, latitude, longitude, description
                FROM natural_phenomenon_locations
                WHERE user_id = $1
                "#,
            user_id.0
        )
        .fetch_all(&self.db)
        .await?
        .into_iter()
        .map(|rec| NaturalPhenomenonLocation {
            id: Some(NaturalPhenomenonLocationId(rec.id)),
            user_id: DatabaseId(rec.user_id),
            name: rec.name,
            latitude: rec.latitude,
            longitude: rec.longitude,
            description: rec.description,
        })
        .collect();

        Ok(locations)
    }

    async fn get_by_id(
        &self,
        user_id: DatabaseId,
        id: NaturalPhenomenonLocationId,
    ) -> Result<NaturalPhenomenonLocation> {
        let rec = sqlx::query!(
            r#"
                SELECT id, user_id, name, latitude, longitude, description
                FROM natural_phenomenon_locations
                WHERE id = $1 AND user_id = $2
                "#,
            id.0,
            user_id.0
        )
        .fetch_one(&self.db)
        .await?;

        Ok(NaturalPhenomenonLocation {
            id: Some(NaturalPhenomenonLocationId(rec.id)),
            user_id: DatabaseId(rec.user_id),
            name: rec.name,
            latitude: rec.latitude,
            longitude: rec.longitude,
            description: rec.description,
        })
    }

    async fn update(
        &self,
        location: UpdateNaturalPhenomenonLocationRequestWithIds,
    ) -> Result<NaturalPhenomenonLocation> {
        let record = sqlx::query!(
            r#"
            UPDATE natural_phenomenon_locations
            SET name = $1, latitude = $2, longitude = $3, description = $4
            WHERE id = $5 AND user_id = $6
            RETURNING id, user_id, name, latitude, longitude, description
            "#,
            location.payload.name,
            location.payload.latitude,
            location.payload.longitude,
            location.payload.description,
            location.id.0,
            location.user_id.0,
        )
        .fetch_one(&self.db)
        .await?;

        Ok(NaturalPhenomenonLocation {
            id: Some(NaturalPhenomenonLocationId(record.id)),
            user_id: DatabaseId(record.user_id),
            name: record.name,
            latitude: record.latitude,
            longitude: record.longitude,
            description: record.description,
        })
    }

    async fn delete(&self, user_id: DatabaseId, id: NaturalPhenomenonLocationId) -> Result<()> {
        sqlx::query!(
            r#"
                DELETE FROM natural_phenomenon_locations
                WHERE id = $1 AND user_id = $2
                "#,
            id.0,
            user_id.0
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }
}
