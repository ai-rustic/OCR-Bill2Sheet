use crate::errors::UploadError;
use tracing::{debug, error, warn};

pub async fn validate_image_format(data: &[u8]) -> Result<String, UploadError> {
    debug!("Starting image format validation for {} bytes", data.len());

    // Magic byte validation
    let kind = infer::get(data).ok_or_else(|| {
        warn!("Failed to detect file format from magic bytes");
        UploadError::InvalidImageFormat("Unknown format".to_string())
    })?;

    debug!("Detected file type: {}", kind.mime_type());

    if !kind.mime_type().starts_with("image/") {
        warn!("File is not an image: detected type {}", kind.mime_type());
        return Err(UploadError::InvalidImageFormat("Not an image".to_string()));
    }

    // Image parsing validation
    match image::load_from_memory(data) {
        Ok(img) => {
            debug!(
                "Image successfully parsed: {}x{} pixels",
                img.width(),
                img.height()
            );
            Ok(kind.mime_type().to_string())
        }
        Err(e) => {
            error!("Image parsing failed: {}", e);
            Err(UploadError::InvalidImageFormat(
                "Corrupted image".to_string(),
            ))
        }
    }
}

pub fn validate_file_size(size: usize, limit: usize) -> Result<(), UploadError> {
    debug!(
        "Validating file size: {} bytes (limit: {} bytes)",
        size, limit
    );

    if size > limit {
        warn!("File size {} exceeds limit {}", size, limit);
        return Err(UploadError::FileSizeExceeded { size, limit });
    }

    debug!("File size validation passed");
    Ok(())
}
