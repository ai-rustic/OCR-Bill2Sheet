use std::time::Duration;
use reqwest::Client;
use serde_json::Value;
use tracing::{info, warn, error, debug};

use crate::config::GeminiConfig;
use crate::models::{
    GeminiOCRRequest, GeminiOCRResponse, BillExtractionResult, BillData, ConfidenceScores,
    OcrProcessingError,
};

/// Service for handling Gemini AI API interactions for OCR processing
///
/// This service manages all communication with Google's Gemini API for
/// processing Vietnamese bill images. It handles request preparation,
/// API calls, response parsing, and error handling.
#[derive(Debug)]
pub struct GeminiService {
    /// HTTP client configured for Gemini API communication
    client: Client,
    /// Configuration settings for the Gemini service
    config: GeminiConfig,
}

impl GeminiService {
    /// Creates a new GeminiService with the provided configuration
    ///
    /// Sets up the HTTP client with appropriate timeouts, headers, and
    /// connection pooling for optimal performance with the Gemini API.
    ///
    /// # Arguments
    /// * `config` - GeminiConfig containing API key, model, and other settings
    ///
    /// # Returns
    /// * `Result<Self, OcrProcessingError>` - Configured service or initialization error
    pub fn new(config: GeminiConfig) -> Result<Self, OcrProcessingError> {
        // Validate configuration before creating the service
        config.validate().map_err(|e| {
            OcrProcessingError::configuration(format!(
                "GeminiService initialization failed: {}",
                e
            ))
        })?;

        // Build HTTP client with appropriate configuration
        let client = Client::builder()
            .timeout(config.timeout_duration())
            .connect_timeout(Duration::from_secs(10))
            .pool_max_idle_per_host(5)
            .pool_idle_timeout(Duration::from_secs(30))
            .user_agent("Bill2Sheet-Backend/1.0")
            .build()
            .map_err(|e| {
                OcrProcessingError::configuration(format!(
                    "Failed to create HTTP client: {}",
                    e
                ))
            })?;

        info!(
            "GeminiService initialized successfully with config: {}",
            config.display_config()
        );

        Ok(Self { client, config })
    }

    /// Creates a GeminiService from environment variables
    ///
    /// Convenience method that loads configuration from environment variables
    /// and creates a new service instance.
    ///
    /// # Returns
    /// * `Result<Self, OcrProcessingError>` - Configured service or configuration error
    pub fn from_env() -> Result<Self, OcrProcessingError> {
        let config = GeminiConfig::from_env().map_err(|e| {
            OcrProcessingError::configuration(format!(
                "Failed to load GeminiConfig from environment: {}",
                e
            ))
        })?;

        Self::new(config)
    }

    /// Prepares a Gemini API request from OCR request data
    ///
    /// Converts the internal GeminiOCRRequest format to the JSON structure
    /// expected by the Gemini API. Handles image encoding, prompt construction,
    /// and request formatting according to Gemini API specifications.
    ///
    /// # Arguments
    /// * `request` - The OCR request containing images and processing options
    ///
    /// # Returns
    /// * `Result<Value, OcrProcessingError>` - JSON request body or preparation error
    pub async fn prepare_gemini_request(
        &self,
        request: &GeminiOCRRequest,
    ) -> Result<Value, OcrProcessingError> {
        debug!("Preparing Gemini API request for {} images", request.images.len());

        // Validate the request first
        request.validate().map_err(|e| {
            OcrProcessingError::validation(
                format!("Request validation failed: {}", e),
                None,
                None,
            )
        })?;

        // Create the Vietnamese bill extraction prompt
        let text_prompt = self.create_vietnamese_bill_prompt(request);

        // Create image parts for the API request
        let mut parts = Vec::new();

        // Add text prompt part
        parts.push(serde_json::json!({
            "text": text_prompt
        }));

        // Add image parts
        for (index, image) in request.images.iter().enumerate() {
            // Validate image
            image.validate().map_err(|e| {
                OcrProcessingError::validation(
                    format!("Image {} validation failed: {}", index, e),
                    None,
                    Some(index),
                )
            })?;

            // Create image part with base64 data
            let image_part = serde_json::json!({
                "inline_data": {
                    "mime_type": image.mime_type,
                    "data": image.to_base64()
                }
            });
            parts.push(image_part);
        }

        // Create the bill data JSON schema for structured output
        let response_schema = self.create_bill_data_schema();

        // Construct the full Gemini API request
        let api_request = serde_json::json!({
            "contents": [{
                "role": "user",
                "parts": parts
            }],
            "generationConfig": {
                "responseMimeType": "application/json",
                "responseSchema": response_schema,
                "temperature": 0.1, // Low temperature for consistent extraction
                "topK": 1,
                "topP": 0.8,
                "maxOutputTokens": 2048
            }
        });

        info!(
            "Prepared Gemini API request with {} images and Vietnamese OCR prompt",
            request.images.len()
        );

        Ok(api_request)
    }

    /// Makes the actual HTTP call to the Gemini API
    ///
    /// Handles the low-level HTTP communication with the Gemini API, including
    /// authentication, request headers, timeout handling, and response validation.
    /// Provides comprehensive error handling for various API failure scenarios.
    ///
    /// # Arguments
    /// * `request_body` - JSON request body prepared by prepare_gemini_request
    ///
    /// # Returns
    /// * `Result<Value, OcrProcessingError>` - Raw API response or HTTP error
    pub async fn call_gemini_api(
        &self,
        request_body: Value,
    ) -> Result<Value, OcrProcessingError> {
        let start_time = std::time::Instant::now();
        let api_url = self.config.generation_url();

        debug!("Making Gemini API call to: {}", api_url);
        debug!("Request timeout: {:?}", self.config.timeout_duration());

        // Make the HTTP request to Gemini API
        let response = self
            .client
            .post(&api_url)
            .header("x-goog-api-key", &self.config.api_key)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| {
                let elapsed = start_time.elapsed();
                error!("Gemini API request failed after {:?}: {}", elapsed, e);

                if e.is_timeout() {
                    OcrProcessingError::timeout(
                        format!("Gemini API request timed out after {:?}: {}", elapsed, e),
                        self.config.timeout_seconds,
                        None,
                    )
                } else if e.is_connect() {
                    OcrProcessingError::network(
                        format!("Failed to connect to Gemini API: {}", e),
                        None,
                    )
                } else {
                    OcrProcessingError::network(
                        format!("Network error during Gemini API call: {}", e),
                        None,
                    )
                }
            })?;

