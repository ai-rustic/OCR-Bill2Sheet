# SSE API Contract: /api/ocr Endpoint

## Endpoint Overview
**Path**: `/api/ocr`
**Method**: `POST`
**Content-Type**: `multipart/form-data`
**Response-Type**: `text/event-stream`
**Purpose**: Upload images and receive real-time validation progress via Server-Sent Events

## Request Specification

### HTTP Headers
```http
POST /api/ocr HTTP/1.1
Host: localhost:3000
Content-Type: multipart/form-data; boundary=----WebKitFormBoundary7MA4YWxkTrZu0gW
Accept: text/event-stream
Cache-Control: no-cache
```

### Multipart Form Data
```
------WebKitFormBoundary7MA4YWxkTrZu0gW
Content-Disposition: form-data; name="images"; filename="invoice1.jpg"
Content-Type: image/jpeg

[binary image data]
------WebKitFormBoundary7MA4YWxkTrZu0gW
Content-Disposition: form-data; name="images"; filename="receipt2.png"
Content-Type: image/png

[binary image data]
------WebKitFormBoundary7MA4YWxkTrZu0gW--
```

### Request Constraints
- **Field name**: Must be `images` for all uploaded files
- **File count**: Limited by `MAX_IMAGE_COUNT` environment variable
- **File size**: Each file limited by `MAX_FILE_SIZE_BYTES` environment variable
- **Supported formats**: JPEG, PNG, GIF (validated by content, not extension)
- **Total request size**: Limited by Axum's multipart limits (2MB per file)

## Response Specification

### HTTP Response Headers
```http
HTTP/1.1 200 OK
Content-Type: text/event-stream
Cache-Control: no-cache
Connection: keep-alive
Access-Control-Allow-Origin: *
Access-Control-Allow-Headers: Cache-Control
```

### SSE Event Format
All events follow this structure:
```
event: {event_type}
id: {optional_event_id}
data: {json_payload}

```

### Event Types and Payloads

#### 1. upload_started
Sent immediately when processing begins.
```
event: upload_started
data: {
  "type": "upload_started",
  "data": {
    "total_files": 3,
    "session_id": "550e8400-e29b-41d4-a716-446655440000",
    "timestamp": "2025-09-22T10:30:00.000Z"
  }
}

```

#### 2. image_received
Sent for each successfully received image file.
```
event: image_received
data: {
  "type": "image_received",
  "data": {
    "file_index": 0,
    "file_name": "invoice1.jpg",
    "size_bytes": 1024000,
    "timestamp": "2025-09-22T10:30:00.100Z"
  }
}

```

#### 3. image_validation_start
Sent when validation begins for each image.
```
event: image_validation_start
data: {
  "type": "image_validation_start",
  "data": {
    "file_index": 0,
    "file_name": "invoice1.jpg",
    "timestamp": "2025-09-22T10:30:00.150Z"
  }
}

```

#### 4. image_validation_success
Sent when an image passes all validations.
```
event: image_validation_success
data: {
  "type": "image_validation_success",
  "data": {
    "file_index": 0,
    "file_info": {
      "file_name": "invoice1.jpg",
      "content_type": "image/jpeg",
      "size_bytes": 1024000,
      "format": "JPEG",
      "validation_status": "Valid",
      "file_index": 0,
      "processed_at": "2025-09-22T10:30:00.200Z",
      "processing_duration_ms": 50
    },
    "timestamp": "2025-09-22T10:30:00.200Z"
  }
}

```

#### 5. image_validation_error
Sent when an image fails validation.
```
event: image_validation_error
data: {
  "type": "image_validation_error",
  "data": {
    "file_index": 1,
    "file_name": "invalid.txt",
    "error_message": "Unsupported file format detected",
    "error_code": {
      "UnsupportedFormat": {
        "detected": "text/plain"
      }
    },
    "timestamp": "2025-09-22T10:30:00.300Z"
  }
}

```

