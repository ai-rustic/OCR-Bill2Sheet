# Data Model: Gemini OCR Integration

## Core Entities

### GeminiRequest
**Purpose**: Represents a request to Gemini API for bill extraction
**Fields**:
- `image_data`: base64-encoded image content
- `filename`: original filename for tracking
- `format`: validated image format (JPG/PNG/JFIF)
- `prompt`: structured output request prompt

**Validation Rules**:
- Image data must be valid base64
- Format must be one of: JPG, PNG, JFIF
- File size within MAX_FILE_SIZE_BYTES limit

### GeminiResponse
**Purpose**: Structured response from Gemini API matching bills schema
**Fields**:
```rust
#[derive(Deserialize, Serialize)]
pub struct GeminiResponse {
    pub form_no: Option<String>,
    pub invoice_no: Option<String>,
    pub invoice_series: Option<String>,
    pub invoice_date: Option<String>, // Will be parsed to NaiveDate
    pub seller_name: Option<String>,
    pub seller_tax_code: Option<String>,
    pub seller_address: Option<String>,
    pub buyer_name: Option<String>,
    pub buyer_tax_code: Option<String>,
    pub buyer_address: Option<String>,
    pub total_amount: Option<String>, // Will be parsed to Decimal
    pub tax_rate: Option<String>, // Will be parsed to Decimal
    pub tax_amount: Option<String>, // Will be parsed to Decimal
    pub payment_method: Option<String>,
}
```

**Validation Rules**:
- Date fields must parse to valid NaiveDate
- Amount fields must parse to valid Decimal
- Tax rate must be between 0-100 if present

### ProcessingStatus
**Purpose**: Tracks the status of each image in batch processing
**States**:
- `Pending`: Queued for processing
- `Processing`: Currently being analyzed by Gemini
- `Completed`: Successfully extracted data
- `Failed`: Processing failed with error

**Fields**:
- `filename`: Image filename
- `status`: Current processing state
- `progress`: Sequential position in batch
- `error_message`: Error details if failed
- `extracted_data`: GeminiResponse if successful

### SSEEvent
**Purpose**: Server-sent event payload for real-time updates
**Types**:
```rust
#[derive(Serialize)]
#[serde(tag = "type")]
pub enum SSEEvent {
    Started {
        total_images: usize,
        batch_id: String,
    },
    Progress {
        filename: String,
        status: ProcessingStatus,
        current: usize,
        total: usize,
    },
    Completed {
        filename: String,
        data: GeminiResponse,
    },
    Error {
        filename: String,
        error: String,
    },
    RateLimited {
        message: String,
        retry_after: Option<u64>,
    },
    BatchComplete {
        total_processed: usize,
        successful: usize,
        failed: usize,
    },
}
```

## Data Flow

### Input Processing
1. **Upload Validation**: Check file formats, sizes against limits
2. **Base64 Encoding**: Convert images for API transmission
3. **Batch Preparation**: Create processing queue with metadata

### Gemini API Integration
1. **Request Formation**: Structure prompt for bill extraction
2. **API Call**: Send image + structured output request
3. **Response Parsing**: Validate and transform to GeminiResponse
4. **Error Handling**: Categorize failures for appropriate action

### Database Integration
1. **Data Transformation**: Convert GeminiResponse to CreateBill
2. **Validation**: Ensure data meets database constraints
3. **Persistence**: Store using existing BillService

### SSE Streaming
1. **Connection Management**: Handle client connections
2. **Event Broadcasting**: Send real-time updates per image
3. **Error Propagation**: Stream errors without breaking connection
4. **Completion Notification**: Summary when batch finished

## Relationships

```
UploadedImages 1..* → ProcessingQueue
ProcessingQueue 1 → GeminiRequest
GeminiRequest 1 → GeminiResponse
GeminiResponse 1 → CreateBill
CreateBill 1 → Bill (database)
ProcessingStatus → SSEEvent (real-time)
```

## State Transitions

### Processing Status Flow
```
Pending → Processing → Completed
                  ↘ Failed
```

### Batch Processing Flow
```
Upload → Validate → Queue → Process → Store → Notify
                          ↘ Error → Continue Next
```

## Constraints

### Performance
- Sequential processing (no parallel Gemini calls)
- Memory-efficient streaming (don't load all images at once)
- Connection timeout handling for SSE

### Business Rules
- Continue processing on individual image failures
- Rate limiting triggers immediate batch termination
- All successful extractions must be stored to database

### Technical Constraints
- Must use existing MAX_FILE_SIZE_BYTES configuration
- Must authenticate with GEMINI_API_KEY from environment
- Must conform to bills database schema exactly