use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Configuration structure for Gemini AI API integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiConfig {
    /// Google AI Studio API key for Gemini service
    pub api_key: String,
    /// Gemini model to use (e.g., "gemini-pro-vision")
    pub model: String,
    /// API request timeout in seconds
    pub timeout_seconds: u64,
    /// Maximum image size allowed for processing (in MB)
    pub max_image_size_mb: u64,
    /// Gemini API base URL endpoint
    pub base_url: String,
}

impl GeminiConfig {
    /// Create a new GeminiConfig with default values
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: "gemini-pro-vision".to_string(),
            timeout_seconds: 45,
            max_image_size_mb: 20,
            base_url: "https://generativelanguage.googleapis.com/v1beta".to_string(),
        }
    }

    /// Create GeminiConfig from environment variables
    pub fn from_env() -> Result<Self, GeminiConfigError> {
        dotenvy::dotenv().ok(); // Load .env file if present

        let api_key = std::env::var("GEMINI_API_KEY").map_err(|_| {
            GeminiConfigError::Configuration(
                "GEMINI_API_KEY environment variable is required".to_string(),
            )
        })?;

        let model =
            std::env::var("GEMINI_MODEL").unwrap_or_else(|_| "gemini-pro-vision".to_string());

        let timeout_seconds = std::env::var("GEMINI_TIMEOUT_SECONDS")
            .unwrap_or_else(|_| "45".to_string())
            .parse()
            .map_err(|_| {
                GeminiConfigError::Configuration(
                    "GEMINI_TIMEOUT_SECONDS must be a valid number".to_string(),
                )
            })?;

        let max_image_size_mb = std::env::var("GEMINI_MAX_IMAGE_SIZE_MB")
            .unwrap_or_else(|_| "20".to_string())
            .parse()
            .map_err(|_| {
                GeminiConfigError::Configuration(
                    "GEMINI_MAX_IMAGE_SIZE_MB must be a valid number".to_string(),
                )
            })?;

        let base_url = std::env::var("GEMINI_BASE_URL")
            .unwrap_or_else(|_| "https://generativelanguage.googleapis.com/v1beta".to_string());

        let config = Self {
            api_key,
            model,
            timeout_seconds,
            max_image_size_mb,
            base_url,
        };

        // Validate configuration
        config.validate().map_err(|e|
            GeminiConfigError::Configuration(format!(
                "Environment configuration validation failed: {e}. Check your .env file or environment variables."
            ))
        )?;

        Ok(config)
    }

    /// Display configuration details for debugging (excludes sensitive API key)
    pub fn display_config(&self) -> String {
        // Mask the API key for security
        let masked_api_key = if self.api_key.len() > 8 {
            format!(
                "{}***{}",
                &self.api_key[..4],
                &self.api_key[self.api_key.len() - 4..]
            )
        } else {
            "***".to_string()
        };

        format!(
            "GeminiConfig {{ api_key: {}, model: {}, timeout: {}s, max_image_size: {}MB, base_url: {} }}",
            masked_api_key, self.model, self.timeout_seconds, self.max_image_size_mb, self.base_url
        )
    }

    /// Validate configuration parameters
    pub fn validate(&self) -> Result<(), GeminiConfigError> {
        // Validate API key is not empty
        if self.api_key.trim().is_empty() {
            return Err(GeminiConfigError::Configuration(
                "api_key cannot be empty".to_string(),
            ));
        }

        // Validate model name is not empty
        if self.model.trim().is_empty() {
            return Err(GeminiConfigError::Configuration(
                "model name cannot be empty".to_string(),
            ));
        }

        // Validate timeout is reasonable (between 1 and 600 seconds)
        if self.timeout_seconds == 0 || self.timeout_seconds > 600 {
            return Err(GeminiConfigError::Configuration(
                "timeout_seconds must be between 1 and 600".to_string(),
            ));
        }

        // Validate max image size is reasonable (between 1 and 100MB)
        if self.max_image_size_mb == 0 || self.max_image_size_mb > 100 {
            return Err(GeminiConfigError::Configuration(
                "max_image_size_mb must be between 1 and 100".to_string(),
            ));
        }

        // Validate base URL format
        if !self.base_url.starts_with("https://") {
            return Err(GeminiConfigError::Configuration(
                "base_url must start with https://".to_string(),
            ));
        }

        Ok(())
    }

    /// Get timeout as Duration for use with HTTP client
    pub fn timeout_duration(&self) -> Duration {
        Duration::from_secs(self.timeout_seconds)
    }

    /// Get max image size in bytes
    pub fn max_image_size_bytes(&self) -> u64 {
        self.max_image_size_mb * 1024 * 1024
    }

    /// Get the full API URL for Gemini generation endpoint
    pub fn generation_url(&self) -> String {
        format!("{}/models/{}:generateContent", self.base_url, self.model)
    }
}

/// Custom error types for Gemini configuration operations
#[derive(Debug, thiserror::Error)]
pub enum GeminiConfigError {
    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Validation error: {0}")]
    Validation(String),
}
