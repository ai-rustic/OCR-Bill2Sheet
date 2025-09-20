# OCR_Bill2Sheet Backend

Rust backend service for OCR bill processing with image upload capabilities.

## Features

- **Bill Management API**: Complete CRUD operations for Vietnamese invoice data
- **OCR Image Upload API**: Multi-file image upload with validation and format detection
- **PostgreSQL Integration**: Database operations with SQLx and compile-time query validation
- **Environment Configuration**: Configurable file size and count limits

## API Endpoints

### Health Endpoints

- `GET /health` - Basic health check
- `GET /health/detail` - Detailed health check with database connectivity

### Bill Management Endpoints

- `GET /api/bills` - Get all bills
- `POST /api/bills` - Create a new bill
- `GET /api/bills/search` - Search bills with query parameters
- `GET /api/bills/count` - Get total bill count
- `GET /api/bills/{id}` - Get bill by ID
- `PUT /api/bills/{id}` - Update bill by ID
- `DELETE /api/bills/{id}` - Delete bill by ID

### OCR Image Upload Endpoint

#### POST /api/ocr

Upload multiple images for OCR processing with validation.

**Request Format**: `multipart/form-data`

**Form Fields**:
- `images` (required): One or more image files
  - Supported formats: JPEG, PNG, GIF, WebP
  - Max file size: Configurable via `MAX_FILE_SIZE_BYTES` (default: 2MB)
  - Max image count: Configurable via `MAX_IMAGE_COUNT` (default: 10)
- `metadata` (optional): Text metadata about the upload batch

**Success Response (200 OK)**:
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
      }
    ],
    "total_count": 1,
    "total_size_bytes": 1048576,
    "processing_time_ms": 150
  }
}
```

**Error Responses**:
- `400 Bad Request`: Multipart parsing failed or no images provided
- `413 Payload Too Large`: File size exceeds configured limit
- `415 Unsupported Media Type`: Invalid image format
- `422 Unprocessable Entity`: Too many images in request
- `500 Internal Server Error`: Server processing error

**Example Usage**:
```bash
# Single image upload
curl -X POST http://localhost:3000/api/ocr \
  -F "images=@invoice.jpg" \
  -F "metadata=Invoice batch from mobile app"

# Multiple image upload
curl -X POST http://localhost:3000/api/ocr \
  -F "images=@invoice1.jpg" \
  -F "images=@receipt2.png" \
  -F "images=@bill3.jpg"
```

## Configuration

Environment variables (set in `.env` file):

### Database Configuration
- `DATABASE_URL`: PostgreSQL connection string
- `DATABASE_MAX_CONNECTIONS`: Max database connections (default: 10)
- `DATABASE_ACQUIRE_TIMEOUT`: Connection acquire timeout in seconds (default: 30)
- `DATABASE_IDLE_TIMEOUT`: Idle connection timeout in seconds (default: 600)
- `DATABASE_MAX_LIFETIME`: Connection max lifetime in seconds (default: 1800)

### Upload Configuration
- `MAX_FILE_SIZE_BYTES`: Maximum size per image file in bytes (default: 2097152 = 2MB)
- `MAX_IMAGE_COUNT`: Maximum number of images per request (default: 10)

## Development

### Prerequisites
- Rust 1.75+ (edition = "2024")
- PostgreSQL server
- Environment variables configured

### Running the Server
```bash
# Copy environment template
cp .env.example .env

# Edit .env with your database credentials and upload limits

# Run the server
cargo run

# Or with debug logging
RUST_LOG=debug cargo run
```

### Testing
```bash
# Run library tests (note: some existing tests may fail due to pre-existing issues)
cargo check

# Run linting
cargo clippy --lib

# Test the OCR endpoint manually
./test_single_image.sh
./test_multiple_images.sh
./test_file_size_limit.sh
./test_invalid_format.sh
./test_image_count_limit.sh
```

## Architecture

- **Axum**: Web framework with multipart form support
- **SQLx**: Async PostgreSQL driver with compile-time query validation
- **Tower**: Middleware stack for request handling and timeouts
- **Tracing**: Structured logging and observability
- **Custom Error Types**: Comprehensive error handling with proper HTTP status codes

## Security Features

- **File Type Validation**: Magic byte detection + image parsing validation
- **Size Limits**: Configurable file size and request body limits
- **Input Sanitization**: Filename and path validation
- **Request Timeouts**: Protection against hanging connections
- **Resource Management**: Automatic cleanup of temporary files

## Logging

The server provides structured logging with:
- Request/response logging with timing
- File validation progress
- Error details for debugging
- Database connection health

Use `RUST_LOG=debug` for detailed logging during development.