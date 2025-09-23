# Data Model: Gemini OCR Integration

**Date**: 2025-09-22
**Status**: Phase 1 Design

## Core Entities

### GeminiOCRRequest
**Purpose**: Represents a request to process images through Gemini API
**Lifecycle**: Created per API call, processed sequentially, discarded after response

```rust
pub struct GeminiOCRRequest {
    pub images: Vec<ImageData>,
    pub options: ProcessingOptions,
}

pub struct ImageData {
    pub content: Vec<u8>,          // Raw image bytes
    pub mime_type: String,         // "image/jpeg", "image/png"
    pub filename: Option<String>,  // Original filename for logging
    pub size_bytes: usize,         // File size validation
}

pub struct ProcessingOptions {
    pub timeout_seconds: u64,      // Per-image timeout
    pub language_hint: String,     // "vi" for Vietnamese
    pub confidence_threshold: f32, // Minimum confidence for field extraction
}
```

**Validation Rules**:
- Max 10 images per request (sequential processing limit)
- Image size ≤ 20MB per file
- Supported MIME types: image/jpeg, image/png
- Timeout range: 10-120 seconds

### GeminiOCRResponse
**Purpose**: Structured response from Gemini API processing
**Lifecycle**: Generated after processing, returned to client, logged for debugging

```rust
pub struct GeminiOCRResponse {
    pub results: Vec<BillExtractionResult>,
    pub processing_summary: ProcessingSummary,
    pub errors: Vec<ProcessingError>,
}

pub struct BillExtractionResult {
    pub image_index: usize,                    // Matches request order
    pub extracted_data: BillData,              // Structured bill information
    pub confidence_scores: ConfidenceScores,   // Per-field confidence
    pub processing_time_ms: u64,               // Individual image processing time
}

pub struct BillData {
    // Maps to existing Bill table schema
    pub form_no: Option<String>,
    pub invoice_no: Option<String>,
    pub tax_code: Option<String>,
    pub company_name: Option<String>,
    pub company_address: Option<String>,
    pub buyer_name: Option<String>,
    pub buyer_address: Option<String>,
    pub issue_date: Option<chrono::NaiveDate>,
    pub total_amount: Option<rust_decimal::Decimal>,
    pub tax_amount: Option<rust_decimal::Decimal>,
    pub tax_rate: Option<rust_decimal::Decimal>,
    pub payment_method: Option<String>,
    pub notes: Option<String>,
}

pub struct ConfidenceScores {
    pub overall: f32,              // 0.0-1.0 overall extraction confidence
    pub field_scores: HashMap<String, f32>, // Per-field confidence
}
```

**State Transitions**:
1. **Pending**: Request received, images queued for processing
2. **Processing**: Currently being processed by Gemini API
3. **Completed**: Successfully processed with results
4. **Failed**: Processing failed with error details

### ProcessingError
**Purpose**: Detailed error information for failed operations
**Lifecycle**: Created on failure, included in response, logged for debugging

```rust
pub struct ProcessingError {
    pub error_type: ErrorType,
    pub message: String,
    pub image_index: Option<usize>,  // Which image failed (if applicable)
    pub retry_after_seconds: Option<u64>, // For rate limit errors
}

pub enum ErrorType {
    ConfigurationError,    // Missing API key, invalid config
    ValidationError,       // Invalid image format, size exceeded
    ApiError,             // Gemini API returned error
    TimeoutError,         // Processing timeout exceeded
    NetworkError,         // Connection issues
    ParseError,           // Invalid API response format
}
```

## Service Layer Entities

### GeminiService
**Purpose**: Core service for Gemini API integration
**Lifecycle**: Singleton, initialized at startup, reused across requests

```rust
pub struct GeminiService {
    client: reqwest::Client,       // HTTP client with connection pooling
    config: GeminiConfig,         // API configuration
    rate_limiter: RateLimiter,    // Request rate limiting
}

pub struct GeminiConfig {
    pub api_key: String,
    pub model_name: String,        // "gemini-pro-vision"
    pub base_url: String,          // API endpoint URL
    pub timeout: Duration,         // Default timeout
    pub max_image_size_mb: usize, // Size validation
}
```

## Integration with Existing Models

### Bill Entity Mapping
The `BillData` struct directly maps to the existing `Bill` table schema:

```sql
-- Existing bills table structure
CREATE TABLE bills (
    id SERIAL PRIMARY KEY,
    form_no TEXT,
    invoice_no TEXT,
    tax_code TEXT,
    company_name TEXT,
    company_address TEXT,
    buyer_name TEXT,
    buyer_address TEXT,
    issue_date DATE,
    total_amount NUMERIC(18,2),
    tax_amount NUMERIC(18,2),
    tax_rate NUMERIC(5,2),
    payment_method TEXT,
    notes TEXT
);
```

**Field Mapping Strategy**:
- Direct 1:1 mapping for compatible fields
- Type conversion: String → Option<String>, Decimal handling
- Date parsing with Vietnamese format support
- Currency extraction with VND to Decimal conversion

### OCR API Handler Enhancement
The existing `/api/ocr` endpoint will be enhanced to:
1. Accept the same multipart/form-data requests
2. Process images through GeminiService
3. Return structured BillData instead of raw text
4. Maintain backward compatibility where possible

## Validation and Constraints

### Input Validation
- Image format validation (JPEG, PNG only)
- File size limits (≤ 20MB per image)
- Request size limits (≤ 10 images per request)
- API key presence and format validation

### Data Validation
- Date format validation for Vietnamese formats
- Currency amount validation and conversion
- Tax rate percentage validation (0-100%)
- Text field length limits matching database schema

### Error Handling
- Graceful degradation for partial extraction failures
- Detailed error logging for debugging
- User-friendly error messages in API responses
- Rate limit handling with exponential backoff

---

**Data Model Status**: ✓ Complete - Ready for Contracts Generation