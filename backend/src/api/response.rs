//! API response wrapper utilities
//!
//! This module provides standardized response structures for all API endpoints.
//! The ApiResponse wrapper ensures consistent response format across the application.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};

/// Standard API response wrapper for all endpoints
///
/// This struct provides a consistent response format with success status,
/// optional data payload, and optional error message. It follows the pattern
/// established by the existing health endpoints while providing a standardized
/// interface for all API responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// Whether the request was successful
    pub success: bool,
    /// Optional data payload for successful responses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// Optional error message for failed responses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    /// Create a successful response with data
    ///
    /// # Arguments
    /// * `data` - The data payload to include in the response
    ///
    /// # Returns
    /// ApiResponse with success=true and the provided data
    ///
    /// # Example
    /// ```rust
    /// use backend::api::response::ApiResponse;
    ///
    /// let response = ApiResponse::success("Hello, World!");
    /// assert!(response.success);
    /// assert_eq!(response.data, Some("Hello, World!"));
    /// assert_eq!(response.error, None);
    /// ```
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    /// Create an error response with message
    ///
    /// # Arguments
    /// * `message` - The error message to include in the response
    ///
    /// # Returns
    /// ApiResponse with success=false and the provided error message
    ///
    /// # Example
    /// ```rust
    /// use backend::api::response::ApiResponse;
    ///
    /// let response: ApiResponse<()> = ApiResponse::error("Something went wrong");
    /// assert!(!response.success);
    /// assert_eq!(response.data, None);
    /// assert_eq!(response.error, Some("Something went wrong".to_string()));
    /// ```
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.into()),
        }
    }

    /// Check if the response represents a successful operation
    ///
    /// # Returns
    /// True if success is true and no error is present
    pub fn is_success(&self) -> bool {
        self.success && self.error.is_none()
    }

    /// Check if the response represents an error
    ///
    /// # Returns
    /// True if success is false or an error message is present
    pub fn is_error(&self) -> bool {
        !self.success || self.error.is_some()
    }

    /// Get the data if the response is successful
    ///
    /// # Returns
    /// Some(data) if the response is successful, None otherwise
    pub fn get_data(&self) -> Option<&T> {
        if self.is_success() {
            self.data.as_ref()
        } else {
            None
        }
    }

    /// Get the error message if the response is an error
    ///
    /// # Returns
    /// Some(error_message) if the response is an error, None otherwise
    pub fn get_error(&self) -> Option<&String> {
        if self.is_error() {
            self.error.as_ref()
        } else {
            None
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    /// Convert ApiResponse into an Axum HTTP response
    ///
    /// Success responses return 200 OK, error responses return 400 Bad Request.
    /// For more specific error status codes, use the existing ApiError enum.
    fn into_response(self) -> axum::response::Response {
        let status_code = if self.success {
            StatusCode::OK
        } else {
            StatusCode::BAD_REQUEST
        };

        (status_code, Json(self)).into_response()
    }
}

// Convenience type aliases for common response patterns
pub type EmptyResponse = ApiResponse<()>;
pub type StringResponse = ApiResponse<String>;
pub type JsonResponse<T> = ApiResponse<T>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_success_response() {
        let response = ApiResponse::success("test data");
        assert!(response.success);
        assert_eq!(response.data, Some("test data"));
        assert_eq!(response.error, None);
        assert!(response.is_success());
        assert!(!response.is_error());
    }

    #[test]
    fn test_error_response() {
        let response: ApiResponse<()> = ApiResponse::error("test error");
        assert!(!response.success);
        assert_eq!(response.data, None);
        assert_eq!(response.error, Some("test error".to_string()));
        assert!(!response.is_success());
        assert!(response.is_error());
    }

    #[test]
    fn test_success_serialization() {
        let response = ApiResponse::success(42);
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"success\":true"));
        assert!(json.contains("\"data\":42"));
        assert!(!json.contains("error"));
    }

    #[test]
    fn test_error_serialization() {
        let response: ApiResponse<()> = ApiResponse::error("test error");
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"success\":false"));
        assert!(json.contains("\"error\":\"test error\""));
        assert!(!json.contains("data"));
    }

    #[test]
    fn test_get_data() {
        let success_response = ApiResponse::success("test");
        assert_eq!(success_response.get_data(), Some(&"test"));

        let error_response: ApiResponse<&str> = ApiResponse::error("error");
        assert_eq!(error_response.get_data(), None);
    }

    #[test]
    fn test_get_error() {
        let error_response: ApiResponse<()> = ApiResponse::error("test error");
        assert_eq!(error_response.get_error(), Some(&"test error".to_string()));

        let success_response = ApiResponse::success("data");
        assert_eq!(success_response.get_error(), None);
    }
}