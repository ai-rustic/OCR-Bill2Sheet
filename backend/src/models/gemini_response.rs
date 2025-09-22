use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use std::collections::HashMap;

/// Main response structure from Gemini API OCR processing
///
/// Contains all results from processing a batch of images, including
/// successful extractions, processing summary, and any errors encountered.
/// Maps directly to the OCR API contract response schema.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiOCRResponse {
    /// Results for each processed image
    pub results: Vec<BillExtractionResult>,
    /// Summary statistics for the entire processing request
    pub processing_summary: ProcessingSummary,
    /// Non-fatal errors encountered during processing
    #[serde(default)]
    pub errors: Vec<ProcessingError>,
}

/// Individual extraction result for a single image
///
/// Contains the structured bill data extracted from one image,
/// along with confidence scores and processing metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillExtractionResult {
    /// Index of the image in the original request (0-based)
    pub image_index: usize,
    /// Structured bill information extracted from the image
    pub extracted_data: BillData,
    /// Confidence scores for the extraction
    pub confidence_scores: ConfidenceScores,
    /// Processing time for this individual image in milliseconds
    pub processing_time_ms: u64,
}

/// Structured bill data extracted from an image
///
/// Maps directly to the existing Bill table schema to enable
/// seamless database insertion. All fields are optional as
/// OCR extraction may not find all information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillData {
    /// Form number (e.g., "Mẫu 01-GTKT")
    pub form_no: Option<String>,
    /// Invoice number
    pub invoice_no: Option<String>,
    /// Company tax identification code
    pub tax_code: Option<String>,
    /// Issuing company name
    pub company_name: Option<String>,
    /// Company address
    pub company_address: Option<String>,
    /// Customer/buyer name
    pub buyer_name: Option<String>,
    /// Customer address
    pub buyer_address: Option<String>,
    /// Invoice issue date
    pub issue_date: Option<NaiveDate>,
    /// Total amount (using Decimal for precise financial calculations)
    pub total_amount: Option<rust_decimal::Decimal>,
    /// Tax/VAT amount
    pub tax_amount: Option<rust_decimal::Decimal>,
    /// Tax rate percentage
    pub tax_rate: Option<rust_decimal::Decimal>,
    /// Payment method description
    pub payment_method: Option<String>,
    /// Additional notes or description
    pub notes: Option<String>,
}

/// Confidence scores for OCR extraction quality
///
/// Provides both overall confidence and per-field confidence scores
/// to help assess the reliability of extracted data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceScores {
    /// Overall extraction confidence (0.0-1.0)
    pub overall: f32,
    /// Per-field confidence scores mapped by field name
    #[serde(default)]
    pub field_scores: HashMap<String, f32>,
}

/// Summary statistics for the entire processing request
///
/// Provides aggregate information about the processing session,
/// including success rates and timing information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingSummary {
    /// Total number of images submitted for processing
    pub total_images: u32,
    /// Number of successfully processed images
    pub successful_extractions: u32,
    /// Number of failed image extractions
    pub failed_extractions: u32,
    /// Total processing time for all images in milliseconds
    pub total_processing_time_ms: u64,
    /// Average confidence across all successful extractions
    pub average_confidence: Option<f32>,
}

/// Detailed error information for processing failures
///
/// Provides specific error details including the type of error,
/// which image failed (if applicable), and retry information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingError {
    /// Category of error encountered
    pub error_type: ErrorType,
    /// Human-readable error description
    pub message: String,
    /// Index of image that caused error (if applicable)
    pub image_index: Option<usize>,
    /// Suggested retry delay for rate limit errors
    pub retry_after_seconds: Option<u64>,
}

/// Categories of errors that can occur during processing
///
/// Maps to the error types defined in the API contract,
/// enabling proper error handling and user feedback.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

impl Default for ConfidenceScores {
    fn default() -> Self {
        Self {
            overall: 0.0,
            field_scores: HashMap::new(),
        }
    }
}

impl Default for BillData {
    fn default() -> Self {
        Self {
            form_no: None,
            invoice_no: None,
            tax_code: None,
            company_name: None,
            company_address: None,
            buyer_name: None,
            buyer_address: None,
            issue_date: None,
            total_amount: None,
            tax_amount: None,
            tax_rate: None,
            payment_method: None,
            notes: None,
        }
    }
}

impl GeminiOCRResponse {
    /// Creates a new empty response
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            processing_summary: ProcessingSummary::new(),
            errors: Vec::new(),
        }
    }

    /// Creates a response with initial capacity for results
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            results: Vec::with_capacity(capacity),
            processing_summary: ProcessingSummary::new(),
            errors: Vec::new(),
        }
    }

    /// Adds a successful extraction result
    pub fn add_result(&mut self, result: BillExtractionResult) {
        self.processing_summary.total_images += 1;
        self.processing_summary.successful_extractions += 1;
        self.processing_summary.total_processing_time_ms += result.processing_time_ms;

        self.results.push(result);

        // Update average confidence
        self.update_average_confidence();
    }

    /// Adds an error to the response
    pub fn add_error(&mut self, error: ProcessingError) {
        if error.image_index.is_some() {
            self.processing_summary.failed_extractions += 1;
        }
        self.errors.push(error);
    }

    /// Updates the average confidence score
    fn update_average_confidence(&mut self) {
        if self.results.is_empty() {
            self.processing_summary.average_confidence = None;
            return;
        }

        let sum: f32 = self.results.iter()
            .map(|r| r.confidence_scores.overall)
            .sum();

        self.processing_summary.average_confidence = Some(sum / self.results.len() as f32);
    }
}

