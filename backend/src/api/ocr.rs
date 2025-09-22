use axum::{
    extract::{Multipart, State},
    response::sse::{Event, KeepAlive, Sse},
    response::IntoResponse,
};
use futures_util::stream::Stream;
use std::{convert::Infallible, pin::Pin, sync::Arc, time::Instant};
use tokio::sync::broadcast;
use uuid::Uuid;
use chrono::Utc;

use crate::{
    config::UploadConfig,
    errors::UploadError,
    models::{ProcessingEvent, ValidationErrorCode, ProcessingErrorType, ImageFileInfo, ValidationStatus},
    services::image_validation::{validate_image_format, validate_file_size},
    state::AppState,
};

pub async fn upload_images_sse(
    State(app_state): State<AppState>,
    multipart: Multipart,
) -> Result<impl IntoResponse, UploadError> {
    let session_id = Uuid::new_v4().to_string();
    let broadcaster = app_state.event_broadcaster.clone();

    // Start background processing
    tokio::spawn(async move {
        if let Err(e) = process_upload_with_events(multipart, broadcaster.clone(), session_id.clone(), app_state.upload_config).await {
            let _ = broadcaster.send(ProcessingEvent::ProcessingError {
                session_id,
                error_message: e.to_string(),
                error_type: ProcessingErrorType::InternalServerError,
                timestamp: Utc::now(),
            });
        }
    });

    // Return SSE stream
    let mut receiver = app_state.event_broadcaster.subscribe();
    let stream: Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send>> = Box::pin(async_stream::stream! {
        while let Ok(event) = receiver.recv().await {
            let event_type = match &event {
                ProcessingEvent::UploadStarted { .. } => "upload_started",
                ProcessingEvent::ImageReceived { .. } => "image_received",
                ProcessingEvent::ImageValidationStart { .. } => "image_validation_start",
                ProcessingEvent::ImageValidationSuccess { .. } => "image_validation_success",
                ProcessingEvent::ImageValidationError { .. } => "image_validation_error",
                ProcessingEvent::AllImagesValidated { .. } => "all_images_validated",
                ProcessingEvent::ProcessingComplete { .. } => "processing_complete",
                ProcessingEvent::ProcessingError { .. } => "processing_error",
            };

            let data = serde_json::to_string(&event).unwrap_or_default();
            yield Ok(Event::default().event(event_type).data(data));

            // Close stream on completion
            if matches!(event, ProcessingEvent::ProcessingComplete { .. } | ProcessingEvent::ProcessingError { .. }) {
                break;
            }
        }
    });

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}

async fn process_upload_with_events(
    mut multipart: Multipart,
    broadcaster: broadcast::Sender<ProcessingEvent>,
    session_id: String,
    config: Arc<UploadConfig>,
) -> Result<(), UploadError> {
    let start_time = Instant::now();
    let mut files = Vec::new();

    // Collect all files first
    while let Some(field) = multipart.next_field().await.map_err(|e| UploadError::MultipartError(e.to_string()))? {
        if field.name() == Some("images") {
            let file_name = field.file_name().map(|s| s.to_string());
            let data = field.bytes().await.map_err(|e| UploadError::MultipartError(e.to_string()))?;

            files.push((file_name, data));
        }
    }

    if files.is_empty() {
        return Err(UploadError::MultipartError("No images provided".to_string()));
    }

    // Send upload started event
    let _ = broadcaster.send(ProcessingEvent::UploadStarted {
        total_files: files.len(),
        session_id: session_id.clone(),
        timestamp: Utc::now(),
    });

    let mut successful_files = 0;

    for (file_index, (file_name, data)) in files.iter().enumerate() {
        // Send image received event
        let _ = broadcaster.send(ProcessingEvent::ImageReceived {
            file_index,
            file_name: file_name.clone(),
            size_bytes: data.len(),
            timestamp: Utc::now(),
        });

        // Send validation start event
        let _ = broadcaster.send(ProcessingEvent::ImageValidationStart {
            file_index,
            file_name: file_name.clone(),
            timestamp: Utc::now(),
        });

        // Validate file
        match validate_file(data, config.as_ref(), file_index).await {
            Ok(file_info) => {
                let _ = broadcaster.send(ProcessingEvent::ImageValidationSuccess {
                    file_index,
                    file_info,
                    timestamp: Utc::now(),
                });
                successful_files += 1;
            }
            Err(error) => {
                let _ = broadcaster.send(ProcessingEvent::ImageValidationError {
                    file_index,
                    file_name: file_name.clone(),
                    error_message: error.to_string(),
                    error_code: map_error_to_code(&error),
                    timestamp: Utc::now(),
                });
            }
        }
    }

    // Send all images validated event
    let _ = broadcaster.send(ProcessingEvent::AllImagesValidated {
        total_processed: files.len(),
        successful_count: successful_files,
        failed_count: files.len() - successful_files,
        timestamp: Utc::now(),
    });

    // Send completion event
    let _ = broadcaster.send(ProcessingEvent::ProcessingComplete {
        session_id,
        total_files: files.len(),
        successful_files,
        duration_ms: start_time.elapsed().as_millis() as u64,
        timestamp: Utc::now(),
    });

    Ok(())
}

async fn validate_file(data: &[u8], config: &UploadConfig, file_index: usize) -> Result<ImageFileInfo, UploadError> {
    let validation_start = Instant::now();

    validate_file_size(data.len(), config.max_file_size_bytes)?;
    let content_type = validate_image_format(data).await?;

    let processing_duration = validation_start.elapsed().as_millis() as u64;

    Ok(ImageFileInfo {
        file_name: None, // Will be set by caller
        content_type: content_type.clone(),
        size_bytes: data.len(),
        format: content_type.split('/').nth(1).unwrap_or("unknown").to_uppercase(),
        validation_status: ValidationStatus::Valid,
        file_index,
        processed_at: Utc::now(),
        processing_duration_ms: processing_duration,
    })
}

fn map_error_to_code(error: &UploadError) -> ValidationErrorCode {
    match error {
        UploadError::FileSizeExceeded { size, limit } => ValidationErrorCode::FileSizeExceeded {
            actual: *size,
            limit: *limit
        },
        UploadError::InvalidImageFormat(format) => ValidationErrorCode::UnsupportedFormat {
            detected: format.clone()
        },
        UploadError::ImageCountExceeded { count, limit } => ValidationErrorCode::CountLimitExceeded {
            count: *count,
            limit: *limit
        },
        UploadError::MultipartError(_) => ValidationErrorCode::CorruptedFile,
    }
}

// Keep the old handler for backward compatibility during transition
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
                file_index: image_count - 1,
                processed_at: Utc::now(),
                processing_duration_ms: 0,
            };

            accepted_images.push(image_info);
        }
    }

    if image_count == 0 {
        return Err(UploadError::MultipartError("No images provided".to_string()));
    }

    let processing_time = start_time.elapsed().as_millis() as u64;
    let result = crate::models::ValidationResult::success(accepted_images, processing_time);

    Ok(axum::Json(result))
}