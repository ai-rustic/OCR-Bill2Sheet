//! API module organization
//!
//! This module organizes all API endpoints and handlers for the application.
//! It provides a clean interface for the Axum router setup and includes
//! error handling middleware for API responses.

use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Json, Response},
};
use serde_json::json;
use tracing::{error, warn};

// Public API modules
pub mod health;

// Re-export endpoint handlers for router setup
pub use health::{get_health, get_health_detail};

// Middleware functions are defined in this module and will be used in main.rs

/// Common API error response structure
#[derive(Debug)]
pub enum ApiError {
    InternalServerError(String),
    BadRequest(String),
    NotFound(String),
    ServiceUnavailable(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            ApiError::InternalServerError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {msg}"),
            ),
            ApiError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                format!("Bad request: {msg}"),
            ),
            ApiError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                format!("Not found: {msg}"),
            ),
            ApiError::ServiceUnavailable(msg) => (
                StatusCode::SERVICE_UNAVAILABLE,
                format!("Service unavailable: {msg}"),
            ),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16()
        }));

        (status, body).into_response()
    }
}

/// Global error handling middleware for API requests
///
/// This middleware catches any panics or unhandled errors in handlers and
/// converts them to properly formatted API error responses. It also provides
/// logging for debugging and monitoring purposes.
pub async fn error_handling_middleware(
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();

    // Call the next handler in the chain
    let response = next.run(request).await;

    // Check if the response status indicates an error
    let status = response.status();

    if status.is_client_error() || status.is_server_error() {
        // Log the error for monitoring and debugging
        if status.is_server_error() {
            error!(
                method = %method,
                uri = %uri,
                status = %status,
                "API request resulted in server error"
            );
        } else {
            warn!(
                method = %method,
                uri = %uri,
                status = %status,
                "API request resulted in client error"
            );
        }
    }

    response
}

/// Request timeout handling middleware
///
/// This middleware adds timeout handling to API requests to prevent
/// long-running requests from hanging indefinitely.
pub async fn timeout_middleware(
    request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, ApiError> {
    use tokio::time::{timeout, Duration};

    let method = request.method().clone();
    let uri = request.uri().clone();

    // Set a 30-second timeout for all requests
    match timeout(Duration::from_secs(30), next.run(request)).await {
        Ok(response) => Ok(response),
        Err(_) => {
            error!(
                method = %method,
                uri = %uri,
                "Request timed out after 30 seconds"
            );
            Err(ApiError::ServiceUnavailable(
                "Request timed out".to_string()
            ))
        }
    }
}

/// Fallback handler for unmatched routes
///
/// This handler returns a standardized 404 Not Found response for any
/// routes that don't match the defined API endpoints.
pub async fn not_found_handler() -> ApiError {
    ApiError::NotFound("The requested endpoint was not found".to_string())
}

/// Convert database errors to API errors
///
/// This function provides a centralized way to convert SQLx database errors
/// into appropriate API error responses with proper HTTP status codes.
impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => {
                ApiError::NotFound("Resource not found".to_string())
            }
            sqlx::Error::Database(db_err) => {
                error!("Database error: {}", db_err);
                ApiError::InternalServerError("Database operation failed".to_string())
            }
            sqlx::Error::Io(io_err) => {
                error!("Database I/O error: {}", io_err);
                ApiError::ServiceUnavailable("Database connection unavailable".to_string())
            }
            sqlx::Error::PoolTimedOut => {
                warn!("Database connection pool timeout");
                ApiError::ServiceUnavailable("Service temporarily unavailable".to_string())
            }
            _ => {
                error!("Unexpected database error: {}", err);
                ApiError::InternalServerError("An unexpected error occurred".to_string())
            }
        }
    }
}