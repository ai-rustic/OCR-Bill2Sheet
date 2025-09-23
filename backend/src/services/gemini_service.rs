//! Gemini AI API Service
//!
//! This module provides the service layer for integrating with Google's Gemini AI API
//! for Vietnamese bill/invoice OCR processing and structured data extraction.

use base64::Engine;
use reqwest::Client;
use serde_json::{json, Value};
use std::time::{Duration, Instant};
use tokio::time::{sleep, timeout};
use tracing::{debug, info, warn, error, instrument};

use crate::models::{GeminiRequest, GeminiResponse};
use crate::utils::env::get_gemini_api_key;

/// Error types for Gemini API operations
#[derive(Debug, thiserror::Error)]
pub enum GeminiError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("API response error: {status} - {message}")]
    ApiError { status: u16, message: String },

    #[error("Rate limit exceeded (429). Retry after: {retry_after:?} seconds")]
    RateLimitExceeded { retry_after: Option<u64> },

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Image encoding error: {0}")]
    ImageEncodingError(String),

    #[error("Invalid API response format: {0}")]
    InvalidResponseFormat(String),

    #[error("Authentication failed: Invalid API key")]
    AuthenticationFailed,

    #[error("Request timeout after {seconds} seconds")]
    Timeout { seconds: u64 },

    #[error("Network error: {0}")]
    NetworkError(String),
}

/// Gemini AI API service configuration
#[derive(Debug, Clone)]
pub struct GeminiConfig {
    /// API base URL
    pub base_url: String,
    /// Request timeout in seconds
    pub timeout_seconds: u64,
    /// Maximum retry attempts for rate limiting
    pub max_retries: u32,
    /// Base delay between retries in milliseconds
    pub retry_delay_ms: u64,
    /// Model name to use for API calls
    pub model: String,
}

impl Default for GeminiConfig {
    fn default() -> Self {
        Self {
            base_url: "https://generativelanguage.googleapis.com/v1beta".to_string(),
            timeout_seconds: 30,
            max_retries: 3,
            retry_delay_ms: 1000,
            model: "gemini-1.5-flash".to_string(),
        }
    }
}

/// Service for interacting with Gemini AI API
///
/// Provides methods for extracting structured bill data from Vietnamese invoices
/// using Google's Gemini AI API with proper error handling and rate limiting.
pub struct GeminiService {
    client: Client,
    api_key: String,
    config: GeminiConfig,
}

impl GeminiService {
    /// Create a new GeminiService instance
    ///
    /// # Arguments
    /// * `config` - Optional configuration for the service. Uses default if None.
    ///
    /// # Returns
    /// Result containing the GeminiService or an error if API key is missing
    pub fn new(config: Option<GeminiConfig>) -> Result<Self, GeminiError> {
        let api_key = get_gemini_api_key();
        let config = config.unwrap_or_default();

        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(|e| GeminiError::NetworkError(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            client,
            api_key,
            config,
        })
    }

    /// Create a new GeminiService with default configuration
    pub fn with_default_config() -> Result<Self, GeminiError> {
        Self::new(None)
    }

    /// Extract bill data from image bytes
    ///
    /// # Arguments
    /// * `image_data` - Raw image bytes (JPEG, PNG, etc.)
    ///
    /// # Returns
    /// Result containing extracted GeminiResponse or error
    #[instrument(skip(self, image_data), fields(image_size = image_data.len()))]
    pub async fn extract_bill_data(&self, image_data: &[u8]) -> Result<GeminiResponse, GeminiError> {
        let start_time = Instant::now();
        info!("Starting Gemini bill data extraction for image of {} bytes", image_data.len());

        // Base64 encode the image
        debug!("Encoding image to base64");
        let encoded_image = self.encode_image(image_data)?;
        debug!("Successfully encoded image to base64, length: {}", encoded_image.len());

        // Create the request with default Vietnamese bill extraction prompt
        let request = GeminiRequest::for_bill_extraction(encoded_image);
        debug!("Created GeminiRequest with Vietnamese bill extraction prompt");

        // Send request to Gemini API with retry logic
        match self.send_request_with_retry(&request).await {
            Ok(response) => {
                let duration = start_time.elapsed();
                info!("Successfully extracted bill data from Gemini API in {:?}", duration);
                debug!("Extracted data contains: form_no={:?}, invoice_no={:?}",
                       response.form_no, response.invoice_no);
                Ok(response)
            }
            Err(e) => {
                let duration = start_time.elapsed();
                error!("Failed to extract bill data from Gemini API after {:?}: {}", duration, e);
                Err(e)
            }
        }
    }

