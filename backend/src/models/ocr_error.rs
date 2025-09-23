use axum::{response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;

/// Comprehensive error handling for OCR processing operations
///
/// This enum covers all types of errors that can occur during Gemini OCR processing,
/// from configuration issues to API failures. Each error type maps to appropriate
/// HTTP status codes and provides detailed error information for debugging.
#[derive(Debug, thiserror::Error)]
pub enum ProcessingError {
    /// Configuration errors (missing API key, invalid settings)
    #[error("Configuration error: {message}")]
    Configuration { message: String },

    /// Input validation errors (invalid image format, size limits exceeded)
    #[error("Validation error: {message}")]
    Validation {
        message: String,
        field: Option<String>,
        image_index: Option<usize>,
    },

    /// Gemini API errors (API failures, authentication issues)
    #[error("API error: {message}")]
    Api {
        message: String,
        status_code: Option<u16>,
        retry_after_seconds: Option<u64>,
        image_index: Option<usize>,
    },

    /// Network connection errors
    #[error("Network error: {message}")]
    Network {
        message: String,
        image_index: Option<usize>,
    },

    /// Processing timeout errors
    #[error("Timeout error: {message}")]
    Timeout {
        message: String,
        timeout_seconds: u64,
        image_index: Option<usize>,
    },

    /// Response parsing errors (malformed API responses)
    #[error("Parse error: {message}")]
    Parse {
        message: String,
        response_body: Option<String>,
        image_index: Option<usize>,
    },

    /// Rate limiting errors
    #[error("Rate limit exceeded: {message}")]
    RateLimit {
        message: String,
        retry_after_seconds: u64,
        quota_remaining: Option<u32>,
    },

    /// Service unavailable errors
    #[error("Service unavailable: {message}")]
    ServiceUnavailable {
        message: String,
        service_status: Option<String>,
        estimated_recovery_time: Option<chrono::DateTime<chrono::Utc>>,
    },

    /// Internal processing errors
    #[error("Internal error: {message}")]
    Internal {
        message: String,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

/// Simplified error type enum for API responses
///
/// Maps ProcessingError variants to simple string identifiers
/// that match the OCR API contract error types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorType {
    /// Missing API key or invalid configuration
    ConfigurationError,
    /// Invalid image format, size exceeded, etc.
    ValidationError,
    /// Gemini API returned an error response
    ApiError,
    /// Processing timeout exceeded
    TimeoutError,
    /// Network connection issues
    NetworkError,
    /// Invalid API response format
    ParseError,
    /// Rate limiting errors
    RateLimitError,
    /// Service unavailable
    ServiceUnavailableError,
    /// Internal server errors
    InternalError,
}

/// Detailed error response structure for API clients
///
/// Provides comprehensive error information including error type,
/// human-readable message, and additional context for debugging.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingErrorResponse {
    /// Category of error encountered
    pub error_type: ErrorType,
    /// Human-readable error description
    pub message: String,
    /// Index of image that caused error (if applicable)
    pub image_index: Option<usize>,
    /// Suggested retry delay for rate limit errors
    pub retry_after_seconds: Option<u64>,
    /// Additional error details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl ProcessingError {
    /// Creates a configuration error
    pub fn configuration<S: Into<String>>(message: S) -> Self {
        Self::Configuration {
            message: message.into(),
        }
    }

    /// Creates a validation error with optional field and image context
    pub fn validation<S: Into<String>>(
        message: S,
        field: Option<String>,
        image_index: Option<usize>,
    ) -> Self {
        Self::Validation {
            message: message.into(),
            field,
            image_index,
        }
    }

    /// Creates an API error with optional status code and retry information
    pub fn api<S: Into<String>>(
        message: S,
        status_code: Option<u16>,
        retry_after_seconds: Option<u64>,
        image_index: Option<usize>,
    ) -> Self {
        Self::Api {
            message: message.into(),
            status_code,
            retry_after_seconds,
            image_index,
        }
    }

    /// Creates a network error
    pub fn network<S: Into<String>>(message: S, image_index: Option<usize>) -> Self {
        Self::Network {
            message: message.into(),
            image_index,
        }
    }

    /// Creates a timeout error
    pub fn timeout<S: Into<String>>(
        message: S,
        timeout_seconds: u64,
        image_index: Option<usize>,
    ) -> Self {
        Self::Timeout {
            message: message.into(),
            timeout_seconds,
            image_index,
        }
    }

    /// Creates a parse error with optional response body
    pub fn parse<S: Into<String>>(
        message: S,
        response_body: Option<String>,
        image_index: Option<usize>,
    ) -> Self {
        Self::Parse {
            message: message.into(),
            response_body,
            image_index,
        }
    }

    /// Creates a rate limit error
    pub fn rate_limit<S: Into<String>>(
        message: S,
        retry_after_seconds: u64,
        quota_remaining: Option<u32>,
    ) -> Self {
        Self::RateLimit {
            message: message.into(),
            retry_after_seconds,
            quota_remaining,
        }
    }

    /// Creates a service unavailable error
    pub fn service_unavailable<S: Into<String>>(
        message: S,
        service_status: Option<String>,
        estimated_recovery_time: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        Self::ServiceUnavailable {
            message: message.into(),
            service_status,
            estimated_recovery_time,
        }
    }

    /// Creates an internal error with optional source error
    pub fn internal<S: Into<String>>(
        message: S,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    ) -> Self {
        Self::Internal {
            message: message.into(),
            source,
        }
    }

    /// Converts the error to an ErrorType enum value
    pub fn error_type(&self) -> ErrorType {
        match self {
            Self::Configuration { .. } => ErrorType::ConfigurationError,
            Self::Validation { .. } => ErrorType::ValidationError,
            Self::Api { .. } => ErrorType::ApiError,
            Self::Network { .. } => ErrorType::NetworkError,
            Self::Timeout { .. } => ErrorType::TimeoutError,
            Self::Parse { .. } => ErrorType::ParseError,
            Self::RateLimit { .. } => ErrorType::RateLimitError,
            Self::ServiceUnavailable { .. } => ErrorType::ServiceUnavailableError,
            Self::Internal { .. } => ErrorType::InternalError,
        }
    }

    /// Gets the image index associated with this error, if any
    pub fn image_index(&self) -> Option<usize> {
        match self {
            Self::Validation { image_index, .. } => *image_index,
            Self::Api { image_index, .. } => *image_index,
            Self::Network { image_index, .. } => *image_index,
            Self::Timeout { image_index, .. } => *image_index,
            Self::Parse { image_index, .. } => *image_index,
            _ => None,
        }
    }

    /// Gets the retry delay for rate limit errors
    pub fn retry_after_seconds(&self) -> Option<u64> {
        match self {
            Self::Api { retry_after_seconds, .. } => *retry_after_seconds,
            Self::RateLimit { retry_after_seconds, .. } => Some(*retry_after_seconds),
            _ => None,
        }
    }

    /// Converts the error to a structured response for API clients
    pub fn to_response(&self) -> ProcessingErrorResponse {
        let mut details = serde_json::Map::new();

        match self {
            Self::Configuration { .. } => {},
            Self::Validation { field, .. } => {
                if let Some(field) = field {
                    details.insert("field".to_string(), json!(field));
                }
            },
            Self::Api { status_code, .. } => {
                if let Some(status_code) = status_code {
                    details.insert("status_code".to_string(), json!(status_code));
                }
            },
            Self::Network { .. } => {},
            Self::Timeout { timeout_seconds, .. } => {
                details.insert("timeout_seconds".to_string(), json!(timeout_seconds));
            },
            Self::Parse { response_body, .. } => {
                if let Some(body) = response_body {
                    details.insert("response_body".to_string(), json!(body));
                }
            },
            Self::RateLimit { quota_remaining, .. } => {
                if let Some(quota) = quota_remaining {
                    details.insert("quota_remaining".to_string(), json!(quota));
                }
            },
            Self::ServiceUnavailable { service_status, estimated_recovery_time, .. } => {
                if let Some(status) = service_status {
                    details.insert("service_status".to_string(), json!(status));
                }
                if let Some(recovery_time) = estimated_recovery_time {
                    details.insert("estimated_recovery_time".to_string(), json!(recovery_time));
                }
            },
            Self::Internal { .. } => {},
        }

        ProcessingErrorResponse {
            error_type: self.error_type(),
            message: self.to_string(),
            image_index: self.image_index(),
            retry_after_seconds: self.retry_after_seconds(),
            details: if details.is_empty() { None } else { Some(json!(details)) },
        }
    }
}

impl IntoResponse for ProcessingError {
    fn into_response(self) -> axum::response::Response {
        let (status, response) = match &self {
            ProcessingError::Configuration { .. } => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_response())
            }
            ProcessingError::Validation { .. } => {
                (StatusCode::BAD_REQUEST, self.to_response())
            }
            ProcessingError::Api { status_code, .. } => {
                let status = status_code
                    .and_then(|code| StatusCode::from_u16(code).ok())
                    .unwrap_or(StatusCode::BAD_GATEWAY);
                (status, self.to_response())
            }
            ProcessingError::Network { .. } => {
                (StatusCode::BAD_GATEWAY, self.to_response())
            }
            ProcessingError::Timeout { .. } => {
                (StatusCode::REQUEST_TIMEOUT, self.to_response())
            }
            ProcessingError::Parse { .. } => {
                (StatusCode::BAD_GATEWAY, self.to_response())
            }
            ProcessingError::RateLimit { .. } => {
                (StatusCode::TOO_MANY_REQUESTS, self.to_response())
            }
            ProcessingError::ServiceUnavailable { .. } => {
                (StatusCode::SERVICE_UNAVAILABLE, self.to_response())
            }
            ProcessingError::Internal { .. } => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_response())
            }
        };

        let mut json_response = axum::Json(json!({
            "success": false,
            "error": response,
            "status": status.as_u16()
        })).into_response();

        // Add retry-after header for rate limit and service unavailable errors
        if let Some(retry_after) = self.retry_after_seconds() {
            if let Ok(header_value) = retry_after.to_string().parse() {
                json_response.headers_mut().insert("retry-after", header_value);
            }
        }

        *json_response.status_mut() = status;
        json_response
    }
}

