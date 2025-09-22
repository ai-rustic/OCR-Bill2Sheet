# Quickstart Guide: SSE OCR Endpoint Implementation

## Prerequisites
- Rust 1.75+ with edition = "2024"
- Existing OCR_Bill2Sheet backend running
- Access to modify `backend/src/api/ocr.rs`
- PostgreSQL database with existing configuration

## Implementation Steps

### 1. Add Required Dependencies
Update `backend/Cargo.toml`:
```toml
[dependencies]
# Existing dependencies...
axum = { version = "0.8.4", features = ["macros", "multipart"] }
tokio = { version = "1.47.1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }

# New dependencies for SSE
futures-util = "0.3"
tokio-stream = { version = "0.1", features = ["sync"] }
async-stream = "0.3"
uuid = { version = "1.0", features = ["v4"] }
```

### 2. Create Event Models
Create `backend/src/models/sse_events.rs`:
```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::models::ImageFileInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ProcessingEvent {
    UploadStarted {
        total_files: usize,
        session_id: String,
        timestamp: DateTime<Utc>,
    },
    ImageReceived {
        file_index: usize,
        file_name: Option<String>,
        size_bytes: usize,
        timestamp: DateTime<Utc>,
    },
    ImageValidationStart {
        file_index: usize,
        file_name: Option<String>,
        timestamp: DateTime<Utc>,
    },
    ImageValidationSuccess {
        file_index: usize,
        file_info: ImageFileInfo,
        timestamp: DateTime<Utc>,
    },
    ImageValidationError {
        file_index: usize,
        file_name: Option<String>,
        error_message: String,
        error_code: ValidationErrorCode,
        timestamp: DateTime<Utc>,
    },
    AllImagesValidated {
        total_processed: usize,
        successful_count: usize,
        failed_count: usize,
        timestamp: DateTime<Utc>,
    },
    ProcessingComplete {
        session_id: String,
        total_files: usize,
        successful_files: usize,
        duration_ms: u64,
        timestamp: DateTime<Utc>,
    },
    ProcessingError {
        session_id: String,
        error_message: String,
        error_type: ProcessingErrorType,
        timestamp: DateTime<Utc>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationErrorCode {
    FileSizeExceeded { actual: usize, limit: usize },
    UnsupportedFormat { detected: String },
    CorruptedFile,
    EmptyFile,
    CountLimitExceeded { count: usize, limit: usize },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessingErrorType {
    MultipartParsingError,
    SystemTimeout,
    InternalServerError,
    ClientDisconnected,
}
```

### 3. Update Application State
Modify `backend/src/main.rs` to add SSE broadcaster:
```rust
use tokio::sync::broadcast;
use std::sync::Arc;

// Add to your app state
#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<sqlx::PgPool>,
    pub upload_config: Arc<UploadConfig>,
    pub event_broadcaster: broadcast::Sender<ProcessingEvent>, // Add this
}

#[tokio::main]
async fn main() {
    // ... existing setup ...

    // Create event broadcaster
    let (event_broadcaster, _) = broadcast::channel(1000);

    let app_state = AppState {
        pool: Arc::new(pool),
        upload_config: Arc::new(upload_config),
        event_broadcaster, // Add this
    };

    let app = Router::new()
        .route("/api/ocr", post(api::ocr::upload_images_sse)) // Update handler name
        // ... other routes ...
        .with_state(app_state);

    // ... rest of main function ...
}
```

