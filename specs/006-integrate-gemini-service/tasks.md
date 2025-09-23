# Tasks: Integrate Gemini Service into API/OCR

**Input**: Design documents from `/specs/006-integrate-gemini-service/`
**Prerequisites**: Current backend already has OCR infrastructure with SSE, need to integrate Gemini AI

## Current Backend Analysis

### Already Available:
- ✅ OCR endpoint with SSE streaming (`/api/ocr`)
- ✅ Image validation service
- ✅ ProcessingEvent system for real-time updates
- ✅ Upload configuration and error handling
- ✅ Bill service and database schema
- ✅ Dependencies: axum, tokio-stream, image, base64

### Need to Add:
- ❌ Gemini AI API client
- ❌ Structured data extraction from images
- ❌ Integration with existing ProcessingEvent system
- ❌ Bill data persistence from Gemini responses

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths

## Phase 1: Setup and Dependencies

- [x] T001 Add reqwest dependency to backend/Cargo.toml for Gemini API calls
- [x] T002 [P] Add GEMINI_API_KEY to backend/.env.example with documentation
- [x] T003 [P] Update backend/src/utils/env.rs to load GEMINI_API_KEY

## Phase 2: Gemini Integration Models

- [x] T004 [P] Create GeminiRequest struct in backend/src/models/gemini_request.rs
- [x] T005 [P] Create GeminiResponse struct in backend/src/models/gemini_response.rs
- [x] T006 [P] Extend ProcessingEvent enum in backend/src/models/sse_events.rs with Gemini events
- [x] T007 Update backend/src/models/mod.rs to export new models

## Phase 3: Gemini Service Implementation

- [x] T008 [P] Create GeminiService in backend/src/services/gemini_service.rs
- [x] T009 [P] Create BillDataExtractor in backend/src/services/bill_extractor.rs
- [x] T010 Update backend/src/services/mod.rs to export new services

## Phase 4: Integration with Current OCR Pipeline

- [x] T011 Extend validate_file function in backend/src/api/ocr.rs to include Gemini processing
- [ ] T012 Add Gemini processing step to process_upload_with_events function
- [ ] T013 Integrate with BillService to persist extracted data to database
- [ ] T014 Add error handling for Gemini API rate limits and failures

## Phase 5: Testing and Polish

- [ ] T015 [P] Test Gemini integration with sample Vietnamese invoices
- [ ] T016 [P] Verify database persistence of extracted bill data
- [ ] T017 [P] Test SSE events for Gemini processing pipeline
- [ ] T018 Add logging and monitoring for Gemini API calls

## Dependencies

**Setup Phase**:
- T001 → T008, T009 (need reqwest dependency before creating GeminiService)
- T002, T003 can run in parallel

**Model Phase** (All [P] - different files):
- T004-T006 can run in parallel
- T007 depends on T004-T006 completion

**Service Phase**:
- T008, T009 can run in parallel (different files)
- T010 depends on T008, T009 completion

**Integration Phase** (Sequential - same files):
- T011-T014 sequential (modify existing backend/src/api/ocr.rs)
- T013 depends on existing BillService

**Testing Phase** (All [P]):
- T015-T018 can run in parallel

## Parallel Execution Examples

### Phase 2 Models:
```bash
Task: "Create GeminiRequest struct in backend/src/models/gemini_request.rs"
Task: "Create GeminiResponse struct in backend/src/models/gemini_response.rs"
Task: "Extend ProcessingEvent enum in backend/src/models/sse_events.rs"
```

### Phase 3 Services:
```bash
Task: "Create GeminiService in backend/src/services/gemini_service.rs"
Task: "Create BillDataExtractor in backend/src/services/bill_extractor.rs"
```

## Implementation Details

### T001: Add Gemini Dependencies
Add to backend/Cargo.toml:
```toml
reqwest = { version = "0.11", features = ["json"] }
```

### T004: GeminiRequest Model
```rust
#[derive(Debug, Serialize)]
pub struct GeminiRequest {
    pub image_data: String, // base64 encoded
    pub prompt: String,     // structured output request
}
```

