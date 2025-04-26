use crate::routes::natural_phenomenon_locations::models::{
    CreateAndUpdateResponseSuccess, CreateNaturalPhenomenonLocationInnerWithImage,
    CreateNaturalPhenomenonLocationRequest, GetAllNaturalPhenomenonLocationResponseSuccess,
    GetByIdNaturalPhenomenonLocationResponseSuccess, NaturalPhenomenonLocationDb,
    NaturalPhenomenonLocationError, NaturalPhenomenonLocationResponseSuccess,
    PostNaturalPhenomenonLocationService, ServiceCreateAndUpdateResponseSuccess,
    UpdateNaturalPhenomenonLocationRequestWithIds, UpdateNaturalPhenomenonLocationResponseSuccess,
};
use crate::shared::models::DatabaseId;
use anyhow::Result;
use async_trait::async_trait;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use sanitize_filename::sanitize_with_options;
use sqlx::PgPool;
use tokio::fs;
use uuid::Uuid;

#[async_trait]
pub trait NaturalPhenomenonLocationServiceImpl: Send + Sync + 'static {
    async fn create(
        &self,
        req: PostNaturalPhenomenonLocationService,
    ) -> Result<CreateAndUpdateResponseSuccess, (StatusCode, Json<NaturalPhenomenonLocationError>)>;
    async fn get_all(
        &self,
        user_id: DatabaseId,
    ) -> Result<
        Vec<GetAllNaturalPhenomenonLocationResponseSuccess>,
        (StatusCode, Json<NaturalPhenomenonLocationError>),
    >;

    async fn get_by_id(
        &self,
        user_id: DatabaseId,
        id: DatabaseId,
    ) -> Result<
        GetByIdNaturalPhenomenonLocationResponseSuccess,
        (StatusCode, Json<NaturalPhenomenonLocationError>),
    >;
    async fn update(
        &self,
        location: UpdateNaturalPhenomenonLocationRequestWithIds,
    ) -> Result<crate::routes::natural_phenomenon_locations::models::UpdateNaturalPhenomenonLocationResponseSuccess, (StatusCode, Json<crate::routes::natural_phenomenon_locations::models::NaturalPhenomenonLocationError>)>;

    /// Delete the row *and* its on-disk image (if any).
    async fn delete(
        &self,
        user_id: DatabaseId,
        id: DatabaseId,
    ) -> Result<
        (StatusCode, Json<NaturalPhenomenonLocationResponseSuccess>),
        (StatusCode, Json<NaturalPhenomenonLocationError>),
    >;}

pub struct NaturalPhenomenonLocationService {
    db: PgPool,
}

