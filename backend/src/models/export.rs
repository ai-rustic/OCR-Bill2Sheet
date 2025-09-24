// Export-related models and types
// This file contains ExportFormat, ExportParams, ExportResponse, and ExportError

use serde::{Deserialize, Serialize};
use std::fmt;

/// Supported export formats
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    /// Comma-separated values format with UTF-8 BOM
    Csv,
    /// Excel spreadsheet format with native UTF-8 encoding
    Xlsx,
}

impl fmt::Display for ExportFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExportFormat::Csv => write!(f, "csv"),
            ExportFormat::Xlsx => write!(f, "xlsx"),
        }
    }
}

/// Query parameters for export endpoint
#[derive(Debug, Deserialize, Serialize)]
pub struct ExportParams {
    /// Export format - either 'csv' or 'xlsx'
    pub format: ExportFormat,
}

impl ExportParams {
    pub fn new(format: ExportFormat) -> Self {
        Self { format }
    }

    /// Validate the export parameters
    pub fn validate(&self) -> Result<(), ExportError> {
        // ExportFormat enum already ensures only valid formats are accepted
        Ok(())
    }
}

/// Helper struct for export response metadata
#[derive(Debug, Clone)]
pub struct ExportResponse {
    /// Generated filename with timestamp
    pub filename: String,
    /// MIME content type for the response
    pub content_type: String,
    /// Exported file content as bytes
    pub content: Vec<u8>,
}

impl ExportResponse {
    pub fn new(format: &ExportFormat, content: Vec<u8>) -> Self {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("bills_export_{}.{}", timestamp, format);
        let content_type = match format {
            ExportFormat::Csv => "text/csv; charset=utf-8".to_string(),
            ExportFormat::Xlsx => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string(),
        };

        Self {
            filename,
            content_type,
            content,
        }
    }

    /// Get the filename for the export
    pub fn filename(&self) -> &str {
        &self.filename
    }

    /// Get the content type for HTTP headers
    pub fn content_type(&self) -> &str {
        &self.content_type
    }

    /// Get the file content as bytes
    pub fn content(&self) -> &[u8] {
        &self.content
    }

    /// Get the size of the exported content
    pub fn content_length(&self) -> usize {
        self.content.len()
    }
}

/// Export-specific error types
#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    /// Database query failed
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    /// CSV/XLSX serialization failed
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// File I/O operation failed
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Invalid export format or parameters
    #[error("Invalid export parameters: {0}")]
    InvalidParams(String),

    /// No data available for export
    #[error("No data available for export")]
    NoData,
}

impl ExportError {
    pub fn serialization(msg: impl Into<String>) -> Self {
        ExportError::SerializationError(msg.into())
    }

    pub fn invalid_params(msg: impl Into<String>) -> Self {
        ExportError::InvalidParams(msg.into())
    }
}

// Convert CSV writer errors to ExportError
impl From<csv::Error> for ExportError {
    fn from(err: csv::Error) -> Self {
        ExportError::SerializationError(format!("CSV error: {}", err))
    }
}

// Convert XLSX writer errors to ExportError
impl From<rust_xlsxwriter::XlsxError> for ExportError {
    fn from(err: rust_xlsxwriter::XlsxError) -> Self {
        ExportError::SerializationError(format!("XLSX error: {}", err))
    }
}