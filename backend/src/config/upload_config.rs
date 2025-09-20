use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct UploadConfig {
    pub max_file_size_bytes: usize,
    pub max_image_count: usize,
}

impl UploadConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();

        let max_file_size = env::var("MAX_FILE_SIZE_BYTES")
            .unwrap_or_else(|_| "2097152".to_string())
            .parse()?;

        let max_image_count = env::var("MAX_IMAGE_COUNT")
            .unwrap_or_else(|_| "10".to_string())
            .parse()?;

        Ok(UploadConfig {
            max_file_size_bytes: max_file_size,
            max_image_count,
        })
    }
}