impl From<reqwest::Error> for ProcessingError {
    fn from(error: reqwest::Error) -> Self {
        if error.is_timeout() {
            Self::timeout(
                format!("HTTP request timeout: {}", error),
                30, // Default timeout
                None,
            )
        } else if error.is_connect() {
            Self::network(
                format!("Connection error: {}", error),
                None,
            )
        } else if error.is_request() {
            Self::validation(
                format!("Invalid request: {}", error),
                None,
                None,
            )
        } else {
            Self::api(
                format!("HTTP error: {}", error),
                error.status().map(|s| s.as_u16()),
                None,
                None,
            )
        }
    }
}

impl From<serde_json::Error> for ProcessingError {
    fn from(error: serde_json::Error) -> Self {
        Self::parse(
            format!("JSON parsing error: {}", error),
            None,
            None,
        )
    }
}

impl From<chrono::ParseError> for ProcessingError {
    fn from(error: chrono::ParseError) -> Self {
        Self::parse(
            format!("Date parsing error: {}", error),
            None,
            None,
        )
    }
}

impl From<rust_decimal::Error> for ProcessingError {
    fn from(error: rust_decimal::Error) -> Self {
        Self::parse(
            format!("Decimal parsing error: {}", error),
            None,
            None,
        )
    }
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorType::ConfigurationError => write!(f, "configuration_error"),
            ErrorType::ValidationError => write!(f, "validation_error"),
            ErrorType::ApiError => write!(f, "api_error"),
            ErrorType::TimeoutError => write!(f, "timeout_error"),
            ErrorType::NetworkError => write!(f, "network_error"),
            ErrorType::ParseError => write!(f, "parse_error"),
            ErrorType::RateLimitError => write!(f, "rate_limit_error"),
            ErrorType::ServiceUnavailableError => write!(f, "service_unavailable_error"),
            ErrorType::InternalError => write!(f, "internal_error"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configuration_error_creation() {
        let error = ProcessingError::configuration("Missing API key");
        assert!(matches!(error, ProcessingError::Configuration { .. }));
        assert_eq!(error.error_type(), ErrorType::ConfigurationError);
        assert_eq!(error.image_index(), None);
    }

    #[test]
    fn test_validation_error_creation() {
        let error = ProcessingError::validation(
            "Invalid image format",
            Some("mime_type".to_string()),
            Some(0),
        );
        assert!(matches!(error, ProcessingError::Validation { .. }));
        assert_eq!(error.error_type(), ErrorType::ValidationError);
        assert_eq!(error.image_index(), Some(0));
    }

    #[test]
    fn test_api_error_creation() {
        let error = ProcessingError::api(
            "Gemini API error",
            Some(429),
            Some(60),
            Some(1),
        );
        assert!(matches!(error, ProcessingError::Api { .. }));
        assert_eq!(error.error_type(), ErrorType::ApiError);
        assert_eq!(error.image_index(), Some(1));
        assert_eq!(error.retry_after_seconds(), Some(60));
    }

    #[test]
    fn test_rate_limit_error_creation() {
        let error = ProcessingError::rate_limit(
            "Rate limit exceeded",
            120,
            Some(1000),
        );
        assert!(matches!(error, ProcessingError::RateLimit { .. }));
        assert_eq!(error.error_type(), ErrorType::RateLimitError);
        assert_eq!(error.retry_after_seconds(), Some(120));
    }

    #[test]
    fn test_timeout_error_creation() {
        let error = ProcessingError::timeout(
            "Processing timeout",
            45,
            Some(2),
        );
        assert!(matches!(error, ProcessingError::Timeout { .. }));
        assert_eq!(error.error_type(), ErrorType::TimeoutError);
        assert_eq!(error.image_index(), Some(2));
    }

    #[test]
    fn test_error_response_conversion() {
        let error = ProcessingError::validation(
            "Invalid image format",
            Some("mime_type".to_string()),
            Some(0),
        );

        let response = error.to_response();
        assert_eq!(response.error_type, ErrorType::ValidationError);
        assert_eq!(response.message, "Validation error: Invalid image format");
        assert_eq!(response.image_index, Some(0));
        assert!(response.details.is_some());
    }

    #[test]
    fn test_reqwest_error_conversion() {
        // This test would require setting up a mock reqwest error
        // For now, we'll just test the error type mapping logic
        let error = ProcessingError::network("Connection failed", None);
        assert!(matches!(error, ProcessingError::Network { .. }));
        assert_eq!(error.error_type(), ErrorType::NetworkError);
    }

    #[test]
    fn test_error_type_display() {
        assert_eq!(ErrorType::ConfigurationError.to_string(), "configuration_error");
        assert_eq!(ErrorType::ValidationError.to_string(), "validation_error");
        assert_eq!(ErrorType::ApiError.to_string(), "api_error");
        assert_eq!(ErrorType::RateLimitError.to_string(), "rate_limit_error");
    }

    #[test]
    fn test_service_unavailable_error() {
        let recovery_time = chrono::Utc::now() + chrono::Duration::hours(1);
        let error = ProcessingError::service_unavailable(
            "Gemini service temporarily unavailable",
            Some("maintenance".to_string()),
            Some(recovery_time),
        );

        assert!(matches!(error, ProcessingError::ServiceUnavailable { .. }));
        assert_eq!(error.error_type(), ErrorType::ServiceUnavailableError);

        let response = error.to_response();
        assert!(response.details.is_some());
    }

    #[test]
    fn test_internal_error_with_source() {
        let source_error = std::io::Error::new(std::io::ErrorKind::Other, "test error");
        let error = ProcessingError::internal(
            "Internal processing failed",
            Some(Box::new(source_error)),
        );

        assert!(matches!(error, ProcessingError::Internal { .. }));
        assert_eq!(error.error_type(), ErrorType::InternalError);
    }
}