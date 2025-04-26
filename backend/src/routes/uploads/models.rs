use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// A single uploaded photo entry.
#[derive(Debug, Serialize)]
pub struct Photo {
    /// The bare filename (e.g. "uuid_pic.png")
    pub filename: String,
    /// The public URL under which it's served (e.g. "/uploads/uuid_pic.png")
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "data")]
pub enum UploadError {
    /// The file already exists
    AlreadyExists,
    /// The file is too large
    TooLarge,
    /// The file is not an image
    NotAnImage,
    /// The file could not be saved
    SaveFailed,
    NotFound,
}

impl Display for UploadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UploadError::AlreadyExists => write!(f, "File already exists"),
            UploadError::TooLarge => write!(f, "File is too large"),
            UploadError::NotAnImage => write!(f, "File is not an image"),
            UploadError::SaveFailed => write!(f, "Failed to save file"),
            UploadError::NotFound => write!(f, "File not found"),
        }
    }
}