    /// Extract bill data with custom prompt
    ///
    /// # Arguments
    /// * `image_data` - Raw image bytes (JPEG, PNG, etc.)
    /// * `custom_prompt` - Custom extraction prompt
    ///
    /// # Returns
    /// Result containing extracted GeminiResponse or error
    pub async fn extract_bill_data_with_prompt(
        &self,
        image_data: &[u8],
        custom_prompt: String,
    ) -> Result<GeminiResponse, GeminiError> {
        let encoded_image = self.encode_image(image_data)?;
        let request = GeminiRequest::new(encoded_image, custom_prompt);
        self.send_request_with_retry(&request).await
    }

    /// Send request to Gemini API with retry logic for rate limiting
    #[instrument(skip(self, request))]
    async fn send_request_with_retry(&self, request: &GeminiRequest) -> Result<GeminiResponse, GeminiError> {
        let mut last_error = None;

        for attempt in 0..=self.config.max_retries {
            debug!("Gemini API request attempt {} of {}", attempt + 1, self.config.max_retries + 1);

            match self.send_gemini_request(request).await {
                Ok(response) => {
                    if attempt > 0 {
                        info!("Gemini API request succeeded on attempt {}", attempt + 1);
                    }
                    return Ok(response);
                },
                Err(GeminiError::RateLimitExceeded { retry_after }) => {
                    if attempt < self.config.max_retries {
                        let delay = retry_after.unwrap_or(self.config.retry_delay_ms / 1000);
                        let delay_ms = (delay * 1000).min(30000); // Cap at 30 seconds
                        warn!("Gemini API rate limit exceeded on attempt {}. Retrying in {}ms",
                              attempt + 1, delay_ms);
                        sleep(Duration::from_millis(delay_ms)).await;
                        continue;
                    } else {
                        error!("Gemini API rate limit exceeded. Max retries ({}) reached", self.config.max_retries);
                        last_error = Some(GeminiError::RateLimitExceeded { retry_after });
                    }
                },
                Err(e) => {
                    error!("Gemini API request failed on attempt {}: {}", attempt + 1, e);
                    last_error = Some(e);
                    break;
                }
            }
        }

        Err(last_error.unwrap_or(GeminiError::NetworkError("Unknown error".to_string())))
    }

    /// Send a single request to Gemini API
    #[instrument(skip(self, request))]
    async fn send_gemini_request(&self, request: &GeminiRequest) -> Result<GeminiResponse, GeminiError> {
        let start_time = Instant::now();
        let url = format!(
            "{}/models/{}:generateContent",
            self.config.base_url,
            self.config.model
        );
        debug!("Sending request to Gemini API: {}", url);

        // Build the request payload according to Gemini API format
        let payload = json!({
            "contents": [{
                "parts": [
                    {
                        "text": request.prompt
                    },
                    {
                        "inlineData": {
                            "mimeType": "image/jpeg", // Assume JPEG for now
                            "data": request.image_data
                        }
                    }
                ]
            }],
            "generationConfig": {
                "temperature": 0.1,
                "topK": 1,
                "topP": 0.8,
                "maxOutputTokens": 2048,
                "stopSequences": []
            },
            "safetySettings": [
                {
                    "category": "HARM_CATEGORY_HARASSMENT",
                    "threshold": "BLOCK_MEDIUM_AND_ABOVE"
                },
                {
                    "category": "HARM_CATEGORY_HATE_SPEECH",
                    "threshold": "BLOCK_MEDIUM_AND_ABOVE"
                },
                {
                    "category": "HARM_CATEGORY_SEXUALLY_EXPLICIT",
                    "threshold": "BLOCK_MEDIUM_AND_ABOVE"
                },
                {
                    "category": "HARM_CATEGORY_DANGEROUS_CONTENT",
                    "threshold": "BLOCK_MEDIUM_AND_ABOVE"
                }
            ]
        });

        // Send the request with timeout
        debug!("Sending HTTP request to Gemini API with timeout of {} seconds", self.config.timeout_seconds);
        let response = timeout(
            Duration::from_secs(self.config.timeout_seconds),
            self.client
                .post(&url)
                .header("Content-Type", "application/json")
                .query(&[("key", &self.api_key)])
                .json(&payload)
                .send()
        )
        .await
        .map_err(|_| {
            error!("Gemini API request timed out after {} seconds", self.config.timeout_seconds);
            GeminiError::Timeout { seconds: self.config.timeout_seconds }
        })?
        .map_err(|e| {
            error!("HTTP request to Gemini API failed: {}", e);
            GeminiError::RequestFailed(e)
        })?;

        // Handle HTTP status codes
        let status = response.status();
        debug!("Received HTTP response with status: {}", status);

        if status == 429 {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse().ok());
            warn!("Gemini API rate limit exceeded (429). Retry after: {:?}", retry_after);
            return Err(GeminiError::RateLimitExceeded { retry_after });
        }