        let status = response.status();
        let elapsed = start_time.elapsed();

        debug!("Gemini API response received after {:?} with status: {}", elapsed, status);

        // Handle different HTTP status codes
        if status.is_success() {
            // Parse the successful response as JSON
            let response_body = response
                .text()
                .await
                .map_err(|e| {
                    error!("Failed to read Gemini API response body: {}", e);
                    OcrProcessingError::network(
                        format!("Failed to read API response: {}", e),
                        None,
                    )
                })?;

            debug!("Raw Gemini API response length: {} bytes", response_body.len());

            // Parse JSON response
            let json_response: Value = serde_json::from_str(&response_body)
                .map_err(|e| {
                    error!("Failed to parse Gemini API JSON response: {}", e);
                    warn!("Response body preview: {}",
                        if response_body.len() > 200 {
                            format!("{}...", &response_body[..200])
                        } else {
                            response_body.clone()
                        }
                    );
                    OcrProcessingError::parse(
                        format!("Invalid JSON response from Gemini API: {}", e),
                        Some(response_body),
                        None,
                    )
                })?;

            info!("Gemini API call completed successfully in {:?}", elapsed);
            Ok(json_response)
        } else {
            // Handle error responses
            let response_body = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error response".to_string());

            warn!("Gemini API returned error status {}: {}", status, response_body);

            // Handle specific error status codes
            match status.as_u16() {
                400 => {
                    // Bad Request - usually validation errors
                    Err(OcrProcessingError::validation(
                        format!("Invalid request to Gemini API: {}", response_body),
                        None,
                        None,
                    ))
                }
                401 => {
                    // Unauthorized - API key issues
                    Err(OcrProcessingError::configuration(
                        "Invalid API key - check GEMINI_API_KEY environment variable"
                    ))
                }
                403 => {
                    // Forbidden - quota or permission issues
                    Err(OcrProcessingError::api(
                        format!("Access forbidden to Gemini API: {}", response_body),
                        Some(403),
                        None,
                        None,
                    ))
                }
                404 => {
                    // Not Found - wrong endpoint or model
                    Err(OcrProcessingError::configuration(
                        format!("Gemini API endpoint not found. Check model name: {}", self.config.model)
                    ))
                }
                429 => {
                    // Rate Limited - extract retry-after if available
                    let retry_after = self.extract_retry_after_seconds(&response_body);
                    Err(OcrProcessingError::rate_limit(
                        format!("Gemini API rate limit exceeded: {}", response_body),
                        retry_after.unwrap_or(60), // Default to 60 seconds if no header
                        None,
                    ))
                }
                500..=599 => {
                    // Server errors - Gemini service issues
                    Err(OcrProcessingError::service_unavailable(
                        format!("Gemini API server error ({}): {}", status, response_body),
                        Some("server_error".to_string()),
                        None, // We don't have recovery time info
                    ))
                }
                _ => {
                    // Other unexpected status codes
                    Err(OcrProcessingError::api(
                        format!("Unexpected Gemini API response ({}): {}", status, response_body),
                        Some(status.as_u16()),
                        None,
                        None,
                    ))
                }
            }
        }
    }

    /// Parses the Gemini API response into structured bill data
    ///
    /// Converts the raw JSON response from the Gemini API into structured
    /// BillData objects. Handles text parsing, field extraction, confidence
    /// calculation, and data validation for Vietnamese bill formats.
    ///
    /// # Arguments
    /// * `api_response` - Raw JSON response from the Gemini API
    /// * `image_count` - Number of images that were processed
    ///
    /// # Returns
    /// * `Result<GeminiOCRResponse, OcrProcessingError>` - Structured response or parsing error
    pub async fn parse_gemini_response(
        &self,
        api_response: Value,
        image_count: usize,
    ) -> Result<GeminiOCRResponse, OcrProcessingError> {
        let start_time = std::time::Instant::now();
        debug!("Parsing Gemini API response for {} images", image_count);

        // Initialize response structure
        let mut response = GeminiOCRResponse::with_capacity(image_count);

        // Extract candidates from Gemini API response
        let candidates = api_response
            .get("candidates")
            .and_then(|c| c.as_array())
            .ok_or_else(|| {
                error!("Gemini API response missing 'candidates' array");
                OcrProcessingError::parse(
                    "Invalid Gemini API response: missing 'candidates' field",
                    Some(api_response.to_string()),
                    None,
                )
            })?;

        debug!("Found {} candidates in Gemini response", candidates.len());

        if candidates.is_empty() {
            warn!("Gemini API returned no candidates");
            return Ok(response);
        }

        // Process the first candidate (Gemini typically returns one candidate for JSON mode)
        let candidate = &candidates[0];

        // Check if candidate was blocked for safety
        if let Some(finish_reason) = candidate.get("finishReason").and_then(|f| f.as_str()) {
            if finish_reason != "STOP" {
                warn!("Gemini candidate blocked with reason: {}", finish_reason);
                response.add_error(crate::models::ProcessingError::new(
                    crate::models::ErrorType::ApiError,
                    format!("Gemini blocked response due to: {}", finish_reason),
                    None,
                ));
                return Ok(response);
            }
        }

        // Extract the content from the candidate
        let content = candidate
            .get("content")
            .and_then(|c| c.get("parts"))
            .and_then(|p| p.as_array())
            .and_then(|parts| parts.first())
            .and_then(|part| part.get("text"))
            .and_then(|t| t.as_str())
            .ok_or_else(|| {
                error!("Could not extract text content from Gemini candidate");
                OcrProcessingError::parse(
                    "Gemini response missing expected content structure",
                    Some(candidate.to_string()),
                    None,
                )
            })?;

        debug!("Extracted text content from Gemini response, length: {}", content.len());

        // Parse the JSON content from Gemini
        let extracted_json: Value = serde_json::from_str(content)
            .map_err(|e| {
                error!("Failed to parse Gemini JSON content: {}", e);
                warn!("Content preview: {}",
                    if content.len() > 200 {
                        format!("{}...", &content[..200])
                    } else {
                        content.to_string()
                    }
                );
                OcrProcessingError::parse(
                    format!("Invalid JSON from Gemini API: {}", e),
                    Some(content.to_string()),
                    None,
                )
            })?;

        // Extract bill data from the parsed JSON
        let bill_data = self.extract_bill_data(&extracted_json)?;

        // Calculate confidence scores for the extraction
        let confidence_scores = self.calculate_confidence_scores(&extracted_json, &bill_data);

        // Validate the extracted data
        let validation_result = self.validate_extracted_data(&bill_data);
        if let Err(validation_errors) = validation_result {
            warn!("Validation errors in extracted data: {:?}", validation_errors);
            // Add validation errors as warnings but continue processing
            for error in validation_errors {
                response.add_error(crate::models::ProcessingError::new(
                    crate::models::ErrorType::ValidationError,
                    format!("Data validation warning: {}", error),
                    Some(0), // Single image processing
                ));
            }
        }

        let processing_time = start_time.elapsed().as_millis() as u64;

        // Create the extraction result
        let extraction_result = BillExtractionResult::new(
            0, // Single image index
            bill_data,
            confidence_scores,
            processing_time,
        );

        response.add_result(extraction_result);

        info!("Successfully parsed Gemini response in {:?}", start_time.elapsed());
        Ok(response)
    }

    /// Extracts structured bill data from the parsed JSON content
    ///
    /// Maps JSON fields to BillData structure, handling type conversions
    /// and Vietnamese text encoding. All fields are optional to handle
    /// partial extraction results gracefully.
    ///
    /// # Arguments
    /// * `json_data` - Parsed JSON data from Gemini response
    ///
    /// # Returns
    /// * `Result<BillData, OcrProcessingError>` - Extracted bill data or conversion error
    fn extract_bill_data(&self, json_data: &Value) -> Result<BillData, OcrProcessingError> {
        debug!("Extracting bill data from JSON response");

        let mut bill_data = BillData::default();

        // Extract string fields
        if let Some(form_no) = json_data.get("form_no").and_then(|v| v.as_str()) {
            bill_data.form_no = Some(form_no.trim().to_string());
        }

        if let Some(invoice_no) = json_data.get("invoice_no").and_then(|v| v.as_str()) {
            bill_data.invoice_no = Some(invoice_no.trim().to_string());
        }

        if let Some(seller_name) = json_data.get("seller_name").and_then(|v| v.as_str()) {
            bill_data.company_name = Some(seller_name.trim().to_string());
        }

        if let Some(seller_tax_code) = json_data.get("seller_tax_code").and_then(|v| v.as_str()) {
            bill_data.tax_code = Some(seller_tax_code.trim().to_string());
        }

        if let Some(item_name) = json_data.get("item_name").and_then(|v| v.as_str()) {
            bill_data.notes = Some(item_name.trim().to_string()); // Store item name in notes
        }

        // Extract and parse date field
        if let Some(issued_date_str) = json_data.get("issued_date").and_then(|v| v.as_str()) {
            match chrono::NaiveDate::parse_from_str(issued_date_str.trim(), "%Y-%m-%d") {
                Ok(date) => {
                    bill_data.issue_date = Some(date);
                }
                Err(e) => {
                    warn!("Failed to parse issued_date '{}': {}", issued_date_str, e);
                    // Try alternative date formats commonly found in Vietnamese bills
                    if let Ok(date) = chrono::NaiveDate::parse_from_str(issued_date_str.trim(), "%d/%m/%Y") {
                        bill_data.issue_date = Some(date);
                    } else if let Ok(date) = chrono::NaiveDate::parse_from_str(issued_date_str.trim(), "%d-%m-%Y") {
                        bill_data.issue_date = Some(date);
                    } else {
                        debug!("Could not parse date '{}' in any supported format", issued_date_str);
                    }
                }
            }
        }

        // Extract and parse decimal fields
        bill_data.total_amount = self.extract_decimal_field(json_data, "total_amount");
        bill_data.tax_rate = self.extract_decimal_field(json_data, "vat_rate");
        bill_data.tax_amount = self.extract_decimal_field(json_data, "vat_amount");

        debug!("Successfully extracted bill data with {} populated fields",
            self.count_populated_fields(&bill_data));

        Ok(bill_data)
    }

    /// Extracts a decimal field from JSON data with error handling
    ///
    /// Attempts to parse decimal values from various JSON types (number, string)
    /// and handles Vietnamese number formatting (comma as decimal separator).
    ///
    /// # Arguments
    /// * `json_data` - JSON object to extract from
    /// * `field_name` - Name of the field to extract
    ///
    /// # Returns
    /// * `Option<rust_decimal::Decimal>` - Parsed decimal value or None if not found/invalid
    fn extract_decimal_field(&self, json_data: &Value, field_name: &str) -> Option<rust_decimal::Decimal> {
        use std::str::FromStr;

        if let Some(value) = json_data.get(field_name) {
            // Handle numeric values directly
            if let Some(num) = value.as_f64() {
                if let Ok(decimal) = rust_decimal::Decimal::try_from(num) {
                    return Some(decimal);
                }
            }

            // Handle string values
            if let Some(str_value) = value.as_str() {
                let mut cleaned = str_value
                    .trim()
                    .replace(" ", "")   // Remove spaces
                    .replace("₫", "")   // Remove Vietnamese dong symbol
                    .replace("VND", "") // Remove currency code
                    .trim()
                    .to_string();

                // Handle Vietnamese number formatting (thousands separator + decimal comma)
                // Examples: "1.234,56" -> "1234.56", "1,234.56" -> "1234.56"
                if cleaned.contains(',') && cleaned.contains('.') {
                    // Format like "1.234,56" - dots are thousands separators, comma is decimal
                    let parts: Vec<&str> = cleaned.split(',').collect();
                    if parts.len() == 2 {
                        let integer_part = parts[0].replace(".", ""); // Remove thousands separators
                        cleaned = format!("{}.{}", integer_part, parts[1]);
                    }
                } else if cleaned.contains(',') && !cleaned.contains('.') {
                    // Format like "1234,56" - comma is decimal separator
                    cleaned = cleaned.replace(",", ".");
                }
                // If only dots, assume it's standard format "1234.56"

                if !cleaned.is_empty() {
                    if let Ok(decimal) = rust_decimal::Decimal::from_str(&cleaned) {
                        return Some(decimal);
                    } else {
                        debug!("Failed to parse decimal field '{}' value '{}' -> '{}'", field_name, str_value, cleaned);
                    }
                }
            }
        }

        None
    }

    /// Calculates confidence scores for the extracted data
    ///
    /// Evaluates the quality and reliability of each extracted field
    /// based on data completeness, format validation, and consistency.
    ///
    /// # Arguments
    /// * `json_data` - Original JSON response from Gemini
    /// * `bill_data` - Extracted and validated bill data
    ///
    /// # Returns
    /// * `ConfidenceScores` - Overall and per-field confidence scores
    fn calculate_confidence_scores(&self, json_data: &Value, bill_data: &BillData) -> ConfidenceScores {
        let mut confidence = ConfidenceScores::default();

        // Define weights for different field types
        const CRITICAL_FIELDS: &[&str] = &["invoice_no", "company_name", "total_amount"];
        const IMPORTANT_FIELDS: &[&str] = &["issue_date", "tax_code", "tax_amount"];
        const OPTIONAL_FIELDS: &[&str] = &["form_no", "tax_rate", "notes", "buyer_name", "company_address"];

        let mut total_weight = 0.0;
        let mut weighted_score = 0.0;

        // Score critical fields (weight: 3.0)
        for field in CRITICAL_FIELDS {
            let score = self.calculate_field_confidence(json_data, bill_data, field);
            confidence.add_field_score(field.to_string(), score);
            total_weight += 3.0;
            weighted_score += score * 3.0;
        }

        // Score important fields (weight: 2.0)
        for field in IMPORTANT_FIELDS {
            let score = self.calculate_field_confidence(json_data, bill_data, field);
            confidence.add_field_score(field.to_string(), score);
            total_weight += 2.0;
            weighted_score += score * 2.0;
        }

        // Score optional fields (weight: 1.0)
        for field in OPTIONAL_FIELDS {
            let score = self.calculate_field_confidence(json_data, bill_data, field);
            confidence.add_field_score(field.to_string(), score);
            total_weight += 1.0;
            weighted_score += score * 1.0;
        }

        // Calculate overall confidence as weighted average
        confidence.overall = if total_weight > 0.0 {
            (weighted_score / total_weight).clamp(0.0, 1.0)
        } else {
            0.0
        };

        debug!("Calculated overall confidence: {:.2}", confidence.overall);
        confidence
    }

    /// Calculates confidence score for a specific field
    ///
    /// Evaluates field quality based on presence, format validity,
    /// and data consistency for Vietnamese bill formats.
    ///
    /// # Arguments
    /// * `json_data` - Original JSON response
    /// * `bill_data` - Extracted bill data
    /// * `field_name` - Name of the field to evaluate
    ///
    /// # Returns
    /// * `f32` - Confidence score between 0.0 and 1.0
    fn calculate_field_confidence(&self, json_data: &Value, bill_data: &BillData, field_name: &str) -> f32 {
        // Check if field is present in JSON
        let json_present = json_data.get(field_name).is_some();
        if !json_present {
            return 0.0;
        }

        // Check if field was successfully extracted
        let extracted = match field_name {
            "form_no" => bill_data.form_no.is_some(),
            "invoice_no" => bill_data.invoice_no.is_some(),
            "issue_date" => bill_data.issue_date.is_some(),
            "company_name" => bill_data.company_name.is_some(),
            "tax_code" => bill_data.tax_code.is_some(),
            "notes" => bill_data.notes.is_some(),
            "buyer_name" => bill_data.buyer_name.is_some(),
            "company_address" => bill_data.company_address.is_some(),
            "total_amount" => bill_data.total_amount.is_some(),
            "tax_rate" => bill_data.tax_rate.is_some(),
            "tax_amount" => bill_data.tax_amount.is_some(),
            "payment_method" => bill_data.payment_method.is_some(),
            _ => false,
        };

        if !extracted {
            return 0.3; // Low confidence if present in JSON but failed to extract
        }

        // Basic confidence for successful extraction
        let mut confidence: f32 = 0.8;

        // Boost confidence for well-formatted data
        match field_name {
            "invoice_no" => {
                if let Some(ref invoice) = bill_data.invoice_no {
                    // Vietnamese invoices typically have specific patterns
                    if invoice.len() >= 3 && invoice.chars().any(|c| c.is_numeric()) {
                        confidence = 0.9;
                    }
                }
            }
            "issue_date" => {
                if bill_data.issue_date.is_some() {
                    confidence = 0.95; // High confidence if date was successfully parsed
                }
            }
            "tax_code" => {
                if let Some(ref tax_code) = bill_data.tax_code {
                    // Vietnamese tax codes are typically 10 digits
                    if tax_code.len() == 10 && tax_code.chars().all(|c| c.is_numeric()) {
                        confidence = 0.95;
                    }
                }
            }
            "total_amount" | "tax_amount" => {
                // High confidence for successfully parsed financial amounts
                confidence = 0.9;
            }
            "tax_rate" => {
                if let Some(rate) = bill_data.tax_rate {
                    // Common Vietnamese VAT rates
                    if rate == rust_decimal::Decimal::from(0) ||
                       rate == rust_decimal::Decimal::from(5) ||
                       rate == rust_decimal::Decimal::from(10) {
                        confidence = 0.95;
                    }
                }
            }
            _ => {}
        }

        confidence.clamp(0.0, 1.0)
    }

    /// Validates extracted bill data for consistency and accuracy
    ///
    /// Performs comprehensive validation including financial calculations,
    /// date ranges, and Vietnamese-specific business rules.
    ///
    /// # Arguments
    /// * `bill_data` - Extracted bill data to validate
    ///
    /// # Returns
    /// * `Result<(), Vec<String>>` - Success or list of validation errors
    fn validate_extracted_data(&self, bill_data: &BillData) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Validate date ranges
        if let Some(issue_date) = bill_data.issue_date {
            let today = chrono::Utc::now().naive_utc().date();

            if issue_date > today {
                errors.push("Issue date cannot be in the future".to_string());
            }

            let ten_years_ago = today - chrono::Duration::days(10 * 365);
            if issue_date < ten_years_ago {
                errors.push("Issue date is older than 10 years".to_string());
            }
        }

        // Validate financial amounts
        if let Some(total_amount) = bill_data.total_amount {
            if total_amount < rust_decimal::Decimal::ZERO {
                errors.push("Total amount cannot be negative".to_string());
            }
        }

        if let Some(tax_amount) = bill_data.tax_amount {
            if tax_amount < rust_decimal::Decimal::ZERO {
                errors.push("Tax amount cannot be negative".to_string());
            }
        }

        // Validate tax rate
        if let Some(tax_rate) = bill_data.tax_rate {
            if tax_rate < rust_decimal::Decimal::ZERO || tax_rate > rust_decimal::Decimal::from(100) {
                errors.push("Tax rate must be between 0 and 100 percent".to_string());
            }
        }

        // Validate financial consistency
        if let (Some(total), Some(rate), Some(tax)) =
            (bill_data.total_amount, bill_data.tax_rate, bill_data.tax_amount) {
            let expected_tax = total * rate / rust_decimal::Decimal::from(100);
            let tolerance = rust_decimal::Decimal::try_from(0.01).unwrap();

            if (expected_tax - tax).abs() > tolerance {
                errors.push(format!(
                    "Tax calculation inconsistency: {} * {}% = {}, but found {}",
                    total, rate, expected_tax, tax
                ));
            }
        }

        // Validate Vietnamese tax code format
        if let Some(ref tax_code) = bill_data.tax_code {
            if !tax_code.is_empty() && (tax_code.len() != 10 || !tax_code.chars().all(|c| c.is_numeric())) {
                errors.push("Vietnamese tax code should be 10 digits".to_string());
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Counts the number of populated fields in BillData
    ///
    /// Helper method for logging and confidence calculation.
    ///
    /// # Arguments
    /// * `bill_data` - Bill data to analyze
    ///
    /// # Returns
    /// * `usize` - Number of non-None fields
    fn count_populated_fields(&self, bill_data: &BillData) -> usize {
        let mut count = 0;

        if bill_data.form_no.is_some() { count += 1; }
        if bill_data.invoice_no.is_some() { count += 1; }
        if bill_data.tax_code.is_some() { count += 1; }
        if bill_data.company_name.is_some() { count += 1; }
        if bill_data.company_address.is_some() { count += 1; }
        if bill_data.buyer_name.is_some() { count += 1; }
        if bill_data.buyer_address.is_some() { count += 1; }
        if bill_data.issue_date.is_some() { count += 1; }
        if bill_data.total_amount.is_some() { count += 1; }
        if bill_data.tax_amount.is_some() { count += 1; }
        if bill_data.tax_rate.is_some() { count += 1; }
        if bill_data.payment_method.is_some() { count += 1; }
        if bill_data.notes.is_some() { count += 1; }

        count
    }

    /// Processes multiple images sequentially through the Gemini API
    ///
    /// Main entry point for OCR processing. Handles multiple images by processing
    /// them sequentially to respect API rate limits. Aggregates results and
    /// provides comprehensive error handling and progress tracking.
    ///
    /// # Arguments
    /// * `request` - Complete OCR request with images and processing options
    ///
    /// # Returns
    /// * `Result<GeminiOCRResponse, OcrProcessingError>` - Aggregated processing results
    ///
    /// # Features
    /// - Sequential processing to respect API rate limits
    /// - Graceful handling of individual image failures
    /// - Comprehensive processing statistics
    /// - Progress tracking and logging
    /// - Rate limit backoff and retry logic
    pub async fn process_images(
        &self,
        request: GeminiOCRRequest,
    ) -> Result<GeminiOCRResponse, OcrProcessingError> {
        let start_time = std::time::Instant::now();
        let total_images = request.images.len();

        info!("Starting sequential processing of {} images", total_images);

        // Validate the request first
        request.validate().map_err(|e| {
            error!("Request validation failed: {}", e);
            OcrProcessingError::validation(
                format!("Request validation failed: {}", e),
                None,
                None,
            )
        })?;

        // Initialize response with capacity for all images
        let mut response = GeminiOCRResponse::with_capacity(total_images);

        // Track processing statistics
        let mut successful_count = 0;
        let mut failed_count = 0;
        let mut total_processing_time = 0u64;

        // Process each image sequentially
        for (index, image) in request.images.iter().enumerate() {
            let image_start_time = std::time::Instant::now();

            info!("Processing image {} of {} ({})",
                index + 1,
                total_images,
                image.filename.as_deref().unwrap_or("unnamed")
            );

            // Create a single-image request for processing
            let single_image_request = GeminiOCRRequest {
                images: vec![image.clone()],
                options: request.options.clone(),
            };

            // Process the individual image through the pipeline
            match self.process_single_image(&single_image_request, index).await {
                Ok(mut extraction_result) => {
                    // Update the image index to reflect position in original request
                    extraction_result.image_index = index;

                    let processing_time = image_start_time.elapsed().as_millis() as u64;
                    extraction_result.processing_time_ms = processing_time;
                    total_processing_time += processing_time;

                    response.add_result(extraction_result);
                    successful_count += 1;

                    info!("Successfully processed image {} in {:?}",
                        index,
                        image_start_time.elapsed()
                    );
                }
                Err(error) => {
                    failed_count += 1;
                    let processing_time = image_start_time.elapsed().as_millis() as u64;
                    total_processing_time += processing_time;

                    error!("Failed to process image {}: {}", index, error);

                    // Convert processing error to response error format
                    let response_error = crate::models::ProcessingError::new(
                        self.map_processing_error_type(&error),
                        error.to_string(),
                        Some(index),
                    );

                    response.add_error(response_error);

                    // Handle rate limiting with backoff
                    if let Some(retry_after) = error.retry_after_seconds() {
                        warn!("Rate limited on image {}, backing off for {} seconds",
                            index, retry_after);

                        // Only sleep if there are more images to process
                        if index + 1 < total_images {
                            tokio::time::sleep(Duration::from_secs(retry_after)).await;
                            info!("Resuming processing after rate limit backoff");
                        }
                    }

                    // Continue processing remaining images despite this failure
                    // This ensures we extract as much data as possible from the batch
                }
            }

            // Add a small delay between images to be respectful of API limits
            // Skip delay for the last image
            if index + 1 < total_images {
                const INTER_IMAGE_DELAY_MS: u64 = 500; // 500ms between images
                tokio::time::sleep(Duration::from_millis(INTER_IMAGE_DELAY_MS)).await;
            }
        }

        // Update final processing statistics
        response.processing_summary.total_images = total_images as u32;
        response.processing_summary.total_processing_time_ms = total_processing_time;

        let total_time = start_time.elapsed();

        info!(
            "Completed processing {} images: {} successful, {} failed in {:?}",
            total_images,
            successful_count,
            failed_count,
            total_time
        );

        // Log detailed statistics
        if successful_count > 0 {
            let avg_time_per_image = total_processing_time / successful_count;
            let avg_confidence = response.processing_summary.average_confidence.unwrap_or(0.0);

            info!(
                "Processing stats: avg time per image: {}ms, avg confidence: {:.2}",
                avg_time_per_image,
                avg_confidence
            );
        }

        // Return results even if some images failed - partial success is still valuable
        Ok(response)
    }

    /// Creates a comprehensive Vietnamese bill extraction prompt
    ///
    /// Generates a detailed prompt that instructs Gemini to extract structured data
    /// from Vietnamese invoices/bills. The prompt includes field descriptions,
    /// format specifications, and examples to ensure accurate extraction.
    ///
    /// # Arguments
    /// * `request` - The OCR request containing processing options
    ///
    /// # Returns
    /// * `String` - The formatted prompt text
    fn create_vietnamese_bill_prompt(&self, request: &GeminiOCRRequest) -> String {
        format!(
            r#"You are an expert Vietnamese invoice/bill OCR processor. Analyze the provided images and extract structured data in JSON format.

IMPORTANT INSTRUCTIONS:
1. Extract text exactly as it appears in Vietnamese
2. For dates, use YYYY-MM-DD format (e.g., "2024-03-15")
3. For decimal numbers, use decimal notation (e.g., "1234.56")
4. If a field is not clearly visible or present, use null
5. Confidence threshold: {:.1}
6. Language context: Vietnamese ({})

EXTRACT THE FOLLOWING FIELDS:

📝 **Document Information:**
- form_no: Form template number (e.g., "Mẫu 01-GTKT", "Form 01-VAT")
- serial_no: Serial number of the invoice book/series
- invoice_no: Invoice number/identifier
- issued_date: Date when invoice was issued (YYYY-MM-DD format)

🏢 **Seller Information:**
- seller_name: Company/business name of the seller
- seller_tax_code: Tax identification code (Mã số thuế)

📦 **Item/Service Details:**
- item_name: Description of goods/services sold
- unit: Unit of measurement (e.g., "kg", "cái", "giờ", "m²")
- quantity: Quantity sold (decimal number)
- unit_price: Price per unit (decimal number)

💰 **Financial Information:**
- total_amount: Total amount before tax (decimal number)
- vat_rate: VAT/tax rate as percentage (e.g., 10.00 for 10%)
- vat_amount: VAT/tax amount (decimal number)

RESPONSE FORMAT:
Return only valid JSON matching the exact field names above. Do not include explanations or additional text.

Example valid response:
{{
  "form_no": "Mẫu 01-GTKT",
  "serial_no": "AA/24E",
  "invoice_no": "0000123",
  "issued_date": "2024-03-15",
  "seller_name": "CÔNG TY TNHH ABC",
  "seller_tax_code": "0123456789",
  "item_name": "Dịch vụ tư vấn",
  "unit": "giờ",
  "quantity": 10.00,
  "unit_price": 500000.00,
  "total_amount": 5000000.00,
  "vat_rate": 10.00,
  "vat_amount": 500000.00
}}

Now analyze the provided images and extract the data:"#,
            request.options.confidence_threshold,
            request.options.language_hint
        )
    }

    /// Creates the JSON schema for structured bill data response
    ///
    /// Defines the exact structure that Gemini should follow when returning
    /// extracted bill data. This schema matches the BillData struct fields
    /// and ensures consistent, parseable responses.
    ///
    /// # Returns
    /// * `Value` - JSON schema object for the API request
    fn create_bill_data_schema(&self) -> Value {
        serde_json::json!({
            "type": "OBJECT",
            "properties": {
                "form_no": {
                    "type": "STRING",
                    "description": "Form number or template identifier (e.g., 'Mẫu 01-GTKT')",
                    "nullable": true
                },
                "serial_no": {
                    "type": "STRING",
                    "description": "Serial number of the invoice",
                    "nullable": true
                },
                "invoice_no": {
                    "type": "STRING",
                    "description": "Invoice number or identifier",
                    "nullable": true
                },
                "issued_date": {
                    "type": "STRING",
                    "description": "Issue date in YYYY-MM-DD format",
                    "pattern": "^\\d{4}-\\d{2}-\\d{2}$",
                    "nullable": true
                },
                "seller_name": {
                    "type": "STRING",
                    "description": "Name of the selling company",
                    "nullable": true
                },
                "seller_tax_code": {
                    "type": "STRING",
                    "description": "Tax identification code of the seller",
                    "nullable": true
                },
                "item_name": {
                    "type": "STRING",
                    "description": "Name or description of the item/service",
                    "nullable": true
                },
                "unit": {
                    "type": "STRING",
                    "description": "Unit of measurement (e.g., 'kg', 'piece', 'hour')",
                    "nullable": true
                },
                "quantity": {
                    "type": "NUMBER",
                    "description": "Quantity of items (decimal number)",
                    "nullable": true
                },
                "unit_price": {
                    "type": "NUMBER",
                    "description": "Price per unit (decimal number)",
                    "nullable": true
                },
                "total_amount": {
                    "type": "NUMBER",
                    "description": "Total amount before tax (decimal number)",
                    "nullable": true
                },
                "vat_rate": {
                    "type": "NUMBER",
                    "description": "VAT rate as percentage (e.g., 10.0 for 10%)",
                    "nullable": true
                },
                "vat_amount": {
                    "type": "NUMBER",
                    "description": "VAT amount (decimal number)",
                    "nullable": true
                }
            },
            "required": [], // All fields are optional since OCR may not find everything
            "additionalProperties": false
        })
    }

    /// Extracts retry-after seconds from an error response body
    ///
    /// Attempts to parse retry delay information from Gemini API error responses.
    /// This is used for rate limiting scenarios to determine appropriate backoff.
    ///
    /// # Arguments
    /// * `response_body` - The error response body from the API
    ///
    /// # Returns
    /// * `Option<u64>` - Retry delay in seconds, if found
    fn extract_retry_after_seconds(&self, response_body: &str) -> Option<u64> {
        // Try to parse JSON error response for retry information
        if let Ok(error_json) = serde_json::from_str::<Value>(response_body) {
            // Check for common retry-after fields in Gemini error responses
            if let Some(retry_after) = error_json.get("retryAfter") {
                if let Some(seconds_str) = retry_after.as_str() {
                    // Handle duration strings like "60s"
                    if seconds_str.ends_with('s') {
                        if let Ok(seconds) = seconds_str.trim_end_matches('s').parse::<u64>() {
                            return Some(seconds);
                        }
                    }
                } else if let Some(seconds) = retry_after.as_u64() {
                    return Some(seconds);
                }
            }

            // Check for error details with retry information
            if let Some(error) = error_json.get("error") {
                if let Some(details) = error.get("details") {
                    if let Some(details_array) = details.as_array() {
                        for detail in details_array {
                            if let Some(retry_info) = detail.get("retryInfo") {
                                if let Some(retry_delay) = retry_info.get("retryDelay") {
                                    if let Some(seconds_str) = retry_delay.as_str() {
                                        if seconds_str.ends_with('s') {
                                            if let Ok(seconds) = seconds_str.trim_end_matches('s').parse::<u64>() {
                                                return Some(seconds);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // If no specific retry information found, return None
        // The caller will use a default value
        None
    }

    /// Gets the current configuration
    ///
    /// Returns a reference to the service configuration for inspection or logging.
    pub fn config(&self) -> &GeminiConfig {
        &self.config
    }

    /// Validates that the service is properly configured and operational
    ///
    /// Performs a quick health check to ensure the service can communicate
    /// with the Gemini API. This is useful for startup validation.
    ///
    /// # Returns
    /// * `Result<(), OcrProcessingError>` - Success or configuration error
    pub async fn health_check(&self) -> Result<(), OcrProcessingError> {
        debug!("Performing GeminiService health check");

        // Validate configuration is still valid
        self.config.validate().map_err(|e| {
            OcrProcessingError::configuration(format!(
                "Service configuration is invalid: {}",
                e
            ))
        })?;

        // Test basic HTTP client functionality
        let test_url = format!("{}/models", self.config.base_url);
        let response = self
            .client
            .get(&test_url)
            .query(&[("key", &self.config.api_key)])
            .send()
            .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    info!("GeminiService health check passed");
                    Ok(())
                } else {
                    let status = resp.status();
                    let error_msg = format!(
                        "Gemini API health check failed with status: {}",
                        status
                    );
                    warn!("{}", error_msg);

                    if status == 401 {
                        Err(OcrProcessingError::configuration(
                            "Invalid API key - check GEMINI_API_KEY environment variable"
                        ))
                    } else {
                        Err(OcrProcessingError::api(
                            error_msg,
                            Some(status.as_u16()),
                            None,
                            None,
                        ))
                    }
                }
            }
            Err(e) => {
                let error_msg = format!("Failed to connect to Gemini API: {}", e);
                error!("{}", error_msg);
                Err(OcrProcessingError::network(error_msg, None))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gemini_service_creation_invalid_config() {
        let invalid_config = GeminiConfig::new("".to_string()); // Empty API key
        let result = GeminiService::new(invalid_config);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), OcrProcessingError::Configuration { .. }));
    }

    #[test]
    fn test_gemini_service_creation_valid_config() {
        let valid_config = GeminiConfig::new("test-api-key".to_string());
        let result = GeminiService::new(valid_config);
        assert!(result.is_ok());

        let service = result.unwrap();
        assert_eq!(service.config().api_key, "test-api-key");
        assert_eq!(service.config().model, "gemini-pro-vision");
    }

    #[tokio::test]
    async fn test_prepare_gemini_request_validation() {
        let config = GeminiConfig::new("test-api-key".to_string());
        let service = GeminiService::new(config).unwrap();

        // Test with empty request (should fail validation)
        let empty_request = GeminiOCRRequest::new(vec![]);
        let result = service.prepare_gemini_request(&empty_request).await;
        assert!(result.is_err());

        // Test with valid request
        let image_data = crate::models::ImageData::new(
            vec![1, 2, 3, 4], // Some dummy content
            "image/jpeg".to_string(),
            Some("test.jpg".to_string()),
        );
        let valid_request = GeminiOCRRequest::new(vec![image_data]);
        let result = service.prepare_gemini_request(&valid_request).await;
        assert!(result.is_ok());

        // Verify the request structure
        let request_json = result.unwrap();
        assert!(request_json.get("contents").is_some());
        assert!(request_json.get("generationConfig").is_some());
    }

    #[tokio::test]
    async fn test_parse_gemini_response_valid() {
        let config = GeminiConfig::new("test-api-key".to_string());
        let service = GeminiService::new(config).unwrap();

        // Create a valid Gemini API response
        let api_response = serde_json::json!({
            "candidates": [{
                "content": {
                    "parts": [{
                        "text": r#"{
                            "form_no": "Mẫu 01-GTKT",
                            "invoice_no": "INV-001",
                            "seller_name": "CÔNG TY TNHH ABC",
                            "seller_tax_code": "0123456789",
                            "total_amount": 1000.50,
                            "vat_rate": 10.0,
                            "vat_amount": 100.05,
                            "issued_date": "2024-03-15"
                        }"#
                    }]
                },
                "finishReason": "STOP"
            }]
        });

        let result = service.parse_gemini_response(api_response, 1).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.results.len(), 1);
        assert_eq!(response.processing_summary.successful_extractions, 1);

        let bill_data = &response.results[0].extracted_data;
        assert_eq!(bill_data.form_no, Some("Mẫu 01-GTKT".to_string()));
        assert_eq!(bill_data.invoice_no, Some("INV-001".to_string()));
        assert_eq!(bill_data.company_name, Some("CÔNG TY TNHH ABC".to_string()));
        assert!(bill_data.total_amount.is_some());
    }

    #[tokio::test]
    async fn test_parse_gemini_response_missing_candidates() {
        let config = GeminiConfig::new("test-api-key".to_string());
        let service = GeminiService::new(config).unwrap();

        let api_response = serde_json::json!({
            "something": "else"
        });

        let result = service.parse_gemini_response(api_response, 1).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), OcrProcessingError::Parse { .. }));
    }

    #[tokio::test]
    async fn test_parse_gemini_response_blocked_candidate() {
        let config = GeminiConfig::new("test-api-key".to_string());
        let service = GeminiService::new(config).unwrap();

        let api_response = serde_json::json!({
            "candidates": [{
                "finishReason": "SAFETY"
            }]
        });

        let result = service.parse_gemini_response(api_response, 1).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.results.len(), 0);
        assert_eq!(response.errors.len(), 1);
        assert!(response.errors[0].message.contains("SAFETY"));
    }

    #[tokio::test]
    async fn test_parse_gemini_response_invalid_json_content() {
        let config = GeminiConfig::new("test-api-key".to_string());
        let service = GeminiService::new(config).unwrap();

        let api_response = serde_json::json!({
            "candidates": [{
                "content": {
                    "parts": [{
                        "text": "invalid json content"
                    }]
                },
                "finishReason": "STOP"
            }]
        });

        let result = service.parse_gemini_response(api_response, 1).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), OcrProcessingError::Parse { .. }));
    }

    #[tokio::test]
    async fn test_stub_methods_return_errors() {
        let config = GeminiConfig::new("test-api-key".to_string());
        let service = GeminiService::new(config).unwrap();

        let test_request = GeminiOCRRequest::new(vec![]);

        // prepare_gemini_request and call_gemini_api are now implemented
        // call_gemini_api will fail with network error since we're using a test API key
        let result = service.call_gemini_api(serde_json::json!({})).await;
        assert!(result.is_err());
        // Should be a network or configuration error since test API key won't work

        // process_images method is still a stub
        assert!(service.process_images(test_request).await.is_err());
    }

    #[test]
    fn test_extract_decimal_field() {
        let config = GeminiConfig::new("test-api-key".to_string());
        let service = GeminiService::new(config).unwrap();

        // Test numeric value
        let json_data = serde_json::json!({
            "amount": 123.45
        });
        let result = service.extract_decimal_field(&json_data, "amount");
        assert!(result.is_some());

        // Test string value with Vietnamese formatting
        let json_data = serde_json::json!({
            "amount": "1.234,56 ₫"
        });
        let result = service.extract_decimal_field(&json_data, "amount");
        assert!(result.is_some());

        // Test missing field
        let json_data = serde_json::json!({});
        let result = service.extract_decimal_field(&json_data, "amount");
        assert!(result.is_none());
    }

    #[test]
    fn test_calculate_field_confidence() {
        let config = GeminiConfig::new("test-api-key".to_string());
        let service = GeminiService::new(config).unwrap();

        let json_data = serde_json::json!({
            "invoice_no": "INV-001",
            "tax_code": "0123456789"
        });

        let mut bill_data = BillData::default();
        bill_data.invoice_no = Some("INV-001".to_string());
        bill_data.tax_code = Some("0123456789".to_string());

        // Test confidence for valid invoice number
        let confidence = service.calculate_field_confidence(&json_data, &bill_data, "invoice_no");
        assert!(confidence > 0.8);

        // Test confidence for valid Vietnamese tax code
        let confidence = service.calculate_field_confidence(&json_data, &bill_data, "tax_code");
        assert!(confidence > 0.9);

        // Test confidence for missing field
        let confidence = service.calculate_field_confidence(&json_data, &bill_data, "missing_field");
        assert_eq!(confidence, 0.0);
    }

    #[test]
    fn test_validate_extracted_data() {
        let config = GeminiConfig::new("test-api-key".to_string());
        let service = GeminiService::new(config).unwrap();

        // Test valid data
        let mut bill_data = BillData::default();
        bill_data.total_amount = Some(rust_decimal::Decimal::from(1000));
        bill_data.tax_rate = Some(rust_decimal::Decimal::from(10));
        bill_data.tax_amount = Some(rust_decimal::Decimal::from(100));
        bill_data.issue_date = Some(chrono::Utc::now().naive_utc().date() - chrono::Duration::days(30));

        let result = service.validate_extracted_data(&bill_data);
        assert!(result.is_ok());

        // Test invalid data - negative amount
        bill_data.total_amount = Some(rust_decimal::Decimal::from(-100));
        let result = service.validate_extracted_data(&bill_data);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("negative")));
    }

    #[test]
    fn test_count_populated_fields() {
        let config = GeminiConfig::new("test-api-key".to_string());
        let service = GeminiService::new(config).unwrap();

        let mut bill_data = BillData::default();
        assert_eq!(service.count_populated_fields(&bill_data), 0);

        bill_data.invoice_no = Some("INV-001".to_string());
        bill_data.total_amount = Some(rust_decimal::Decimal::from(1000));
        assert_eq!(service.count_populated_fields(&bill_data), 2);
    }

    #[test]
    fn test_config_access() {
        let config = GeminiConfig::new("test-api-key".to_string());
        let service = GeminiService::new(config.clone()).unwrap();

        assert_eq!(service.config().api_key, config.api_key);
        assert_eq!(service.config().model, config.model);
    }
}