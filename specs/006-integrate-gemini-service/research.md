# Research: Gemini AI OCR Integration

## Gemini AI API Integration

### Decision: Use Gemini 1.5 Pro with Structured Output
**Rationale**:
- Supports image input and structured JSON output
- Excellent Vietnamese text recognition capabilities
- Rate limiting can be handled gracefully
- Multimodal capabilities for bill/invoice processing

**Alternatives considered**:
- OpenAI GPT-4V: More expensive, less structured output control
- Google Cloud Vision API: No structured output, requires additional processing
- Local OCR (Tesseract): Poor Vietnamese text recognition

### API Authentication Pattern
**Decision**: Use GEMINI_API_KEY environment variable with Bearer token
**Implementation**: Store in `.env`, validate on startup

## Server-Sent Events (SSE) with Axum

### Decision: Use `axum::response::sse` module
**Rationale**:
- Built-in SSE support in Axum 0.8.4
- Handles connection management automatically
- Supports async streaming

**Implementation Pattern**:
```rust
use axum::response::sse::{Event, Sse};
use tokio_stream::{wrappers::UnboundedReceiverStream, StreamExt};

async fn ocr_stream() -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    // Create stream for processing updates
}
```

## Image Processing

### Decision: Base64 encoding for Gemini API
**Rationale**:
- Gemini API accepts base64-encoded images
- Supports JPG, PNG, JFIF formats as specified
- Can validate format before processing

**Libraries**:
- `base64` crate for encoding
- `image` crate for format validation

## Rate Limiting Strategy

### Decision: Fail-fast with user notification
**Rationale**:
- Aligns with requirement FR-011
- Better user experience than silent queuing
- Prevents resource exhaustion

**Implementation**:
- Detect 429 status from Gemini API
- Close SSE stream immediately
- Send clear retry message to user

## Vietnamese Text Processing

### Decision: No additional preprocessing required
**Rationale**:
- Gemini 1.5 Pro has excellent Vietnamese language support
- Bills typically use standard Vietnamese fonts
- Structured output request will handle encoding

## Structured Output Schema

### Decision: Mirror bills database schema exactly
**Rationale**:
- Ensures compatibility with existing data model
- Reduces transformation overhead
- Maintains data consistency

**Schema Fields** (from existing bills table):
- form_no, invoice_no, invoice_series, invoice_date
- seller_name, seller_tax_code, seller_address
- buyer_name, buyer_tax_code, buyer_address
- total_amount, tax_rate, tax_amount, payment_method

## Error Handling Patterns

### Decision: Continue processing on individual failures
**Rationale**:
- Aligns with requirement FR-008
- Better user experience for batch operations
- Allows partial success scenarios

**Error Categories**:
1. Image format/size errors: Skip image, continue batch
2. Gemini API errors: Retry once, then skip
3. Network errors: Fail entire batch
4. Rate limiting: Close stream, notify user

## Dependencies Assessment

### Required New Dependencies:
- `reqwest` (v0.11+): HTTP client for Gemini API calls
- `base64` (v0.21+): Image encoding
- `tokio-stream` (v0.1+): SSE streaming utilities
- `serde_json` (v1.0+): JSON parsing for API responses

### Existing Dependencies (confirmed compatible):
- `axum` 0.8.4: SSE support available
- `sqlx` 0.8.6: Database operations
- `tokio` 1.47.1: Async runtime
- `dotenvy` 0.15.7: Environment configuration