        if status == 401 || status == 403 {
            error!("Gemini API authentication failed ({}). Check API key", status);
            return Err(GeminiError::AuthenticationFailed);
        }

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            error!("Gemini API returned error status {}: {}", status, error_text);
            return Err(GeminiError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        // Parse the response
        debug!("Parsing JSON response from Gemini API");
        let response_json: Value = response.json().await.map_err(|e| {
            error!("Failed to parse JSON response: {}", e);
            GeminiError::RequestFailed(e)
        })?;

        let api_duration = start_time.elapsed();
        debug!("Gemini API call completed in {:?}", api_duration);

        match self.parse_gemini_response(response_json).await {
            Ok(parsed_response) => {
                info!("Successfully parsed Gemini response in {:?}", api_duration);
                Ok(parsed_response)
            }
            Err(e) => {
                error!("Failed to parse Gemini response: {}", e);
                Err(e)
            }
        }
    }

    /// Parse Gemini API response and extract GeminiResponse
    async fn parse_gemini_response(&self, response: Value) -> Result<GeminiResponse, GeminiError> {
        // Extract the generated text from Gemini response
        let candidates = response["candidates"]
            .as_array()
            .ok_or_else(|| GeminiError::InvalidResponseFormat("Missing candidates array".to_string()))?;

        if candidates.is_empty() {
            return Err(GeminiError::InvalidResponseFormat("Empty candidates array".to_string()));
        }

        let content = &candidates[0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or_else(|| GeminiError::InvalidResponseFormat("Missing text content".to_string()))?;

        // Parse the JSON response from Gemini
        // Clean the response to handle potential markdown formatting
        let cleaned_content = self.clean_json_response(content);

        let gemini_response: GeminiResponse = serde_json::from_str(&cleaned_content)
            .map_err(|e| {
                GeminiError::InvalidResponseFormat(
                    format!("Failed to parse JSON response: {}. Response: {}", e, cleaned_content)
                )
            })?;

        Ok(gemini_response)
    }

    /// Clean JSON response from potential markdown formatting
    fn clean_json_response(&self, content: &str) -> String {
        // Remove markdown code blocks if present
        let content = content.trim();

        // Remove ```json and ``` markers
        let content = if content.starts_with("```json") {
            content.strip_prefix("```json").unwrap_or(content)
        } else if content.starts_with("```") {
            content.strip_prefix("```").unwrap_or(content)
        } else {
            content
        };

        let content = if content.ends_with("```") {
            content.strip_suffix("```").unwrap_or(content)
        } else {
            content
        };

        content.trim().to_string()
    }

    /// Encode image data to base64 string
    fn encode_image(&self, image_data: &[u8]) -> Result<String, GeminiError> {
        if image_data.is_empty() {
            return Err(GeminiError::ImageEncodingError("Image data is empty".to_string()));
        }

        // Validate image format (basic check)
        if !self.is_valid_image_format(image_data) {
            return Err(GeminiError::ImageEncodingError(
                "Unsupported image format. Only JPEG and PNG are supported.".to_string()
            ));
        }

        Ok(base64::engine::general_purpose::STANDARD.encode(image_data))
    }

    /// Basic image format validation
    fn is_valid_image_format(&self, data: &[u8]) -> bool {
        if data.len() < 8 {
            return false;
        }

        // Check for JPEG magic bytes
        if data.starts_with(&[0xFF, 0xD8, 0xFF]) {
            return true;
        }

        // Check for PNG magic bytes
        if data.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]) {
            return true;
        }

