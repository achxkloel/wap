use crate::routes::natural_phenomenon_location::models::{
    CreateNaturalPhenomenonLocationRequest, ServiceCreateAndUpdateResponseSuccess,
    UpdateNaturalPhenomenonLocationRequestWithIds,
};
use crate::shared::models::DatabaseId;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait NaturalPhenomenonLocationService: Send + Sync + 'static {
    async fn create(
        &self,
        location: &CreateNaturalPhenomenonLocationRequest,
    ) -> Result<ServiceCreateAndUpdateResponseSuccess>;
    async fn get_all(
        &self,
        user_id: DatabaseId,
    ) -> Result<Vec<ServiceCreateAndUpdateResponseSuccess>>;
    async fn get_by_id(
        &self,
        user_id: DatabaseId,
        id: DatabaseId,
    ) -> Result<ServiceCreateAndUpdateResponseSuccess>;
    async fn update(
        &self,
        location: UpdateNaturalPhenomenonLocationRequestWithIds,
    ) -> Result<ServiceCreateAndUpdateResponseSuccess>;
    async fn delete(&self, user_id: DatabaseId, id: DatabaseId) -> Result<()>;
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
        location: &CreateNaturalPhenomenonLocationRequest,
    ) -> Result<ServiceCreateAndUpdateResponseSuccess> {
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
        Ok(ServiceCreateAndUpdateResponseSuccess {
            id: DatabaseId(rec.id),
            user_id: DatabaseId(rec.user_id),
            name: rec.name,
            latitude: rec.latitude,
            longitude: rec.longitude,
            description: Option::from(rec.description),
        })
    }

    async fn get_all(
        &self,
        user_id: DatabaseId,
    ) -> Result<Vec<ServiceCreateAndUpdateResponseSuccess>> {
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
        .map(|rec| ServiceCreateAndUpdateResponseSuccess {
            id: DatabaseId(rec.id),
            user_id: DatabaseId(rec.user_id),
            name: rec.name,
            latitude: rec.latitude,
            longitude: rec.longitude,
            description: Option::from(rec.description),
        })
        .collect();

        Ok(locations)
    }

    async fn get_by_id(
        &self,
        user_id: DatabaseId,
        id: DatabaseId,
    ) -> Result<ServiceCreateAndUpdateResponseSuccess> {
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

        Ok(ServiceCreateAndUpdateResponseSuccess {
            id: DatabaseId(rec.id),
            user_id: DatabaseId(rec.user_id),
            name: rec.name,
            latitude: rec.latitude,
            longitude: rec.longitude,
            description: Option::from(rec.description),
        })
    }

    async fn update(
        &self,
        location: UpdateNaturalPhenomenonLocationRequestWithIds,
    ) -> Result<ServiceCreateAndUpdateResponseSuccess> {
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

        Ok(ServiceCreateAndUpdateResponseSuccess {
            id: DatabaseId(record.id),
            user_id: DatabaseId(record.user_id),
            name: record.name,
            latitude: record.latitude,
            longitude: record.longitude,
            description: Option::from(record.description),
        })
    }

    async fn delete(&self, user_id: DatabaseId, id: DatabaseId) -> Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routes::natural_phenomenon_location::models::{
        CreateNaturalPhenomenonLocationRequest, ServiceCreateAndUpdateResponseSuccess,
        UpdateNaturalPhenomenonLocationRequest, UpdateNaturalPhenomenonLocationRequestWithIds,
    };
    use crate::routes::natural_phenomenon_location::services::PgNaturalPhenomenonLocationService;
    use crate::shared::models::DatabaseId;
    use crate::tests::tests::TestApp;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_natural_phenomenon_location_service_crud(pool: PgPool) {
        // Setup: initialize TestApp and service
        let test_app = TestApp::new(pool.clone()).await;
        let user_id: DatabaseId = test_app.users[0].user.id; // Copy newtype
        let service = PgNaturalPhenomenonLocationService::new(pool.clone());

        // 1) CREATE
        let create_req = CreateNaturalPhenomenonLocationRequest {
            user_id,
            name: "Grand Canyon".to_string(),
            latitude: 36.1069,
            longitude: -112.1129,
            description: Some("A famous canyon".to_string()),
        };
        let created: ServiceCreateAndUpdateResponseSuccess =
            service.create(&create_req).await.expect("create failed");
        assert_eq!(created.user_id, user_id);
        assert_eq!(created.name, create_req.name);
        assert_eq!(created.latitude, create_req.latitude);
        assert_eq!(created.longitude, create_req.longitude);
        assert_eq!(created.description, create_req.description);
        let id = created.id;

        // 2) GET_ALL
        let all: Vec<ServiceCreateAndUpdateResponseSuccess> =
            service.get_all(user_id).await.expect("get_all failed");
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].id, id);

        // 3) GET_BY_ID
        let fetched = service
            .get_by_id(user_id, id)
            .await
            .expect("get_by_id failed");
        assert_eq!(fetched.id, id);
        assert_eq!(fetched.user_id, user_id);

        // 4) UPDATE
        let update_req = UpdateNaturalPhenomenonLocationRequestWithIds {
            user_id,
            id,
            payload: UpdateNaturalPhenomenonLocationRequest {
                name: Some("Grand Canyon Updated".to_string()),
                latitude: Some(36.11),
                longitude: Some(-112.11),
                description: Some("Updated description".to_string()),
            },
        };
        let updated: ServiceCreateAndUpdateResponseSuccess = service
            .update(update_req.clone())
            .await
            .expect("update failed");
        assert_eq!(updated.id, id);
        assert_eq!(updated.user_id, user_id);
        assert_eq!(updated.name, update_req.payload.name.expect(&"name"));
        assert_eq!(
            updated.latitude,
            update_req.payload.latitude.expect(&"latitude")
        );
        assert_eq!(
            updated.longitude,
            update_req.payload.longitude.expect(&"longitude")
        );
        assert_eq!(updated.description, update_req.payload.description);

        // 5) DELETE
        service.delete(user_id, id).await.expect("delete failed");
        let remaining = service
            .get_all(user_id)
            .await
            .expect("get_all after delete failed");
        assert!(remaining.is_empty(), "Expected no remaining locations");
    }
}
