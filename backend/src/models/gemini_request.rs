use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};

/// Main request structure for Gemini API OCR processing
///
/// Represents a complete request to process one or more images through the Gemini API.
/// Images are processed sequentially to respect API rate limits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiOCRRequest {
    /// Vector of images to be processed
    pub images: Vec<ImageData>,
    /// Processing configuration options
    pub options: ProcessingOptions,
}

/// Structure for image information and base64 data
///
/// Contains raw image data and metadata needed for Gemini API processing.
/// Supports common image formats with size validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageData {
    /// Raw image bytes
    pub content: Vec<u8>,
    /// MIME type of the image ("image/jpeg", "image/png")
    pub mime_type: String,
    /// Original filename for logging purposes
    pub filename: Option<String>,
    /// File size in bytes for validation
    pub size_bytes: usize,
}

/// Configuration options for processing requests
///
/// Controls how images are processed through the Gemini API,
/// including timeout, language hints, and confidence thresholds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingOptions {
    /// Per-image timeout in seconds (10-120 seconds range)
    pub timeout_seconds: u64,
    /// Language hint for OCR processing ("vi" for Vietnamese)
    pub language_hint: String,
    /// Minimum confidence threshold for field extraction (0.0-1.0)
    pub confidence_threshold: f32,
}

impl Default for ProcessingOptions {
    fn default() -> Self {
        Self {
            timeout_seconds: 60,
            language_hint: "vi".to_string(),
            confidence_threshold: 0.5,
        }
    }
}

impl GeminiOCRRequest {
    /// Creates a new Gemini OCR request with default processing options
    pub fn new(images: Vec<ImageData>) -> Self {
        Self {
            images,
            options: ProcessingOptions::default(),
        }
    }

    /// Creates a new request with custom processing options
    pub fn with_options(images: Vec<ImageData>, options: ProcessingOptions) -> Self {
        Self { images, options }
    }

    /// Validates the request according to API constraints
    pub fn validate(&self) -> Result<(), String> {
        // Check image count limit
        if self.images.is_empty() {
            return Err("At least one image is required".to_string());
        }

        if self.images.len() > 10 {
            return Err("Maximum 10 images per request allowed".to_string());
        }

        // Validate each image
        for (index, image) in self.images.iter().enumerate() {
            if let Err(e) = image.validate() {
                return Err(format!("Image {} validation failed: {}", index, e));
            }
        }

        // Validate processing options
        self.options.validate()
    }
}

impl ImageData {
    /// Creates new image data from bytes and MIME type
    pub fn new(content: Vec<u8>, mime_type: String, filename: Option<String>) -> Self {
        let size_bytes = content.len();
        Self {
            content,
            mime_type,
            filename,
            size_bytes,
        }
    }

    /// Validates image data according to API constraints
    pub fn validate(&self) -> Result<(), String> {
        // Check MIME type
        match self.mime_type.as_str() {
            "image/jpeg" | "image/png" => {},
            _ => return Err(format!("Unsupported MIME type: {}", self.mime_type)),
        }

        // Check size limit (20MB)
        const MAX_SIZE_BYTES: usize = 20 * 1024 * 1024; // 20MB
        if self.size_bytes > MAX_SIZE_BYTES {
            return Err(format!(
                "Image size {} bytes exceeds maximum {} bytes",
                self.size_bytes, MAX_SIZE_BYTES
            ));
        }

        // Check minimum size
        if self.content.is_empty() {
            return Err("Image content cannot be empty".to_string());
        }

        Ok(())
    }

    /// Returns the file extension based on MIME type
    pub fn file_extension(&self) -> &str {
        match self.mime_type.as_str() {
            "image/jpeg" => "jpg",
            "image/png" => "png",
            _ => "bin",
        }
    }

    /// Converts image to base64 string for API requests
    pub fn to_base64(&self) -> String {
        general_purpose::STANDARD.encode(&self.content)
    }
}

impl ProcessingOptions {
    /// Creates new processing options with validation
    pub fn new(
        timeout_seconds: u64,
        language_hint: String,
        confidence_threshold: f32,
    ) -> Result<Self, String> {
        let options = Self {
            timeout_seconds,
            language_hint,
            confidence_threshold,
        };
        options.validate()?;
        Ok(options)
    }

    /// Validates processing options
    pub fn validate(&self) -> Result<(), String> {
        // Validate timeout range
        if self.timeout_seconds < 10 || self.timeout_seconds > 120 {
            return Err("Timeout must be between 10 and 120 seconds".to_string());
        }

        // Validate confidence threshold
        if self.confidence_threshold < 0.0 || self.confidence_threshold > 1.0 {
            return Err("Confidence threshold must be between 0.0 and 1.0".to_string());
        }

        // Validate language hint
        if self.language_hint.is_empty() {
            return Err("Language hint cannot be empty".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processing_options_default() {
        let options = ProcessingOptions::default();
        assert_eq!(options.timeout_seconds, 60);
        assert_eq!(options.language_hint, "vi");
        assert_eq!(options.confidence_threshold, 0.5);
        assert!(options.validate().is_ok());
    }

    #[test]
    fn test_image_data_validation() {
        let valid_image = ImageData::new(
            vec![1, 2, 3, 4], // Some dummy content
            "image/jpeg".to_string(),
            Some("test.jpg".to_string()),
        );
        assert!(valid_image.validate().is_ok());

        let invalid_mime = ImageData::new(
            vec![1, 2, 3, 4],
            "image/gif".to_string(),
            None,
        );
        assert!(invalid_mime.validate().is_err());
    }

    #[test]
    fn test_request_validation() {
        let image = ImageData::new(
            vec![1, 2, 3, 4],
            "image/jpeg".to_string(),
            Some("test.jpg".to_string()),
        );

        let request = GeminiOCRRequest::new(vec![image]);
        assert!(request.validate().is_ok());

        let empty_request = GeminiOCRRequest::new(vec![]);
        assert!(empty_request.validate().is_err());
    }
}