        false
    }

    /// Test the API connection with a simple request
    pub async fn test_connection(&self) -> Result<(), GeminiError> {
        // Create a minimal test image (1x1 PNG)
        let test_image = vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D,
            0x49, 0x48, 0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
            0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53, 0xDE, 0x00, 0x00, 0x00,
            0x0C, 0x49, 0x44, 0x41, 0x54, 0x08, 0xD7, 0x63, 0xF8, 0x00, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44,
            0xAE, 0x42, 0x60, 0x82
        ];

        let encoded = self.encode_image(&test_image)?;
        let test_request = GeminiRequest::new(
            encoded,
            "Describe this image briefly.".to_string()
        );

        match self.send_gemini_request(&test_request).await {
            Ok(_) => Ok(()),
            Err(GeminiError::InvalidResponseFormat(_)) => {
                // For connection test, we don't care about response format,
                // just that we can reach the API
                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    /// Get the current configuration
    pub fn get_config(&self) -> &GeminiConfig {
        &self.config
    }

    /// Update the service configuration
    pub fn update_config(&mut self, config: GeminiConfig) {
        self.config = config;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_format_validation() {
        let service = GeminiService::with_default_config().unwrap();

        // Test JPEG format
        let jpeg_data = vec![0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46];
        assert!(service.is_valid_image_format(&jpeg_data));

        // Test PNG format
        let png_data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        assert!(service.is_valid_image_format(&png_data));

        // Test invalid format
        let invalid_data = vec![0x00, 0x01, 0x02, 0x03];
        assert!(!service.is_valid_image_format(&invalid_data));

        // Test empty data
        assert!(!service.is_valid_image_format(&[]));
    }

    #[test]
    fn test_clean_json_response() {
        let service = GeminiService::with_default_config().unwrap();

        // Test with markdown code blocks
        let markdown_json = "```json\n{\"test\": \"value\"}\n```";
        assert_eq!(service.clean_json_response(markdown_json), "{\"test\": \"value\"}");

        // Test with simple code blocks
        let simple_markdown = "```\n{\"test\": \"value\"}\n```";
        assert_eq!(service.clean_json_response(simple_markdown), "{\"test\": \"value\"}");

        // Test with plain JSON
        let plain_json = "{\"test\": \"value\"}";
        assert_eq!(service.clean_json_response(plain_json), "{\"test\": \"value\"}");

        // Test with extra whitespace
        let whitespace_json = "  \n  {\"test\": \"value\"}  \n  ";
        assert_eq!(service.clean_json_response(whitespace_json), "{\"test\": \"value\"}");
    }

    #[test]
    fn test_encode_image() {
        let service = GeminiService::with_default_config().unwrap();

        // Test valid JPEG
        let jpeg_data = vec![0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46];
        assert!(service.encode_image(&jpeg_data).is_ok());

        // Test empty data
        assert!(service.encode_image(&[]).is_err());

        // Test invalid format
        let invalid_data = vec![0x00, 0x01, 0x02, 0x03];
        assert!(service.encode_image(&invalid_data).is_err());
    }

    #[test]
    fn test_default_config() {
        let config = GeminiConfig::default();
        assert_eq!(config.base_url, "https://generativelanguage.googleapis.com/v1beta");
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.model, "gemini-1.5-flash");
    }

    #[tokio::test]
    async fn test_service_creation() {
        // This test requires GEMINI_API_KEY environment variable
        // Skip if not available in test environment
        if std::env::var("GEMINI_API_KEY").is_err() {
            return;
        }

        let service = GeminiService::with_default_config();
        assert!(service.is_ok());
    }
}