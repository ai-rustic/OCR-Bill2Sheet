# Research: Gemini OCR Integration

**Date**: 2025-09-22
**Status**: Complete

## Research Questions Resolved

### 1. Gemini API Integration Method

**Decision**: Direct HTTP calls to Gemini REST API using `reqwest`
**Rationale**:
- User provided direct API integration approach
- Uses generativelanguage.googleapis.com endpoint
- Supports structured JSON responses via responseSchema
- No additional client library dependencies needed

**Implementation Approach**:
```rust
// Direct API call example
let response = reqwest::Client::new()
    .post("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent")
    .header("x-goog-api-key", api_key)
    .header("Content-Type", "application/json")
    .json(&request_body)
    .send()
    .await?;
```

**Request Structure**:
- `contents`: Array with image parts and text prompts
- `generationConfig`: Specify JSON response format and schema
- `responseSchema`: Define structure matching Bill table fields

### 2. Performance Targets and API Constraints

**Decision**: Temporarily ignore performance constraints for MVP
**Rationale**:
- User specified to temporarily ignore for initial implementation
- Focus on basic functionality first
- Performance optimization can be addressed in future iterations

**MVP Implementation Approach**:
- Use reasonable default timeouts (45 seconds)
- Implement basic error handling without sophisticated retry logic
- Sequential processing without rate limit optimization
- Standard image validation (format, size checks)

### 3. Structured Output for Vietnamese Text

**Decision**: Use Gemini's JSON mode with custom prompt engineering
**Rationale**:
- Gemini Pro supports structured JSON responses
- Vietnamese text handling built into model
- Custom prompts can enforce specific field extraction
- Fallback to text parsing if JSON mode fails

**Research Findings**:
- Gemini handles Vietnamese characters natively
- JSON schema can be specified in prompt
- Currency formatting preservation requires specific instructions
- Confidence scoring available through API responses

**Implementation Strategy**:
- Design JSON schema matching Bill table fields
- Craft prompts for Vietnamese invoice recognition
- Include field descriptions and examples
- Handle partial extractions gracefully

### 4. Error Handling Patterns

**Decision**: Implement multi-layer error handling with user-friendly responses
**Rationale**:
- API failures should not crash the service
- Users need meaningful error messages
- Logging required for debugging API issues
- Graceful degradation for partial failures

**Error Categories**:
1. **Configuration Errors**: Missing API key, invalid endpoint
2. **Request Errors**: Invalid image format, size exceeded
3. **API Errors**: Rate limit, service unavailable, timeout
4. **Processing Errors**: No text detected, invalid response format

**Response Strategy**:
- Return structured error JSON with error codes
- Log detailed errors for debugging
- Provide user-friendly error messages
- Maintain service availability during API outages

## Integration Architecture

### Service Layer Design
```rust
pub struct GeminiService {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
    timeout: Duration,
}

pub struct OCRRequest {
    images: Vec<ImageData>,
    extraction_schema: BillSchema,
}

pub struct OCRResponse {
    results: Vec<BillData>,
    errors: Vec<ProcessingError>,
    processing_time: Duration,
}
```

### Configuration Requirements
```env
GEMINI_API_KEY=your_api_key_here
GEMINI_MODEL=gemini-2.5-flash
GEMINI_TIMEOUT_SECONDS=45
GEMINI_BASE_URL=https://generativelanguage.googleapis.com/v1beta
```

### Bill Field Mapping
Based on existing Bills table schema:
- `form_no` → "Form Number" field extraction
- `invoice_no` → "Invoice Number" identification
- `tax_code` → "Tax Code" pattern matching
- `company_name` → Company/business name extraction
- `total_amount` → Total amount with currency conversion
- `issue_date` → Date parsing with Vietnamese format support
- All other fields mapped to corresponding Vietnamese invoice sections

## Next Steps for Phase 1

1. **Data Model**: Define Rust structs for Gemini integration
2. **API Contracts**: Design enhanced /api/ocr endpoint specification
3. **Contract Tests**: Create failing tests for new functionality
4. **Quickstart**: Document setup and usage examples

---

**Research Status**: ✓ Complete - Ready for Phase 1 Design