impl BillExtractionResult {
    /// Creates a new extraction result
    pub fn new(
        image_index: usize,
        extracted_data: BillData,
        confidence_scores: ConfidenceScores,
        processing_time_ms: u64,
    ) -> Self {
        Self {
            image_index,
            extracted_data,
            confidence_scores,
            processing_time_ms,
        }
    }
}

impl ProcessingSummary {
    /// Creates a new empty processing summary
    pub fn new() -> Self {
        Self {
            total_images: 0,
            successful_extractions: 0,
            failed_extractions: 0,
            total_processing_time_ms: 0,
            average_confidence: None,
        }
    }
}

impl ProcessingError {
    /// Creates a new processing error
    pub fn new(
        error_type: ErrorType,
        message: String,
        image_index: Option<usize>,
    ) -> Self {
        Self {
            error_type,
            message,
            image_index,
            retry_after_seconds: None,
        }
    }

    /// Creates a rate limit error with retry information
    pub fn rate_limit_error(
        message: String,
        retry_after_seconds: u64,
        image_index: Option<usize>,
    ) -> Self {
        Self {
            error_type: ErrorType::ApiError,
            message,
            image_index,
            retry_after_seconds: Some(retry_after_seconds),
        }
    }
}

impl ConfidenceScores {
    /// Creates new confidence scores with overall score
    pub fn new(overall: f32) -> Self {
        Self {
            overall: overall.clamp(0.0, 1.0),
            field_scores: HashMap::new(),
        }
    }

    /// Adds a field confidence score
    pub fn add_field_score(&mut self, field_name: String, score: f32) {
        self.field_scores.insert(field_name, score.clamp(0.0, 1.0));
    }

    /// Calculates overall score from field scores if not set
    pub fn calculate_overall(&mut self) {
        if self.field_scores.is_empty() {
            return;
        }

        let sum: f32 = self.field_scores.values().sum();
        self.overall = sum / self.field_scores.len() as f32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gemini_ocr_response_creation() {
        let response = GeminiOCRResponse::new();
        assert_eq!(response.results.len(), 0);
        assert_eq!(response.processing_summary.total_images, 0);
        assert_eq!(response.errors.len(), 0);
    }

    #[test]
    fn test_add_result_updates_summary() {
        let mut response = GeminiOCRResponse::new();

        let bill_data = BillData::default();
        let confidence = ConfidenceScores::new(0.8);
        let result = BillExtractionResult::new(0, bill_data, confidence, 1500);

        response.add_result(result);

        assert_eq!(response.processing_summary.total_images, 1);
        assert_eq!(response.processing_summary.successful_extractions, 1);
        assert_eq!(response.processing_summary.total_processing_time_ms, 1500);
        assert_eq!(response.processing_summary.average_confidence, Some(0.8));
    }

    #[test]
    fn test_confidence_scores_validation() {
        let mut scores = ConfidenceScores::new(1.5); // Should be clamped to 1.0
        assert_eq!(scores.overall, 1.0);

        scores.add_field_score("invoice_no".to_string(), -0.5); // Should be clamped to 0.0
        assert_eq!(scores.field_scores.get("invoice_no"), Some(&0.0));
    }

    #[test]
    fn test_processing_error_creation() {
        let error = ProcessingError::new(
            ErrorType::ValidationError,
            "Invalid image format".to_string(),
            Some(0),
        );

        assert!(matches!(error.error_type, ErrorType::ValidationError));
        assert_eq!(error.message, "Invalid image format");
        assert_eq!(error.image_index, Some(0));
        assert_eq!(error.retry_after_seconds, None);
    }

    #[test]
    fn test_bill_data_default() {
        let bill_data = BillData::default();
        assert_eq!(bill_data.form_no, None);
        assert_eq!(bill_data.invoice_no, None);
        assert_eq!(bill_data.total_amount, None);
    }

    #[test]
    fn test_calculate_overall_confidence() {
        let mut scores = ConfidenceScores::new(0.0);
        scores.add_field_score("invoice_no".to_string(), 0.9);
        scores.add_field_score("total_amount".to_string(), 0.7);
        scores.add_field_score("company_name".to_string(), 0.8);

        scores.calculate_overall();

        // Should be (0.9 + 0.7 + 0.8) / 3 = 0.8
        assert!((scores.overall - 0.8).abs() < f32::EPSILON);
    }
}