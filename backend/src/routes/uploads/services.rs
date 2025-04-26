use crate::routes::uploads::models::Photo;
use async_trait::async_trait;
use std::fs;
use std::io;
use std::path::PathBuf;

/// Service for reading the `uploads/` folder.
#[derive(Clone)]
pub struct UploadsService {
    /// On-disk directory (must exist)
    pub directory: PathBuf,
    /// URL prefix under which static files will be served (no trailing slash)
    pub url_prefix: String,
}

#[async_trait]
pub trait UploadsServiceImpl: Send + Sync + 'static {
    /// Create a new service pointing at `uploads/` and URL prefix `/uploads`
    async fn new() -> Self;

    /// Read `uploads/` and produce a list of `Photo` entries.
    async fn list_photos(&self) -> io::Result<Vec<Photo>>;
}

#[async_trait]
impl UploadsServiceImpl for UploadsService {
    /// Create a new service pointing at `uploads/` and URL prefix `/uploads`
    async fn new() -> Self {
        UploadsService {
            directory: PathBuf::from("uploads"),
            url_prefix: "/uploads".to_string(),
        }
    }

    /// Read `uploads/` and produce a list of `Photo` entries.
    async fn list_photos(&self) -> io::Result<Vec<Photo>> {
        let mut photos = Vec::new();
        // Ensure directory exists
        if !self.directory.exists() {
            fs::create_dir_all(&self.directory)?;
        }
        for entry in fs::read_dir(&self.directory)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            if file_type.is_file() {
                let filename = entry.file_name().to_string_lossy().to_string();
                let url = format!("{}/{}", self.url_prefix, filename);
                photos.push(Photo { filename, url });
            }
        }
        Ok(photos)
    }
}
