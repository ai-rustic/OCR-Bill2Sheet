use axum::{
    extract::{Multipart, State},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use std::time::Instant;

use crate::{
    config::UploadConfig,
    errors::UploadError,
    models::{ImageFileInfo, ValidationResult, ValidationStatus},
    services::image_validation::{validate_image_format, validate_file_size},
};

pub async fn upload_images(
    State(config): State<Arc<UploadConfig>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, UploadError> {
    let start_time = Instant::now();
    let mut image_count = 0;
    let mut accepted_images = Vec::new();
    let mut _total_size = 0;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| UploadError::MultipartError(e.to_string()))?
    {
        if field.name() == Some("images") {
            image_count += 1;

            if image_count > config.max_image_count {
                return Err(UploadError::ImageCountExceeded {
                    count: image_count,
                    limit: config.max_image_count,
                });
            }

            let file_name = field.file_name().map(|s| s.to_string());
            let data = field
                .bytes()
                .await
                .map_err(|e| UploadError::MultipartError(e.to_string()))?;

            validate_file_size(data.len(), config.max_file_size_bytes)?;
            let content_type = validate_image_format(&data).await?;

            _total_size += data.len();

            let image_info = ImageFileInfo {
                file_name,
                content_type: content_type.clone(),
                size_bytes: data.len(),
                format: content_type.split('/').nth(1).unwrap_or("unknown").to_uppercase(),
                validation_status: ValidationStatus::Valid,
            };

            accepted_images.push(image_info);
        }
    }

    if image_count == 0 {
        return Err(UploadError::MultipartError("No images provided".to_string()));
    }

    let processing_time = start_time.elapsed().as_millis() as u64;
    let result = ValidationResult::success(accepted_images, processing_time);

    Ok(Json(result))
}