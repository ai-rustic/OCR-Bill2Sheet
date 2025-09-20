use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageFileInfo {
    pub file_name: Option<String>,
    pub content_type: String,
    pub size_bytes: usize,
    pub format: String,
    pub validation_status: ValidationStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum ValidationStatus {
    #[default]
    Pending,
    Validating,
    Valid,
    Invalid(String),
}