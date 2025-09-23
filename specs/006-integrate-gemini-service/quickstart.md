# Quickstart: Gemini OCR Integration

## Prerequisites

1. **Environment Setup**:
   ```bash
   # Add to .env file
   GEMINI_API_KEY=your_gemini_api_key_here
   DATABASE_URL=postgresql://user:pass@localhost/bill_ocr
   MAX_FILE_SIZE_BYTES=10485760  # 10MB
   ```

2. **Database**: Ensure bills table exists (from migration 002-create-a-bill)

3. **Dependencies**: Add to Cargo.toml:
   ```toml
   reqwest = { version = "0.11", features = ["json", "multipart"] }
   base64 = "0.21"
   tokio-stream = "0.1"
   ```

## Quick Test Scenario

### 1. Prepare Test Images
Create test directory with sample Vietnamese bill images:
```
tests/fixtures/
├── invoice1.jpg (valid Vietnamese invoice)
├── invoice2.png (valid Vietnamese receipt)
└── invalid.txt (unsupported format)
```

### 2. Start Server
```bash
cd backend
cargo run
# Server starts on http://localhost:3000
```

### 3. Test Basic OCR Processing

#### Single Image Success Case
```bash
curl -X POST http://localhost:3000/api/ocr \
  -F "files=@tests/fixtures/invoice1.jpg" \
  --no-buffer
```

**Expected SSE Output**:
```
event: started
data: {"type":"started","total_images":1,"batch_id":"batch_abc123"}

event: progress
data: {"type":"progress","filename":"invoice1.jpg","status":"processing","current":1,"total":1}

event: completed
data: {"type":"completed","filename":"invoice1.jpg","data":{"invoice_no":"INV-001","total_amount":"150000.00","seller_name":"Công ty ABC"}}

event: batch_complete
data: {"type":"batch_complete","total_processed":1,"successful":1,"failed":0}
```

#### Multiple Images with Mixed Results
```bash
curl -X POST http://localhost:3000/api/ocr \
  -F "files=@tests/fixtures/invoice1.jpg" \
  -F "files=@tests/fixtures/invoice2.png" \
  -F "files=@tests/fixtures/invalid.txt" \
  --no-buffer
```

**Expected SSE Output**:
```
event: started
data: {"type":"started","total_images":3,"batch_id":"batch_def456"}

event: progress
data: {"type":"progress","filename":"invoice1.jpg","status":"processing","current":1,"total":3}

event: completed
data: {"type":"completed","filename":"invoice1.jpg","data":{"invoice_no":"INV-001"}}

event: progress
data: {"type":"progress","filename":"invoice2.png","status":"processing","current":2,"total":3}

event: completed
data: {"type":"completed","filename":"invoice2.png","data":{"invoice_no":"REC-002"}}

event: error
data: {"type":"error","filename":"invalid.txt","error":"Unsupported file format"}

event: batch_complete
data: {"type":"batch_complete","total_processed":3,"successful":2,"failed":1}
```

### 4. Verify Database Storage

```sql
-- Check that bills were stored
SELECT invoice_no, seller_name, total_amount, created_at
FROM bills
ORDER BY created_at DESC
LIMIT 5;
```

Expected: Records for successfully processed invoices

### 5. Test Error Scenarios

#### File Size Limit
```bash
# Create large file
dd if=/dev/zero of=/tmp/large.jpg bs=1M count=20

curl -X POST http://localhost:3000/api/ocr \
  -F "files=@/tmp/large.jpg"
```

**Expected**: HTTP 400 with file size error

#### Rate Limiting Simulation
Mock high rate of requests to trigger Gemini rate limiting:
```bash
# Multiple rapid requests
for i in {1..10}; do
  curl -X POST http://localhost:3000/api/ocr \
    -F "files=@tests/fixtures/invoice1.jpg" &
done
```

**Expected**: Some requests return rate_limited SSE event

### 6. Frontend Integration Test

#### JavaScript SSE Client
```javascript
const eventSource = new EventSource('/api/ocr');

eventSource.onmessage = function(event) {
    const data = JSON.parse(event.data);
    console.log('Received:', data);

    switch(data.type) {
        case 'started':
            console.log(`Processing ${data.total_images} images`);
            break;
        case 'completed':
            console.log(`Extracted data from ${data.filename}:`, data.data);
            break;
        case 'error':
            console.error(`Failed to process ${data.filename}: ${data.error}`);
            break;
        case 'batch_complete':
            console.log(`Batch complete: ${data.successful}/${data.total_processed} successful`);
            eventSource.close();
            break;
    }
};

// Send files via form
const formData = new FormData();
formData.append('files', fileInput.files[0]);

fetch('/api/ocr', {
    method: 'POST',
    body: formData
});
```

## End-to-End Validation

### User Story 1: Successful Batch Processing
1. **Setup**: 3 valid Vietnamese bill images
2. **Action**: Upload via /api/ocr endpoint
3. **Verify**:
   - SSE events received in correct sequence
   - All 3 bills stored in database
   - Data matches expected Vietnamese invoice fields

### User Story 2: Partial Failure Handling
1. **Setup**: 2 valid images + 1 corrupted image
2. **Action**: Upload batch
3. **Verify**:
   - 2 successful completions
   - 1 error event for corrupted image
   - Processing continues for valid images
   - Database contains 2 new records

### User Story 3: Rate Limiting
1. **Setup**: Mock Gemini API to return 429
2. **Action**: Upload images
3. **Verify**:
   - rate_limited event sent
   - SSE connection closed
   - Clear retry message provided

## Performance Benchmarks

### Expected Metrics
- **Single image processing**: < 10 seconds
- **SSE event latency**: < 100ms
- **Memory usage**: < 50MB per batch
- **Database insertion**: < 500ms per record

### Load Testing
```bash
# Test with multiple concurrent uploads
ab -n 10 -c 2 -p invoice.jpg -T multipart/form-data http://localhost:3000/api/ocr
```

## Troubleshooting

### Common Issues
1. **No SSE events**: Check CORS headers for frontend
2. **Gemini API errors**: Verify GEMINI_API_KEY in .env
3. **Database errors**: Ensure PostgreSQL connection and bills table
4. **File upload fails**: Check MAX_FILE_SIZE_BYTES configuration

### Debug Commands
```bash
# Check environment variables
cargo run -- --check-env

# Test database connection
psql $DATABASE_URL -c "SELECT count(*) FROM bills;"

# Verify Gemini API access
curl -H "Authorization: Bearer $GEMINI_API_KEY" https://generativelanguage.googleapis.com/v1/models
```