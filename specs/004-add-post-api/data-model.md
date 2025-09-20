# Data Model: OCR Image Upload API

**Phase 1 Design** | **Date**: 2025-09-20

## Core Entities

### 1. UploadConfig
Configuration entity for environment-based limits.

**Fields**:
- `max_file_size_bytes: usize` - Maximum size per image file in bytes
- `max_image_count: usize` - Maximum number of images per request

**Validation Rules**:
- `max_file_size_bytes` must be > 0 and <= 50MB (50,331,648 bytes)
- `max_image_count` must be > 0 and <= 100

**Source**: Environment variables with defaults
- `MAX_FILE_SIZE_BYTES` (default: 2,097,152 = 2MB)
- `MAX_IMAGE_COUNT` (default: 10)

### 2. ImageUploadRequest
Request entity representing incoming multipart form data.

**Fields**:
- `images: Vec<FieldData<NamedTempFile>>` - Collection of uploaded image files
- `metadata: Option<String>` - Optional metadata about the upload

**Validation Rules**:
- Images count must not exceed `UploadConfig.max_image_count`
- Each image size must not exceed `UploadConfig.max_file_size_bytes`
- Each file must be a valid image format (JPEG, PNG, GIF, WebP)
- File content must match detected format (magic byte validation)

**State Transitions**:
1. **Received** - Raw multipart data received
2. **Validating** - Size and format validation in progress
3. **Validated** - All validation checks passed
4. **Rejected** - Validation failed with specific error

### 3. ImageFileInfo
Metadata entity for each validated image file.

**Fields**:
- `file_name: Option<String>` - Original filename from upload
- `content_type: String` - Detected MIME type (e.g., "image/jpeg")
- `size_bytes: usize` - File size in bytes
- `format: String` - Detected image format (JPEG, PNG, etc.)
- `temp_path: String` - Path to temporary file
- `validation_status: ValidationStatus` - Current validation state

**Validation Rules**:
- `content_type` must start with "image/"
- `size_bytes` must be > 0 and <= configured limit
- `format` must be supported (JPEG, PNG, GIF, WebP)
- `temp_path` must point to accessible temporary file

### 4. ValidationResult
Response entity containing validation outcome.

**Fields**:
- `success: bool` - Overall validation success
- `accepted_images: Vec<ImageFileInfo>` - Successfully validated images
- `errors: Vec<ValidationError>` - List of validation failures
- `total_count: usize` - Total number of files processed
- `total_size_bytes: usize` - Combined size of all accepted files

**Validation Rules**:
- If `success = true`, `errors` must be empty
- If `success = false`, `errors` must not be empty
- `accepted_images.len()` <= `UploadConfig.max_image_count`
- Sum of `accepted_images[].size_bytes` <= practical memory limits

### 5. ValidationError
Error entity for specific validation failures.

**Fields**:
- `error_type: ErrorType` - Category of validation error
- `message: String` - Human-readable error description
- `file_name: Option<String>` - Associated filename if applicable
- `details: Option<serde_json::Value>` - Additional error context

**Error Types**:
- `FileSizeExceeded` - File exceeds size limit
- `ImageCountExceeded` - Too many images in request
- `InvalidImageFormat` - Not a valid image file
- `CorruptedImage` - Image data is corrupted
- `UnsupportedFormat` - Image format not supported
- `MultipartParsingError` - Failed to parse multipart data

## Entity Relationships

```
UploadConfig (1) ← validates ← (1) ImageUploadRequest
                                         ↓
                                   (1 to many)
                                         ↓
                                 ImageFileInfo
                                         ↓
                                   (aggregates to)
                                         ↓
                                 ValidationResult
                                         ↓
                                   (may contain)
                                         ↓
                                 ValidationError (0 to many)
```

## Data Flow

1. **Configuration Loading**: `UploadConfig` loaded from environment at startup
2. **Request Receiving**: `ImageUploadRequest` created from multipart data
3. **File Processing**: Each file becomes an `ImageFileInfo` with validation
4. **Result Aggregation**: `ValidationResult` aggregates all outcomes
5. **Error Collection**: `ValidationError` instances collected for failures

## Storage Patterns

### Temporary File Management
- Images stored in temporary files during validation
- Automatic cleanup after request processing
- No persistence to permanent storage in this feature

### Memory Usage
- Stream large files directly to temp storage
- Keep only metadata in memory during processing
- Validate files sequentially to control memory usage

### Configuration Caching
- `UploadConfig` loaded once at startup
- Cached in application state for request processing
- Environment changes require application restart

## Rust Type Mappings

```rust
// Configuration
#[derive(Debug, Clone)]
struct UploadConfig {
    max_file_size_bytes: usize,
    max_image_count: usize,
}

// Request handling
#[derive(TryFromMultipart)]
struct ImageUploadRequest {
    #[form_data(limit = "configurable")]
    images: Vec<FieldData<NamedTempFile>>,
    metadata: Option<String>,
}

// File information
#[derive(Debug, Serialize)]
struct ImageFileInfo {
    file_name: Option<String>,
    content_type: String,
    size_bytes: usize,
    format: String,
    temp_path: String,
    validation_status: ValidationStatus,
}

// Response types
#[derive(Debug, Serialize)]
struct ValidationResult {
    success: bool,
    accepted_images: Vec<ImageFileInfo>,
    errors: Vec<ValidationError>,
    total_count: usize,
    total_size_bytes: usize,
}

#[derive(Debug, Serialize, thiserror::Error)]
enum ValidationError {
    #[error("File size {size} exceeds limit {limit}")]
    FileSizeExceeded { size: usize, limit: usize },

    #[error("Image count {count} exceeds limit {limit}")]
    ImageCountExceeded { count: usize, limit: usize },

    #[error("Invalid image format: {format}")]
    InvalidImageFormat { format: String },

    #[error("Corrupted image data")]
    CorruptedImage,

    #[error("Unsupported format: {format}")]
    UnsupportedFormat { format: String },

    #[error("Multipart parsing error: {details}")]
    MultipartParsingError { details: String },
}

#[derive(Debug, Serialize)]
enum ValidationStatus {
    Pending,
    Validating,
    Valid,
    Invalid(String),
}
```