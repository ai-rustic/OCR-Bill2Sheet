use axum::{http::StatusCode, response::IntoResponse};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum UploadError {
    #[error("File size {size} exceeds limit {limit}")]
    FileSizeExceeded { size: usize, limit: usize },

    #[error("Image count {count} exceeds limit {limit}")]
    ImageCountExceeded { count: usize, limit: usize },

    #[error("Invalid image format: {0}")]
    InvalidImageFormat(String),

    #[error("Multipart parsing failed: {0}")]
    MultipartError(String),
}

impl IntoResponse for UploadError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            UploadError::FileSizeExceeded { .. } => {
                (StatusCode::PAYLOAD_TOO_LARGE, self.to_string())
            }
            UploadError::ImageCountExceeded { .. } => {
                (StatusCode::UNPROCESSABLE_ENTITY, self.to_string())
            }
            UploadError::InvalidImageFormat(_) => {
                (StatusCode::UNSUPPORTED_MEDIA_TYPE, self.to_string())
            }
            UploadError::MultipartError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        };

        axum::Json(json!({
            "success": false,
            "error": message,
            "status": status.as_u16()
        }))
        .into_response()
    }
}