### T005: GeminiResponse Model
```rust
#[derive(Debug, Deserialize)]
pub struct GeminiResponse {
    // Mirror bills database schema exactly
    pub form_no: Option<String>,
    pub invoice_no: Option<String>,
    pub invoice_series: Option<String>,
    pub invoice_date: Option<String>,
    pub seller_name: Option<String>,
    pub seller_tax_code: Option<String>,
    pub seller_address: Option<String>,
    pub buyer_name: Option<String>,
    pub buyer_tax_code: Option<String>,
    pub buyer_address: Option<String>,
    pub total_amount: Option<String>,
    pub tax_rate: Option<String>,
    pub tax_amount: Option<String>,
    pub payment_method: Option<String>,
}
```

### T006: Extend ProcessingEvent
Add to existing enum:
```rust
pub enum ProcessingEvent {
    // ... existing variants ...
    GeminiProcessingStart {
        file_index: usize,
        file_name: Option<String>,
        timestamp: DateTime<Utc>,
    },
    GeminiProcessingSuccess {
        file_index: usize,
        extracted_data: GeminiResponse,
        timestamp: DateTime<Utc>,
    },
    GeminiProcessingError {
        file_index: usize,
        error_message: String,
        timestamp: DateTime<Utc>,
    },
    BillDataSaved {
        file_index: usize,
        bill_id: i32,
        timestamp: DateTime<Utc>,
    },
}
```

### T008: GeminiService Implementation
```rust
pub struct GeminiService {
    client: reqwest::Client,
    api_key: String,
}

impl GeminiService {
    pub async fn extract_bill_data(&self, image_data: &[u8]) -> Result<GeminiResponse, GeminiError> {
        // Base64 encode image
        // Send structured request to Gemini API
        // Parse response to GeminiResponse
        // Handle rate limiting (429)
    }
}
```

### T011-T012: Integration with OCR Pipeline
Modify process_upload_with_events to:
1. After successful validation → Send GeminiProcessingStart event
2. Call GeminiService.extract_bill_data()
3. Send GeminiProcessingSuccess/Error events
4. Persist data via BillService
5. Send BillDataSaved event

### T013: Database Persistence
```rust
// Convert GeminiResponse to CreateBill
let create_bill = CreateBill {
    form_no: gemini_response.form_no,
    invoice_no: gemini_response.invoice_no,
    // ... map all fields
};

// Use existing BillService
let bill_id = bill_service.create_bill(create_bill).await?;
```

## Integration with Current System

### New Event Flow:
```
UploadStarted → ImageReceived → ImageValidationStart → ImageValidationSuccess
→ GeminiProcessingStart → GeminiProcessingSuccess → BillDataSaved
→ AllImagesValidated → ProcessingComplete
```

### Error Handling:
- Rate limiting (429): Send GeminiProcessingError, continue with other images
- API errors: Send GeminiProcessingError, continue processing
- Database errors: Log but continue (don't break stream)

### Configuration:
- GEMINI_API_KEY from environment
- Existing MAX_FILE_SIZE_BYTES configuration
- Existing supported formats (JPG, PNG) from image validation

## Constitutional Compliance

✅ **Skip TDD**: All tasks create working code directly
✅ **SQLx Integration**: Use existing BillService with compile-time validation
✅ **Environment Config**: GEMINI_API_KEY from environment
✅ **Speed Priority**: Parallel execution to accelerate development
✅ **Build on existing**: Leverage available infrastructure

## Validation Checklist

- [x] Integration with existing OCR pipeline
- [x] Use existing ProcessingEvent system
- [x] Parallel tasks are truly independent
- [x] Each task has specific file path
- [x] No task modifies same file as another [P] task
- [x] Complies with constitutional requirements (skip TDD)
- [x] Vietnamese invoice context considered

## Notes

- **[P] tasks** = different files, no dependencies
- **Sequential tasks** = same file or dependent operations
- **Commit after each task** for incremental progress
- **Test manually** with Vietnamese invoice samples after completion
- **Monitor Gemini API usage** to avoid rate limits