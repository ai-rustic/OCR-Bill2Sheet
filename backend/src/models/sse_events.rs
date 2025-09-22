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

#[derive(Debug, Clone, Serialize)]
pub struct SSEEventEnvelope {
    pub event_type: String,
    pub event_id: Option<String>,
    pub data: ProcessingEvent,
    pub retry: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct ProcessingSession {
    pub session_id: String,
    pub start_time: DateTime<Utc>,
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