impl NaturalPhenomenonLocationService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl NaturalPhenomenonLocationServiceImpl for NaturalPhenomenonLocationService {
    async fn create(
        &self,
        req: PostNaturalPhenomenonLocationService,
    ) -> Result<CreateAndUpdateResponseSuccess, (StatusCode, Json<NaturalPhenomenonLocationError>)>
    {
        // 1) save image to disk
        let safe = sanitize_with_options(&req.image_filename, Default::default());
        let filename = format!("{}_{}", Uuid::new_v4(), safe);
        let path = format!("uploads/{}", filename);
        fs::create_dir_all("uploads").await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(NaturalPhenomenonLocationError::DatabaseError),
            )
        })?;
        fs::write(&path, &req.image_bytes).await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(NaturalPhenomenonLocationError::DatabaseError),
            )
        })?;

        // 2) insert into DB (including image_url column!)
        let rec = sqlx::query_as!(
            NaturalPhenomenonLocationDb,
            r#"
            INSERT INTO natural_phenomenon_locations
                (user_id, name, latitude, longitude, description, image_path, radius)
            VALUES ($1,$2,$3,$4,$5,$6,$7)
            RETURNING *
            "#,
            req.user_id.0,
            req.name,
            req.latitude,
            req.longitude,
            req.description,
            path, // store the file path or a URL base + path
            req.radius,
        )
        .fetch_one(&self.db)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(NaturalPhenomenonLocationError::DatabaseError),
            )
        })?;

        // 3) build your success DTO
        Ok(CreateAndUpdateResponseSuccess {
            id: rec.id,
            user_id: rec.user_id,
            name: rec.name,
            latitude: rec.latitude,
            longitude: rec.longitude,
            description: rec.description,
            image_path: path,
            radius: rec.radius,
            created_at: rec.created_at,
            updated_at: rec.updated_at,
        })
    }

    async fn get_all(
        &self,
        user_id: DatabaseId,
    ) -> Result<
        Vec<GetAllNaturalPhenomenonLocationResponseSuccess>,
        (StatusCode, Json<NaturalPhenomenonLocationError>),
    > {
        // 1) try to fetch all rows, and map any SQL error into our tuple
        let records = sqlx::query_as!(
            NaturalPhenomenonLocationDb,
            r#"
                SELECT *
                FROM natural_phenomenon_locations
                WHERE user_id = $1
            "#,
            user_id.0
        )
        .fetch_all(&self.db)
        .await
        .map_err(|e| {
            tracing::error!("Error fetching locations: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(NaturalPhenomenonLocationError::DatabaseError),
            )
        })?;

        // 2) now map the Vec<NaturalPhenomenonLocationDb> into our response DTOs
        let locations = records
            .into_iter()
            .map(|rec| GetAllNaturalPhenomenonLocationResponseSuccess {
                id: rec.id,
                user_id: rec.user_id,
                name: rec.name,
                latitude: rec.latitude,
                longitude: rec.longitude,
                description: rec.description,
                radius: rec.radius,
                image_path: rec.image_path,
            })
            .collect();

        Ok(locations)
    }

    async fn get_by_id(
        &self,
        user_id: DatabaseId,
        id: DatabaseId,
    ) -> Result<
        GetByIdNaturalPhenomenonLocationResponseSuccess,
        (StatusCode, Json<NaturalPhenomenonLocationError>),
    > {
        let rec = sqlx::query_as!(
            NaturalPhenomenonLocationDb,
            r#"
                SELECT *
                FROM natural_phenomenon_locations
                WHERE id = $1 AND user_id = $2
                "#,
            id.0,
            user_id.0
        )
        .fetch_one(&self.db)
        .await
        .map_err(|_| {
            tracing::error!("Error fetching location by ID: {:?}", id);
            (
                StatusCode::NOT_FOUND,
                Json(NaturalPhenomenonLocationError::NotFound),
            )
        })?;

        Ok(GetByIdNaturalPhenomenonLocationResponseSuccess {
            id: rec.id,
            user_id: rec.user_id,
            name: rec.name,
            latitude: rec.latitude,
            longitude: rec.longitude,
            description: rec.description,
            image_path: rec.image_path,
        })
    }

    async fn update(
        &self,
        location: UpdateNaturalPhenomenonLocationRequestWithIds,
    ) -> Result<
        UpdateNaturalPhenomenonLocationResponseSuccess,
        (StatusCode, Json<NaturalPhenomenonLocationError>),
    > {
        let record = sqlx::query_as!(
            NaturalPhenomenonLocationDb,
            r#"
        UPDATE natural_phenomenon_locations
        SET
            name        = COALESCE($1, name),
            latitude    = COALESCE($2, latitude),
            longitude   = COALESCE($3, longitude),
            description = COALESCE($4, description)
        WHERE id = $5 AND user_id = $6
        RETURNING *
        "#,
            // these are Option<...>, so COALESCE will pick the existing value when None:
            location.payload.name,
            location.payload.latitude,
            location.payload.longitude,
            location.payload.description,
            location.id.0,
            location.user_id.0,
        )
        .fetch_one(&self.db)
        .await
        .map_err(|_| {
            tracing::error!("Error updating location: {:?}", location);
            (
                StatusCode::NOT_FOUND,
                Json(NaturalPhenomenonLocationError::NotFound),
            )
        })?;

        Ok(UpdateNaturalPhenomenonLocationResponseSuccess {
            id: record.id,
            user_id: record.user_id,
            name: record.name,
            latitude: record.latitude,
            longitude: record.longitude,
            description: record.description,
            image_path: record.image_path,
        })
    }

    async fn delete(
        &self,
        user_id: DatabaseId,
        id: DatabaseId,
    ) -> Result<
        (StatusCode, Json<NaturalPhenomenonLocationResponseSuccess>),
        (StatusCode, Json<NaturalPhenomenonLocationError>),
    > {
        // 1) Delete the DB row, grabbing the image_path
        let rec = sqlx::query!(
            r#"
            DELETE FROM natural_phenomenon_locations
             WHERE id = $1 AND user_id = $2
            RETURNING image_path
            "#,
            id.0,
            user_id.0,
        )
            .fetch_one(&self.db)
            .await
            .map_err(|e| {
                tracing::error!("DB delete error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(NaturalPhenomenonLocationError::DatabaseError),
                )
            })?;

        // 2) If there was an image_path, remove the file (ignore FS errors)
        if let path = rec.image_path {
            let _ = fs::remove_file(&path).await;
        }

        // 3) Success → return a 204 + our Deleted enum
        Ok((
            StatusCode::NO_CONTENT,
            Json(NaturalPhenomenonLocationResponseSuccess::Deleted),
        ))
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::routes::natural_phenomenon_locations::models::{
//         CreateAndUpdateResponseSuccess, CreateNaturalPhenomenonLocationWithImage,
//         GetAllNaturalPhenomenonLocationResponseSuccess,
//         GetByIdNaturalPhenomenonLocationResponseSuccess, UpdateNaturalPhenomenonLocationRequest,
//         UpdateNaturalPhenomenonLocationRequestWithIds,
//         UpdateNaturalPhenomenonLocationResponseSuccess,
//     };
//     use crate::routes::natural_phenomenon_locations::services::PgNaturalPhenomenonLocationService;
//     use crate::shared::models::DatabaseId;
//     use crate::tests::tests::TestApp;
//     use sqlx::PgPool;
//     use tokio::fs;
//
//     #[sqlx::test]
//     async fn test_natural_phenomenon_location_service_crud(pool: PgPool) {
//         // 0) setup
//         let test_app = TestApp::new(pool.clone()).await;
//         let user_id: DatabaseId = test_app.users[0].user.id;
//         let service = PgNaturalPhenomenonLocationService::new(pool.clone());
//
//         // prepare a dummy “image upload”
//         let image_bytes = b"this is a test image".to_vec();
//         let image_filename = "foo.png".to_owned();
//
//         // 1) CREATE
//         let create_req = CreateNaturalPhenomenonLocationWithImage {
//             user_id,
//             name: "Grand Canyon".into(),
//             latitude: 36.1069,
//             longitude: -112.1129,
//             description: "A famous canyon".into(),
//             image_bytes: image_bytes.clone(),
//             image_filename: image_filename.clone(),
//         };
//         let created: CreateAndUpdateResponseSuccess =
//             service.create(create_req).await.expect("create failed");
//
//         // basic field assertions
//         assert_eq!(created.user_id, user_id);
//         assert_eq!(created.name, "Grand Canyon");
//         assert_eq!(created.latitude, 36.1069);
//         assert_eq!(created.longitude, -112.1129);
//         assert_eq!(created.description, "A famous canyon");
//
//         // the image was written to disk at `created.image_path`
//         assert!(
//             fs::metadata(&created.image_path).await.is_ok(),
//             "image file not found on disk"
//         );
//
//         let id = created.id;
//
//         // 2) GET_ALL
//         let all: Vec<GetAllNaturalPhenomenonLocationResponseSuccess> =
//             service.get_all(user_id).await.expect("get_all failed");
//         assert_eq!(all.len(), 1);
//         let first = &all[0];
//         assert_eq!(first.id, id);
//         assert_eq!(first.image_path, created.image_path);
//
//         // 3) GET_BY_ID
//         let fetched: GetByIdNaturalPhenomenonLocationResponseSuccess = service
//             .get_by_id(user_id, id)
//             .await
//             .expect("get_by_id failed");
//         assert_eq!(fetched.id, id);
//         assert_eq!(fetched.user_id, user_id);
//         assert_eq!(fetched.image_path, created.image_path);
//
//         // 4) UPDATE
//         let update_req = UpdateNaturalPhenomenonLocationRequestWithIds {
//             user_id,
//             id,
//             payload: UpdateNaturalPhenomenonLocationRequest {
//                 name: Some("Grand Canyon Updated".into()),
//                 latitude: Some(36.11),
//                 longitude: Some(-112.11),
//                 description: Some("Now even more famous".into()),
//             },
//         };
//         let updated: UpdateNaturalPhenomenonLocationResponseSuccess = service
//             .update(update_req.clone())
//             .await
//             .expect("update failed");
//         assert_eq!(updated.id, id);
//         assert_eq!(updated.name, "Grand Canyon Updated");
//         assert!((updated.latitude - 36.11).abs() < f64::EPSILON);
//         assert!((updated.longitude + 112.11).abs() < f64::EPSILON);
//         assert_eq!(updated.description, "Now even more famous");
//
//         // the image_path must be unchanged
//         assert_eq!(updated.image_path, created.image_path);
//
//         // 5) DELETE
//         service.delete(user_id, id).await.expect("delete failed");
//         let remaining = service
//             .get_all(user_id)
//             .await
//             .expect("get_all after delete");
//         assert!(remaining.is_empty(), "expected no remaining entries");
//     }
// }