#### 6. all_images_validated
Sent after all images have been processed.
```
event: all_images_validated
data: {
  "type": "all_images_validated",
  "data": {
    "total_processed": 3,
    "successful_count": 2,
    "failed_count": 1,
    "timestamp": "2025-09-22T10:30:00.500Z"
  }
}

```

#### 7. processing_complete
Sent when all processing is successfully completed.
```
event: processing_complete
data: {
  "type": "processing_complete",
  "data": {
    "session_id": "550e8400-e29b-41d4-a716-446655440000",
    "total_files": 3,
    "successful_files": 2,
    "duration_ms": 500,
    "timestamp": "2025-09-22T10:30:00.600Z"
  }
}

```

#### 8. processing_error
Sent when a system-level error occurs.
```
event: processing_error
data: {
  "type": "processing_error",
  "data": {
    "session_id": "550e8400-e29b-41d4-a716-446655440000",
    "error_message": "Processing timeout exceeded",
    "error_type": "SystemTimeout",
    "timestamp": "2025-09-22T10:30:00.700Z"
  }
}

```

### Keep-Alive Events
Sent periodically to maintain connection:
```
: keep-alive

```

## Error Response Codes

### 400 Bad Request
Returned for validation failures before SSE stream starts:
```http
HTTP/1.1 400 Bad Request
Content-Type: application/json

{
  "success": false,
  "error": "No images provided in request"
}
```

### 413 Payload Too Large
Returned when file size limits are exceeded:
```http
HTTP/1.1 413 Payload Too Large
Content-Type: application/json

{
  "success": false,
  "error": "File size exceeds maximum allowed (2MB)"
}
```

### 429 Too Many Requests
Returned when concurrent connection limits are reached:
```http
HTTP/1.1 429 Too Many Requests
Content-Type: application/json

{
  "success": false,
  "error": "Maximum concurrent connections exceeded"
}
```

## Client Integration Examples

### JavaScript/Fetch API
```javascript
const formData = new FormData();
formData.append('images', file1);
formData.append('images', file2);

const eventSource = new EventSource('/api/ocr', {
  method: 'POST',
  body: formData
});

eventSource.addEventListener('upload_started', (event) => {
  const data = JSON.parse(event.data);
  console.log(`Processing ${data.data.total_files} files`);
});

eventSource.addEventListener('image_validation_success', (event) => {
  const data = JSON.parse(event.data);
  console.log(`File ${data.data.file_info.file_name} validated successfully`);
});

eventSource.addEventListener('processing_complete', (event) => {
  eventSource.close();
  console.log('Upload processing completed');
});

eventSource.addEventListener('error', (event) => {
  console.error('SSE connection error:', event);
  eventSource.close();
});
```

### cURL Example
```bash
curl -X POST http://localhost:3000/api/ocr \
  -H "Accept: text/event-stream" \
  -H "Cache-Control: no-cache" \
  -F "images=@invoice1.jpg" \
  -F "images=@receipt2.png" \
  --no-buffer
```

## Event Sequence Guarantees

### Ordering
1. Events for each file are sent in order: `image_received` → `image_validation_start` → (`image_validation_success` | `image_validation_error`)
2. `upload_started` is always the first event
3. `all_images_validated` is sent after all individual file events
4. `processing_complete` or `processing_error` is always the final event

### Timing
- Events are sent as soon as each processing step completes
- No artificial delays or batching
- Client disconnection immediately stops event generation

### Error Handling
- Individual file validation errors do not terminate the stream
- System errors terminate the stream with `processing_error` event
- Network interruptions require client reconnection (no automatic retry)

## Configuration Dependencies

### Environment Variables
- `MAX_IMAGE_COUNT`: Maximum number of images per request
- `MAX_FILE_SIZE_BYTES`: Maximum size per image file
- `SSE_BUFFER_SIZE`: Internal event buffer size (default: 1000)
- `SSE_KEEP_ALIVE_INTERVAL`: Keep-alive interval in seconds (default: 15)

### Runtime Limits
- Maximum concurrent SSE connections: 100 (configurable)
- Processing timeout: 30 seconds per request
- Event buffer overflow handling: Drop oldest events