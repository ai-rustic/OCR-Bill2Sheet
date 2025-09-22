use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageFileInfo {
    pub file_name: Option<String>,
    pub content_type: String,
    pub size_bytes: usize,
    pub format: String,
    pub validation_status: ValidationStatus,
    // New SSE-specific fields
    pub file_index: usize,
    pub processed_at: DateTime<Utc>,
    pub processing_duration_ms: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum ValidationStatus {
    #[default]
    Pending,
    Validating,
    Valid,
    Invalid(String),
}