### 4. Modify OCR Handler
Replace `backend/src/api/ocr.rs` content:
```rust
use axum::{
    extract::{Multipart, State},
    response::sse::{Event, KeepAlive, Sse},
    response::IntoResponse,
};
use futures_util::stream::Stream;
use std::{convert::Infallible, sync::Arc, time::Instant};
use tokio::sync::broadcast;
use uuid::Uuid;
use chrono::Utc;

use crate::{
    config::UploadConfig,
    errors::UploadError,
    models::{sse_events::*, ImageFileInfo, ValidationStatus},
    services::image_validation::{validate_image_format, validate_file_size},
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
    let stream = async_stream::stream! {
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
    };

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
        match validate_file(data, config.as_ref()).await {
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

async fn validate_file(data: &[u8], config: &UploadConfig) -> Result<ImageFileInfo, UploadError> {
    validate_file_size(data.len(), config.max_file_size_bytes)?;
    let content_type = validate_image_format(data).await?;

    Ok(ImageFileInfo {
        file_name: None, // Will be set by caller
        content_type: content_type.clone(),
        size_bytes: data.len(),
        format: content_type.split('/').nth(1).unwrap_or("unknown").to_uppercase(),
        validation_status: ValidationStatus::Valid,
        file_index: 0, // Will be set by caller
        processed_at: Utc::now(),
        processing_duration_ms: 0, // Calculate if needed
    })
}

fn map_error_to_code(error: &UploadError) -> ValidationErrorCode {
    match error {
        UploadError::FileSizeExceeded { size, limit } => ValidationErrorCode::FileSizeExceeded {
            actual: *size,
            limit: *limit
        },
        UploadError::UnsupportedFormat { .. } => ValidationErrorCode::UnsupportedFormat {
            detected: "unknown".to_string()
        },
        _ => ValidationErrorCode::CorruptedFile,
    }
}
```

### 5. Update Models Module
Add to `backend/src/models/mod.rs`:
```rust
pub mod sse_events;
```

And update `ImageFileInfo` in existing models to include new fields:
```rust
pub struct ImageFileInfo {
    // ... existing fields ...
    pub file_index: usize,
    pub processed_at: chrono::DateTime<chrono::Utc>,
    pub processing_duration_ms: u64,
}
```

### 6. Test the Implementation

#### Build and Run
```bash
cd backend
cargo build
cargo run
```

#### Test with cURL
```bash
curl -X POST http://localhost:3000/api/ocr \
  -H "Accept: text/event-stream" \
  -H "Cache-Control: no-cache" \
  -F "images=@test1.jpg" \
  -F "images=@test2.png" \
  --no-buffer
```

Expected output:
```
event: upload_started
data: {"type":"upload_started","data":{"total_files":2,"session_id":"...","timestamp":"..."}}

event: image_received
data: {"type":"image_received","data":{"file_index":0,"file_name":"test1.jpg","size_bytes":1024,"timestamp":"..."}}

event: image_validation_start
data: {"type":"image_validation_start","data":{"file_index":0,"file_name":"test1.jpg","timestamp":"..."}}

event: image_validation_success
data: {"type":"image_validation_success","data":{"file_index":0,"file_info":{...},"timestamp":"..."}}

...

event: processing_complete
data: {"type":"processing_complete","data":{"session_id":"...","total_files":2,"successful_files":2,"duration_ms":150,"timestamp":"..."}}
```

## Verification Checklist

### Functional Tests
- [ ] Upload single image file → receives all expected events
- [ ] Upload multiple images → receives events for each file
- [ ] Upload invalid file → receives validation error event
- [ ] Upload oversized file → receives error event
- [ ] Client disconnection → processing stops gracefully

### Performance Tests
- [ ] Concurrent uploads don't interfere with each other
- [ ] Memory usage remains bounded during processing
- [ ] Event ordering is maintained under load

### Integration Tests
- [ ] Existing file size limits are respected
- [ ] Environment configuration works correctly
- [ ] Error handling preserves existing behavior

## Troubleshooting

### Common Issues
1. **Events not received**: Check `Accept: text/event-stream` header
2. **Connection timeout**: Verify keep-alive settings
3. **Memory growth**: Check broadcast channel buffer size
4. **Build errors**: Ensure all dependencies are added to Cargo.toml

### Debug Commands
```bash
# Check server logs
RUST_LOG=debug cargo run

# Test event stream
curl -v -X POST http://localhost:3000/api/ocr \
  -H "Accept: text/event-stream" \
  -F "images=@test.jpg"

# Monitor memory usage
cargo run --features="tokio-console"
```

## Next Steps
1. Update frontend to consume SSE events instead of JSON
2. Add proper error handling for edge cases
3. Implement connection limits and rate limiting
4. Add monitoring and metrics for SSE performance