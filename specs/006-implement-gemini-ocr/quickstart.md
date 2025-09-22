# Quickstart Guide: Gemini OCR Integration

**Date**: 2025-09-22
**Status**: Phase 1 Design

## Prerequisites

### Environment Setup
1. **Gemini API Access**: Obtain Google AI Studio API key
2. **Environment Configuration**: Add required variables to `.env`
3. **Dependencies**: Ensure Rust dependencies are updated
4. **Database**: Verify bills table exists with required schema

### Required Environment Variables
```env
# Add to backend/.env
GEMINI_API_KEY=your_gemini_api_key_here
GEMINI_MODEL=gemini-pro-vision
GEMINI_TIMEOUT_SECONDS=45
GEMINI_MAX_IMAGE_SIZE_MB=20
GEMINI_BASE_URL=https://generativelanguage.googleapis.com/v1beta
```

## Quick Test Scenarios

### Scenario 1: Single Vietnamese Bill Image
**Objective**: Verify basic OCR functionality with a clear Vietnamese invoice

**Test Steps**:
1. Prepare a clear Vietnamese bill image (JPEG/PNG, <20MB)
2. Send POST request to `/api/ocr` with image
3. Verify structured response with extracted bill data

**Expected Result**:
```json
{
  "results": [
    {
      "image_index": 0,
      "extracted_data": {
        "invoice_no": "INV-2024-001",
        "company_name": "Công ty ABC",
        "total_amount": "1500000.00",
        "issue_date": "2024-09-22"
      },
      "confidence_scores": {
        "overall": 0.85,
        "field_scores": {
          "invoice_no": 0.95,
          "company_name": 0.88
        }
      },
      "processing_time_ms": 2500
    }
  ],
  "processing_summary": {
    "total_images": 1,
    "successful_extractions": 1,
    "failed_extractions": 0,
    "total_processing_time_ms": 2500,
    "average_confidence": 0.85
  }
}
```

### Scenario 2: Multiple Images Sequential Processing
**Objective**: Test sequential processing of multiple bill images

**Test Steps**:
1. Prepare 3 different Vietnamese bill images
2. Send multipart request with all 3 images
3. Verify results are returned in same order as submitted
4. Check processing times are reasonable

**Expected Behavior**:
- Images processed one at a time (sequential)
- Results array matches input order
- Total processing time ≈ sum of individual times
- No parallel processing conflicts

### Scenario 3: Error Handling - Invalid Image
**Objective**: Verify graceful handling of non-bill images

**Test Steps**:
1. Upload a random photo (not a bill)
2. Send request to `/api/ocr`
3. Verify appropriate error handling

**Expected Result**:
```json
{
  "results": [
    {
      "image_index": 0,
      "extracted_data": {
        "form_no": null,
        "invoice_no": null,
        "company_name": null
      },
      "confidence_scores": {
        "overall": 0.1
      },
      "processing_time_ms": 1200
    }
  ],
  "processing_summary": {
    "successful_extractions": 0,
    "failed_extractions": 1
  },
  "errors": [
    {
      "error_type": "parse_error",
      "message": "No bill-like content detected in image",
      "image_index": 0
    }
  ]
}
```

### Scenario 4: Configuration Error
**Objective**: Test behavior when Gemini API key is missing/invalid

**Test Steps**:
1. Remove GEMINI_API_KEY from environment
2. Restart backend service
3. Attempt OCR request

**Expected Result**:
```json
{
  "error": "configuration_error",
  "message": "Gemini API key not configured or invalid",
  "details": {
    "required_env_vars": ["GEMINI_API_KEY"]
  }
}
```

## Manual Testing Commands

### cURL Examples

**Single Image Upload**:
```bash
curl -X POST http://localhost:3000/api/ocr \
  -F "images=@vietnamese_bill.jpg" \
  -F "options={\"timeout_seconds\":60,\"confidence_threshold\":0.7}"
```

**Multiple Images**:
```bash
curl -X POST http://localhost:3000/api/ocr \
  -F "images=@bill1.jpg" \
  -F "images=@bill2.png" \
  -F "images=@bill3.jpg"
```

**With Custom Options**:
```bash
curl -X POST http://localhost:3000/api/ocr \
  -F "images=@complex_bill.jpg" \
  -F "options={\"timeout_seconds\":90,\"confidence_threshold\":0.5,\"language_hint\":\"vi\"}"
```

### Development Testing

**Cargo Commands**:
```bash
# Run backend with OCR endpoint
cd backend && cargo run

# Run tests (when implemented)
cd backend && cargo test gemini_ocr

# Check compilation
cd backend && cargo check

# Lint code
cd backend && cargo clippy
```

## Integration Verification

### Database Integration Test
1. **Extract Bill Data**: Process a bill image through OCR
2. **Create Bill Record**: Use extracted data to create new bill entry
3. **Verify Database**: Check that Vietnamese text is properly stored
4. **Query Back**: Retrieve and verify data integrity

**SQL Verification**:
```sql
-- Check latest OCR-created bill
SELECT * FROM bills
WHERE notes LIKE '%OCR%'
ORDER BY id DESC
LIMIT 1;

-- Verify Vietnamese text encoding
SELECT company_name, buyer_name
FROM bills
WHERE company_name IS NOT NULL;
```

### Performance Benchmarks
- **Single Image**: <5 seconds end-to-end
- **Multiple Images**: <10 seconds per image
- **Memory Usage**: Stable during sequential processing
- **Database Operations**: <100ms per bill insertion

### Error Recovery Tests
1. **Network Interruption**: Verify graceful timeout handling
2. **API Rate Limits**: Test exponential backoff behavior
3. **Invalid Responses**: Handle malformed Gemini API responses
4. **Partial Failures**: Process remaining images when one fails

## Production Readiness Checklist

### Configuration
- [ ] Gemini API key securely configured
- [ ] Timeout values tuned for production workload
- [ ] Image size limits appropriate for user needs
- [ ] Rate limiting configured to prevent API quota exhaustion

### Monitoring
- [ ] Request/response logging implemented
- [ ] Error rate monitoring in place
- [ ] Processing time metrics tracked
- [ ] API quota usage monitored

### Security
- [ ] API key not logged or exposed
- [ ] Input validation on all image uploads
- [ ] File type restrictions enforced
- [ ] Request size limits configured

---

**Quickstart Status**: ✓ Complete - Ready for Implementation