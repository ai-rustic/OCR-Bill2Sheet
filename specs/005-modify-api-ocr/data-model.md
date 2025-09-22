# Data Model: SSE Event Streaming for OCR Processing

## Overview
Data structures and event models for Server-Sent Events streaming during image upload and validation processing.

## Event Models

### ProcessingEvent (Primary Entity)
Main event type broadcasted through SSE stream during image processing.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ProcessingEvent {
    UploadStarted {
        total_files: usize,
        session_id: String,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    ImageReceived {
        file_index: usize,
        file_name: Option<String>,
        size_bytes: usize,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    ImageValidationStart {
        file_index: usize,
        file_name: Option<String>,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    ImageValidationSuccess {
        file_index: usize,
        file_info: ImageFileInfo,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    ImageValidationError {
        file_index: usize,
        file_name: Option<String>,
        error_message: String,
        error_code: ValidationErrorCode,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    AllImagesValidated {
        total_processed: usize,
        successful_count: usize,
        failed_count: usize,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    ProcessingComplete {
        session_id: String,
        total_files: usize,
        successful_files: usize,
        duration_ms: u64,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    ProcessingError {
        session_id: String,
        error_message: String,
        error_type: ProcessingErrorType,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
}
```

### ValidationErrorCode (Supporting Entity)
Enumeration of specific validation failure reasons.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationErrorCode {
    FileSizeExceeded { actual: usize, limit: usize },
    UnsupportedFormat { detected: String },
    CorruptedFile,
    EmptyFile,
    CountLimitExceeded { count: usize, limit: usize },
}
```

### ProcessingErrorType (Supporting Entity)
System-level error categorization.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessingErrorType {
    MultipartParsingError,
    SystemTimeout,
    InternalServerError,
    ClientDisconnected,
}
```

### Enhanced ImageFileInfo (Extended Entity)
Extends existing ImageFileInfo with SSE-specific metadata.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageFileInfo {
    // Existing fields
    pub file_name: Option<String>,
    pub content_type: String,
    pub size_bytes: usize,
    pub format: String,
    pub validation_status: ValidationStatus,

    // New SSE-specific fields
    pub file_index: usize,
    pub processed_at: chrono::DateTime<chrono::Utc>,
    pub processing_duration_ms: u64,
}
```

### SSEEventEnvelope (Transport Entity)
Wrapper structure for SSE event transmission.

```rust
#[derive(Debug, Clone, Serialize)]
pub struct SSEEventEnvelope {
    pub event_type: String,
    pub event_id: Option<String>,
    pub data: ProcessingEvent,
    pub retry: Option<u32>,
}
```

## State Models

### ProcessingSession (Session Management)
Tracks individual upload sessions for concurrent request handling.

```rust
#[derive(Debug, Clone)]
pub struct ProcessingSession {
    pub session_id: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub total_files: usize,
    pub processed_files: usize,
    pub status: SessionStatus,
    pub client_connected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionStatus {
    Processing,
    Completed,
    Failed,
    Cancelled,
}
```

### StreamingState (Application State Extension)
Extension to existing app state for SSE support.

```rust
#[derive(Clone)]
pub struct StreamingState {
    pub event_broadcaster: broadcast::Sender<ProcessingEvent>,
    pub active_sessions: Arc<RwLock<HashMap<String, ProcessingSession>>>,
    pub max_concurrent_streams: usize,
}
```

## Data Flow Relationships

### Event Sequence Flow
```
1. ProcessingEvent::UploadStarted
   ↓ (for each file)
2. ProcessingEvent::ImageReceived
   ↓
3. ProcessingEvent::ImageValidationStart
   ↓
4. ProcessingEvent::ImageValidationSuccess | ImageValidationError
   ↓ (after all files)
5. ProcessingEvent::AllImagesValidated
   ↓
6. ProcessingEvent::ProcessingComplete | ProcessingError
```

### Entity Relationships
- **ProcessingEvent** contains **ImageFileInfo** in success events
- **ProcessingEvent** contains **ValidationErrorCode** in error events
- **ProcessingSession** tracks multiple **ProcessingEvent** instances
- **SSEEventEnvelope** wraps **ProcessingEvent** for transmission

## Validation Rules

### Event Consistency Rules
1. **file_index** must be sequential starting from 0
2. **session_id** must be unique per upload session
3. **timestamp** must be monotonically increasing within session
4. **total_files** must remain consistent across session events

### Business Logic Constraints
1. Cannot send **ImageValidationStart** without preceding **ImageReceived**
2. Cannot send **AllImagesValidated** before all files processed
3. Must send exactly one completion event (**ProcessingComplete** or **ProcessingError**)
4. **ValidationErrorCode** must match actual validation failure

### Stream Integrity Rules
1. Events must be sent in chronological order
2. No duplicate events for same file_index
3. Session must end with completion or error event
4. Client disconnection cancels remaining events

## Storage Considerations

### In-Memory Only
- All event data is ephemeral (no persistence required)
- Events exist only during active processing session
- Session cleanup occurs on completion or disconnection

### Configuration Data
- File size limits from environment variables (existing)
- Stream buffer sizes from configuration
- Concurrent connection limits from settings

### No Database Changes
- No new tables or schema modifications required
- Existing bill table schema unchanged
- Configuration through environment variables only

## Error Handling Patterns

### Validation Error Recovery
```rust
// Individual file failures don't terminate stream
match validate_image(&file_data).await {
    Ok(file_info) => send_success_event(file_info),
    Err(error) => {
        send_error_event(error);
        continue; // Process next file
    }
}
```

### System Error Termination
```rust
// System-level errors terminate entire session
if system_error.is_fatal() {
    send_processing_error_event(system_error);
    return; // End processing
}
```

## Performance Characteristics

### Memory Usage
- **ProcessingEvent**: ~200-500 bytes per event
- **Session tracking**: ~100 bytes per active session
- **Broadcast buffer**: Configurable (default 1000 events)

### Event Frequency
- **Typical session**: 8-15 events per 3 images
- **Peak rate**: ~50 events/second per session
- **Concurrent sessions**: Limited by connection pool

### Cleanup Behavior
- **Session cleanup**: Automatic on completion/disconnection
- **Event cleanup**: Automatic via broadcast channel buffer
- **Memory bounds**: Enforced through bounded channels