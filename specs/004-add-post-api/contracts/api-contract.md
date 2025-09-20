# API Contract: OCR Image Upload Endpoint

**Endpoint**: `POST /api/ocr`
**Phase 1 Contract** | **Date**: 2025-09-20

## Request Specification

### HTTP Details
- **Method**: POST
- **Path**: `/api/ocr`
- **Content-Type**: `multipart/form-data`
- **Authentication**: None (for this phase)

### Request Body (Multipart Form Data)

#### Form Fields

**images** (required)
- **Type**: File upload field (multiple files allowed)
- **Description**: One or more image files to process
- **Constraints**:
  - Minimum: 1 file
  - Maximum: Configurable via `MAX_IMAGE_COUNT` env var (default: 10)
  - Supported formats: JPEG, PNG, GIF, WebP
  - Max file size per image: Configurable via `MAX_FILE_SIZE_BYTES` env var (default: 2MB)

**metadata** (optional)
- **Type**: Text field
- **Description**: Optional metadata about the upload batch
- **Constraints**:
  - Max length: 1000 characters
  - Format: Plain text or JSON string

### Request Examples

#### Valid Request (curl)
```bash
curl -X POST http://localhost:3000/api/ocr \
  -F "images=@invoice1.jpg" \
  -F "images=@receipt2.png" \
  -F "metadata=Batch upload from mobile app"
```

#### Valid Request (HTTP)
```http
POST /api/ocr HTTP/1.1
Host: localhost:3000
Content-Type: multipart/form-data; boundary=----WebKitFormBoundary7MA4YWxkTrZu0gW

------WebKitFormBoundary7MA4YWxkTrZu0gW
Content-Disposition: form-data; name="images"; filename="invoice.jpg"
Content-Type: image/jpeg

[binary image data]
------WebKitFormBoundary7MA4YWxkTrZu0gW
Content-Disposition: form-data; name="images"; filename="receipt.png"
Content-Type: image/png

[binary image data]
------WebKitFormBoundary7MA4YWxkTrZu0gW
Content-Disposition: form-data; name="metadata"

Invoice batch from accounting system
------WebKitFormBoundary7MA4YWxkTrZu0gW--
```

## Response Specification

### Success Response (200 OK)

```json
{
  "success": true,
  "message": "Images validated successfully",
  "data": {
    "accepted_images": [
      {
        "file_name": "invoice.jpg",
        "content_type": "image/jpeg",
        "size_bytes": 1048576,
        "format": "JPEG",
        "validation_status": "Valid"
      },
      {
        "file_name": "receipt.png",
        "content_type": "image/png",
        "size_bytes": 524288,
        "format": "PNG",
        "validation_status": "Valid"
      }
    ],
    "total_count": 2,
    "total_size_bytes": 1572864,
    "processing_time_ms": 150
  }
}
```

### Error Responses

#### 400 Bad Request - Validation Failure
```json
{
  "success": false,
  "error": "Validation failed",
  "details": {
    "errors": [
      {
        "error_type": "FileSizeExceeded",
        "message": "File size 3145728 exceeds limit 2097152",
        "file_name": "large_image.jpg"
      },
      {
        "error_type": "InvalidImageFormat",
        "message": "Invalid image format: application/pdf",
        "file_name": "document.pdf"
      }
    ],
    "accepted_images": [
      {
        "file_name": "valid_image.png",
        "content_type": "image/png",
        "size_bytes": 512000,
        "format": "PNG",
        "validation_status": "Valid"
      }
    ],
    "total_count": 3,
    "rejected_count": 2
  }
}
```

#### 413 Payload Too Large - Request Size Exceeded
```json
{
  "success": false,
  "error": "Request payload too large",
  "details": {
    "message": "Total request size exceeds maximum allowed",
    "max_size_bytes": 52428800,
    "received_size_bytes": 67108864
  }
}
```

#### 422 Unprocessable Entity - Too Many Files
```json
{
  "success": false,
  "error": "Too many images",
  "details": {
    "message": "Image count 15 exceeds limit 10",
    "max_image_count": 10,
    "received_count": 15
  }
}
```

#### 415 Unsupported Media Type - Invalid Content Type
```json
{
  "success": false,
  "error": "Unsupported media type",
  "details": {
    "message": "Content-Type must be multipart/form-data",
    "received_content_type": "application/json"
  }
}
```

#### 500 Internal Server Error - Server Failure
```json
{
  "success": false,
  "error": "Internal server error",
  "details": {
    "message": "An unexpected error occurred during processing",
    "request_id": "req_abc123xyz"
  }
}
```

## HTTP Status Code Mapping

| Status Code | Condition | Response Type |
|-------------|-----------|---------------|
| 200 | All images validated successfully | Success |
| 400 | Image validation failures (format, corruption) | Partial/Error |
| 413 | Individual file or total request size exceeded | Error |
| 415 | Invalid content type (not multipart/form-data) | Error |
| 422 | Too many images in request | Error |
| 500 | Server error during processing | Error |

## Validation Rules

### File-Level Validation
1. **Format Validation**: Magic byte detection + image parsing
2. **Size Validation**: Per-file size against configured limit
3. **Content Validation**: Verify image can be decoded
4. **Name Validation**: Sanitize filename for security

### Request-Level Validation
1. **Count Validation**: Total image count against configured limit
2. **Content-Type Validation**: Must be multipart/form-data
3. **Field Validation**: Required 'images' field present
4. **Size Validation**: Total request size within bounds

### Configuration Validation
1. **Environment Variables**: Validate numeric values and ranges
2. **Default Fallback**: Use safe defaults for missing config
3. **Limit Ranges**: Enforce reasonable min/max values

## Security Considerations

### Input Validation
- Magic byte validation prevents file type spoofing
- File size limits prevent memory exhaustion
- Request count limits prevent DoS attacks
- Filename sanitization prevents path traversal

### Resource Management
- Temporary files auto-cleanup after processing
- Memory usage controlled through streaming
- Request timeout to prevent hanging connections
- Error responses don't leak internal details

### Rate Limiting (Future Consideration)
- Per-IP request rate limiting
- Total concurrent upload limits
- File processing queue management

## Performance Characteristics

### Expected Performance
- **Processing Time**: 50-200ms per image for validation
- **Memory Usage**: < 50MB total during processing
- **Throughput**: 100+ concurrent requests with proper limits
- **File Size**: Up to configured limit (default 2MB per file)

### Scalability Limits
- **Max Concurrent**: Dependent on system resources
- **Max File Size**: Hardware memory constraints
- **Max Request Size**: Axum body limit configuration
- **Processing Queue**: Temporary file system capacity

## Testing Requirements

### Contract Tests
- Validate request/response schemas
- Test all error scenarios and status codes
- Verify multipart parsing behavior
- Check configuration loading and defaults

### Integration Tests
- End-to-end request processing
- File upload and validation flow
- Error handling and edge cases
- Performance under load

### Example Test Cases
1. Single valid image upload
2. Multiple valid images upload
3. Mixed valid/invalid images
4. File size limit exceeded
5. Image count limit exceeded
6. Invalid image formats
7. Corrupted image data
8. Missing required fields
9. Invalid content type
10. Configuration edge cases