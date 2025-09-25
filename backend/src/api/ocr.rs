use axum::{
    extract::{Multipart, State},
    response::IntoResponse,
    response::sse::{Event, KeepAlive, Sse},
};
use chrono::Utc;
use futures_util::stream::Stream;
use std::{convert::Infallible, pin::Pin, sync::Arc, time::Instant};
use tokio::sync::broadcast;
use tracing::{debug, error, info, instrument, warn};
use uuid::Uuid;

use crate::{
    config::UploadConfig,
    errors::UploadError,
    models::{
        ImageFileInfo, ProcessingErrorType, ProcessingEvent, ValidationErrorCode, ValidationStatus,
    },
    services::{
        bill_extractor::BillDataExtractor,
        bill_service::BillService,
        gemini_service::{GeminiError, GeminiService},
        image_validation::{validate_file_size, validate_image_format},
    },
    state::AppState,
    utils::image_utils::resize_image_default,
};

pub async fn upload_images_sse(
    State(app_state): State<AppState>,
    multipart: Multipart,
) -> Result<impl IntoResponse, UploadError> {
    let session_id = Uuid::new_v4().to_string();
    let broadcaster = app_state.event_broadcaster.clone();
    let app_state_clone = app_state.clone();

    // Start background processing
    tokio::spawn(async move {
        if let Err(e) = process_upload_with_events(
            multipart,
            broadcaster.clone(),
            session_id.clone(),
            app_state_clone,
        )
        .await
        {
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
    let stream: Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send>> = Box::pin(
        async_stream::stream! {
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
                    ProcessingEvent::GeminiProcessingStart { .. } => "gemini_processing_start",
                    ProcessingEvent::GeminiProcessingSuccess { .. } => "gemini_processing_success",
                    ProcessingEvent::GeminiProcessingError { .. } => "gemini_processing_error",
                    ProcessingEvent::BillDataSaved { .. } => "bill_data_saved",
                };

                let data = serde_json::to_string(&event).unwrap_or_default();
                yield Ok(Event::default().event(event_type).data(data));

                // Close stream on completion
                if matches!(event, ProcessingEvent::ProcessingComplete { .. } | ProcessingEvent::ProcessingError { .. }) {
                    break;
                }
            }
        },
    );

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}

async fn process_upload_with_events(
    mut multipart: Multipart,
    broadcaster: broadcast::Sender<ProcessingEvent>,
    session_id: String,
    app_state: AppState,
) -> Result<(), UploadError> {
    let config = app_state.upload_config.clone();
    let start_time = Instant::now();
    let mut files = Vec::new();

    // Collect all files first
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| UploadError::MultipartError(e.to_string()))?
    {
        if field.name() == Some("images") {
            let file_name = field.file_name().map(|s| s.to_string());
            let data = field
                .bytes()
                .await
                .map_err(|e| UploadError::MultipartError(e.to_string()))?;

            files.push((file_name, data));
        }
    }

    if files.is_empty() {
        return Err(UploadError::MultipartError(
            "No images provided".to_string(),
        ));
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

                // Resize image pixel dimensions before processing with Gemini
                info!("Resizing image pixel dimensions to 40% for file index {}", file_index);
                let resized_data = match resize_image_default(data) {
                    Ok(resized) => {
                        let size_reduction = ((data.len() - resized.len()) as f32 / data.len() as f32) * 100.0;
                        info!(
                            "Image pixel dimensions resized: {} bytes -> {} bytes (file size reduced by {:.1}%)",
                            data.len(),
                            resized.len(),
                            size_reduction
                        );
                        resized
                    }
                    Err(e) => {
                        warn!("Failed to resize image pixel dimensions for file index {}: {}. Using original image.", file_index, e);
                        data.to_vec()
                    }
                };

                // Process with Gemini after successful validation and resizing
                match process_with_gemini(
                    &resized_data,
                    file_index,
                    file_name.clone(),
                    broadcaster.clone(),
                    &app_state.pool,
                )
                .await
                {
                    Ok(()) => {
                        successful_files += 1;
                    }
                    Err(e) => {
                        // Log Gemini error but don't fail the entire upload
                        let _ = broadcaster.send(ProcessingEvent::GeminiProcessingError {
                            file_index,
                            error_message: format!("Gemini processing failed: {}", e),
                            timestamp: Utc::now(),
                        });
                        successful_files += 1; // Still count as successful since image validation passed
                    }
                }
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

async fn validate_file(
    data: &[u8],
    config: &UploadConfig,
    file_index: usize,
) -> Result<ImageFileInfo, UploadError> {
    let validation_start = Instant::now();

    validate_file_size(data.len(), config.max_file_size_bytes)?;
    let content_type = validate_image_format(data).await?;

    let processing_duration = validation_start.elapsed().as_millis() as u64;

    Ok(ImageFileInfo {
        file_name: None, // Will be set by caller
        content_type: content_type.clone(),
        size_bytes: data.len(),
        format: content_type
            .split('/')
            .nth(1)
            .unwrap_or("unknown")
            .to_uppercase(),
        validation_status: ValidationStatus::Valid,
        file_index,
        processed_at: Utc::now(),
        processing_duration_ms: processing_duration,
    })
}

/// Process image with Gemini AI and save extracted bill data
#[instrument(
    skip(image_data, broadcaster, connection_pool),
    fields(file_index, file_name)
)]
async fn process_with_gemini(
    image_data: &[u8],
    file_index: usize,
    file_name: Option<String>,
    broadcaster: broadcast::Sender<ProcessingEvent>,
    connection_pool: &crate::config::ConnectionPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!(
        "Starting Gemini processing for file {} (index: {})",
        file_name.as_deref().unwrap_or("unknown"),
        file_index
    );

    // Send Gemini processing start event
    let _ = broadcaster.send(ProcessingEvent::GeminiProcessingStart {
        file_index,
        file_name: file_name.clone(),
        timestamp: Utc::now(),
    });

    // Initialize Gemini service
    debug!("Initializing Gemini service");
    let gemini_service = GeminiService::with_default_config().map_err(|e| {
        error!("Failed to initialize Gemini service: {}", e);
        format!("Failed to initialize Gemini service: {}", e)
    })?;

    // Extract bill data from image
    let gemini_responses = match gemini_service.extract_bill_data(image_data).await {
        Ok(response) => response,
        Err(GeminiError::RateLimitExceeded { retry_after }) => {
            let error_msg = format!(
                "Gemini API rate limit exceeded. Retry after: {:?} seconds",
                retry_after
            );
            let _ = broadcaster.send(ProcessingEvent::GeminiProcessingError {
                file_index,
                error_message: error_msg.clone(),
                timestamp: Utc::now(),
            });
            return Err(UploadError::MultipartError(error_msg).into());
        }
        Err(GeminiError::AuthenticationFailed) => {
            let error_msg =
                "Gemini API authentication failed. Please check your API key.".to_string();
            let _ = broadcaster.send(ProcessingEvent::GeminiProcessingError {
                file_index,
                error_message: error_msg.clone(),
                timestamp: Utc::now(),
            });
            return Err(UploadError::MultipartError(error_msg).into());
        }
        Err(GeminiError::Timeout { seconds }) => {
            let error_msg = format!("Gemini API request timeout after {} seconds", seconds);
            let _ = broadcaster.send(ProcessingEvent::GeminiProcessingError {
                file_index,
                error_message: error_msg.clone(),
                timestamp: Utc::now(),
            });
            return Err(UploadError::MultipartError(error_msg).into());
        }
        Err(GeminiError::ApiError { status, message }) => {
            let error_msg = format!("Gemini API error {}: {}", status, message);
            let _ = broadcaster.send(ProcessingEvent::GeminiProcessingError {
                file_index,
                error_message: error_msg.clone(),
                timestamp: Utc::now(),
            });
            return Err(UploadError::MultipartError(error_msg).into());
        }
        Err(e) => {
            let error_msg = format!("Gemini processing failed: {}", e);
            let _ = broadcaster.send(ProcessingEvent::GeminiProcessingError {
                file_index,
                error_message: error_msg.clone(),
                timestamp: Utc::now(),
            });
            return Err(UploadError::MultipartError(error_msg).into());
        }
    };

    // Send Gemini processing success event
    let _ = broadcaster.send(ProcessingEvent::GeminiProcessingSuccess {
        file_index,
        extracted_data: gemini_responses.clone(),
        timestamp: Utc::now(),
    });

    debug!(
        "Extracting and validating bill data from Gemini response ({} candidate(s))",
        gemini_responses.len()
    );
    let extractor = BillDataExtractor::new();
    let bill_service = BillService::new(connection_pool.pool().clone());

    for (candidate_idx, response) in gemini_responses.iter().enumerate() {
        debug!(
            "Validating Gemini candidate {} for file index {}",
            candidate_idx, file_index
        );

        let bill_data = extractor.extract_and_validate(response).map_err(|e| {
            error!(
                "Data extraction error for candidate {}: {}",
                candidate_idx, e
            );
            UploadError::MultipartError(format!("Data extraction error: {}", e))
        })?;

        debug!(
            "Successfully extracted bill data from candidate {}: form_no={:?}, invoice_no={:?}",
            candidate_idx, bill_data.form_no, bill_data.invoice_no
        );

        match bill_service.create_bill(bill_data).await {
            Ok(bill) => {
                info!(
                    "Successfully saved bill data (candidate {}) to database with ID: {}",
                    candidate_idx, bill.id
                );
                let _ = broadcaster.send(ProcessingEvent::BillDataSaved {
                    file_index,
                    bill_id: bill.id,
                    timestamp: Utc::now(),
                });
            }
            Err(e) => {
                error!(
                    "Failed to save bill data (candidate {}) to database: {:?}",
                    candidate_idx, e
                );
            }
        }
    }

    Ok(())
}

fn map_error_to_code(error: &UploadError) -> ValidationErrorCode {
    match error {
        UploadError::FileSizeExceeded { size, limit } => ValidationErrorCode::FileSizeExceeded {
            actual: *size,
            limit: *limit,
        },
        UploadError::InvalidImageFormat(format) => ValidationErrorCode::UnsupportedFormat {
            detected: format.clone(),
        },
        UploadError::ImageCountExceeded { count, limit } => {
            ValidationErrorCode::CountLimitExceeded {
                count: *count,
                limit: *limit,
            }
        }
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
                format: content_type
                    .split('/')
                    .nth(1)
                    .unwrap_or("unknown")
                    .to_uppercase(),
                validation_status: ValidationStatus::Valid,
                file_index: image_count - 1,
                processed_at: Utc::now(),
                processing_duration_ms: 0,
            };

            accepted_images.push(image_info);
        }
    }

    if image_count == 0 {
        return Err(UploadError::MultipartError(
            "No images provided".to_string(),
        ));
    }

    let processing_time = start_time.elapsed().as_millis() as u64;
    let result = crate::models::ValidationResult::success(accepted_images, processing_time);

    Ok(axum::Json(result))
}
