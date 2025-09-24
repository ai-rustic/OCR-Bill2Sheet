//! Export API endpoints
//!
//! This module contains the GET /api/bills/export handler for exporting bills
//! in CSV and XLSX formats. The handler follows the established API patterns
//! with proper error handling middleware integration and response formatting.

use axum::{
    extract::{Query, State},
    http::{header, StatusCode},
    response::IntoResponse,
};
use tracing::{error, info, warn};

use crate::{
    api::ApiError,
    config::ConnectionPool,
    models::export::{ExportError, ExportParams},
    services::export_service::ExportService,
};

/// Convert ExportError to ApiError for middleware integration
impl From<ExportError> for ApiError {
    fn from(err: ExportError) -> Self {
        match err {
            ExportError::DatabaseError(db_err) => {
                error!("Database error during export: {}", db_err);
                ApiError::InternalServerError("Failed to retrieve bills from database".to_string())
            }
            ExportError::SerializationError(ser_err) => {
                error!("Serialization error during export: {}", ser_err);
                ApiError::InternalServerError("Failed to generate export file".to_string())
            }
            ExportError::IoError(io_err) => {
                error!("I/O error during export: {}", io_err);
                ApiError::InternalServerError("File generation error".to_string())
            }
            ExportError::InvalidParams(param_err) => {
                warn!("Invalid export parameters: {}", param_err);
                ApiError::BadRequest(format!("Invalid export parameters: {}", param_err))
            }
            ExportError::NoData => {
                info!("No bills found for export");
                ApiError::NotFound("No bills available for export".to_string())
            }
        }
    }
}

/// GET /api/bills/export endpoint handler
///
/// Exports bills from the database in CSV or XLSX format based on the format parameter.
/// Uses the ExportService to generate the export data with proper Vietnamese text encoding.
/// Returns the file with appropriate headers for download.
///
/// This handler integrates with the API error handling middleware to ensure consistent
/// error response formatting and proper HTTP status code mapping:
/// - 400 Bad Request for invalid format parameters
/// - 404 Not Found when no bills are available for export
/// - 500 Internal Server Error for database, serialization, and I/O errors
///
/// # Query Parameters
/// - `format`: Export format (csv or xlsx)
///
/// # Returns
/// - 200 OK with exported file content and download headers
/// - Error responses handled by middleware via ApiError conversion
pub async fn export_bills(
    State(pool): State<ConnectionPool>,
    Query(params): Query<ExportParams>,
) -> Result<impl IntoResponse, ApiError> {
    info!("Export bills request received with format: {}", params.format);

    // Validate export parameters - conversion to ApiError happens automatically
    params.validate().map_err(|e| {
        warn!("Export parameter validation failed: {}", e);
        ApiError::BadRequest(format!("Invalid export parameters: {}", e))
    })?;

    // Create export service with the connection pool
    let export_service = ExportService::new(pool.pool().clone());

    // Generate export using the service - ExportError -> ApiError conversion is automatic
    let export_format = params.format.clone();
    let export_response = export_service.export_bills(params.format).await?;

    info!(
        "Export successful: {} bytes, format: {}",
        export_response.content_length(),
        export_format
    );

    // Set appropriate headers for file download
    let headers = [
        (header::CONTENT_TYPE, export_response.content_type()),
        (
            header::CONTENT_DISPOSITION,
            &format!("attachment; filename=\"{}\"", export_response.filename()),
        ),
        (header::CACHE_CONTROL, "no-cache, no-store, must-revalidate"),
        (header::PRAGMA, "no-cache"),
        (header::EXPIRES, "0"),
    ];

    Ok((StatusCode::OK, headers, export_response.content().to_vec()).into_response())
}