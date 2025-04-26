use crate::shared::models::DatabaseId;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema, Clone, PartialEq)]
pub struct NaturalPhenomenonLocationDb {
    pub id: DatabaseId,
    pub user_id: DatabaseId,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub image_path: String,
    pub radius: i32,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, ToSchema, Clone, Serialize, PartialEq)]
pub struct UpdateNaturalPhenomenonLocationRequest {
    pub name: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub radius: Option<i32>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct UpdateNaturalPhenomenonLocationRequestWithIds {
    pub id: DatabaseId,
    pub user_id: DatabaseId,
    pub payload: UpdateNaturalPhenomenonLocationRequest,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub struct CreateAndUpdateResponseSuccess {
    pub id: DatabaseId, // None for new entities
    pub user_id: DatabaseId,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub radius: i32,
    pub description: String,
    pub image_path: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub struct GetAllNaturalPhenomenonLocationResponseSuccess {
    pub id: DatabaseId, // None for new entities
    pub user_id: DatabaseId,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub radius: i32,
    pub description: String,
    pub image_path: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub struct GetByIdNaturalPhenomenonLocationResponseSuccess {
    pub id: DatabaseId, // None for new entities
    pub user_id: DatabaseId,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub radius: i32,
    pub description: String,
    pub image_path: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub struct UpdateNaturalPhenomenonLocationResponseSuccess {
    pub id: DatabaseId, // None for new entities
    pub user_id: DatabaseId,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub radius: i32,
    pub description: String,
    pub image_path: String,
}



#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateNaturalPhenomenonLocationRequest {
    pub user_id: DatabaseId,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub radius: i32,
    pub image_path: String,
    pub description: String,
}

// Implement display for CreateNaturalPhenomenonLocationRequest
impl std::fmt::Display for CreateNaturalPhenomenonLocationRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CreateNaturalPhenomenonLocationRequest {{ user_id: {:?}, name: {}, latitude: {}, longitude: {}, description: {} }}",
               self.user_id, self.name, self.latitude, self.longitude, self.description)
    }
}

// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// pub struct NaturalPhenomenonResponseSuccess {
//     pub message: String,
// }

/// include the raw bytes + original filename
#[derive(Debug, ToSchema, Deserialize, Serialize)]
pub struct PostNaturalPhenomenonLocationService {
    pub user_id: DatabaseId,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub description: String,
    pub radius: i32,

    #[schema(format = Binary, content_media_type = "application/octet-stream")]
    #[serde(with = "serde_bytes")]
    pub image_bytes: Vec<u8>,

    #[schema(example = "photo.jpg")]
    pub image_filename: String,
}

/// include the raw bytes + original filename
#[derive(Debug, ToSchema, Deserialize, Serialize)]
pub struct PostNaturalPhenomenonLocationSchema {
    pub user_id: DatabaseId,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub description: String,
    pub radius: i32,

    #[schema(format = Binary, content_media_type = "application/octet-stream")]
    #[serde(with = "serde_bytes")]
    pub image: Vec<u8>,
}

#[derive(Debug, ToSchema, Deserialize, Serialize)]
pub struct CreateNaturalPhenomenonLocationInnerWithImage {
    pub user_id: DatabaseId,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub description: String,
    pub image_bytes: Vec<u8>,
    pub image_filename: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "data")]
pub enum NaturalPhenomenonLocationResponseSuccess {
    Deleted
}

impl Display for NaturalPhenomenonLocationResponseSuccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NaturalPhenomenonLocationResponseSuccess::Deleted => write!(f, "Deleted"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "data")]
pub enum NaturalPhenomenonLocationError {
    NotFound,
    AlreadyExists,
    DatabaseError,
    LocationCouldNotBeDeleted,
    ImageInLocationCloudNotBeDeleted,
}

impl Display for NaturalPhenomenonLocationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NaturalPhenomenonLocationError::NotFound => write!(f, "Not Found"),
            NaturalPhenomenonLocationError::AlreadyExists => write!(f, "Already Exists"),
            NaturalPhenomenonLocationError::DatabaseError => write!(f, "Database Error"),
            NaturalPhenomenonLocationError::LocationCouldNotBeDeleted => write!(f, "Location Could Not Be Deleted"),
            NaturalPhenomenonLocationError::ImageInLocationCloudNotBeDeleted => write!(f, "Image In Location Could Not Be Deleted"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub struct ServiceCreateAndUpdateResponseSuccess {
    pub id: DatabaseId, // None for new entities
    pub user_id: DatabaseId,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub radius: i32,
    pub description: String,
}
