# Research: OCR Image Upload API Endpoint

**Phase 0 Research Results** | **Date**: 2025-09-20

## Research Questions Resolved

### 1. Axum Multipart Form Data Handling

**Decision**: Use Axum's native `Multipart` extractor with `axum-typed-multipart` for structured handling

**Rationale**:
- Axum 0.8.4 provides built-in multipart support through the `Multipart` extractor
- `axum-typed-multipart` adds type safety and automatic validation
- Supports streaming large files to avoid memory issues
- Integrates well with existing Axum ecosystem

**Alternatives Considered**:
- Raw multipart parsing with `multer` crate - more complex, less integrated
- Using `tower-multipart` - older approach, less maintained
- Custom implementation - unnecessary complexity

**Implementation Pattern**:
```rust
use axum::extract::Multipart;
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};

#[derive(TryFromMultipart)]
struct ImageUploadRequest {
    #[form_data(limit = "configurable")]
    images: Vec<FieldData<NamedTempFile>>,
}
```

### 2. Environment Variable Configuration

**Decision**: Use `dotenvy` with custom configuration struct and validation

**Rationale**:
- Already part of project dependencies (dotenvy 0.15.7)
- Provides clean environment variable loading
- Can implement custom validation and defaults
- Type-safe configuration with serde

**Alternatives Considered**:
- Direct `std::env::var` usage - no type safety, manual parsing
- `config` crate - additional dependency, overkill for simple config
- `figment` crate - too complex for this use case

**Implementation Pattern**:
```rust
use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct UploadConfig {
    max_file_size_bytes: usize,
    max_image_count: usize,
}

impl UploadConfig {
    fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();
        let max_file_size = env::var("MAX_FILE_SIZE_BYTES")
            .unwrap_or_else(|_| "2097152".to_string()) // 2MB default
            .parse()?;
        let max_image_count = env::var("MAX_IMAGE_COUNT")
            .unwrap_or_else(|_| "10".to_string())
            .parse()?;

        Ok(UploadConfig {
            max_file_size_bytes: max_file_size,
            max_image_count,
        })
    }
}
```

### 3. Image Format Validation

**Decision**: Use `infer` crate for magic byte detection + `image` crate for format validation

**Rationale**:
- `infer` provides fast magic byte detection without full parsing
- `image` crate validates actual image structure and decodability
- Two-layer validation prevents bypassing via file extension spoofing
- Minimal memory overhead for validation

**Alternatives Considered**:
- File extension only - easily spoofed, unreliable
- `imageproc` crate - heavier dependency, unnecessary features
- Custom magic byte detection - reinventing the wheel

**Implementation Pattern**:
```rust
use infer;
use image;

async fn validate_image_format(data: &[u8]) -> Result<String, ValidationError> {
    // First check: magic bytes
    let kind = infer::get(data)
        .ok_or(ValidationError::UnknownFormat)?;

    if !kind.mime_type().starts_with("image/") {
        return Err(ValidationError::NotAnImage);
    }

    // Second check: actual image parsing
    image::load_from_memory(data)
        .map_err(|_| ValidationError::CorruptedImage)?;

    Ok(kind.mime_type().to_string())
}
```

### 4. Error Handling Patterns

**Decision**: Use custom error types with `axum::response::IntoResponse` implementation

**Rationale**:
- Type-safe error handling with clear error categories
- Automatic HTTP status code mapping
- Consistent JSON error response format
- Easy to extend for additional validation types

**Alternatives Considered**:
- String-based errors - not type safe, poor debugging
- `anyhow` errors - too generic, loses semantic meaning
- HTTP status codes only - not descriptive enough

**Implementation Pattern**:
```rust
#[derive(Debug, thiserror::Error)]
enum UploadError {
    #[error("File size {size} bytes exceeds limit of {limit} bytes")]
    FileSizeExceeded { size: usize, limit: usize },

    #[error("Image count {count} exceeds limit of {limit}")]
    ImageCountExceeded { count: usize, limit: usize },

    #[error("Invalid image format: {0}")]
    InvalidImageFormat(String),

    #[error("Multipart parsing failed: {0}")]
    MultipartError(String),
}

impl IntoResponse for UploadError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            UploadError::FileSizeExceeded { .. } => (StatusCode::PAYLOAD_TOO_LARGE, self.to_string()),
            UploadError::ImageCountExceeded { .. } => (StatusCode::BAD_REQUEST, self.to_string()),
            UploadError::InvalidImageFormat(_) => (StatusCode::UNSUPPORTED_MEDIA_TYPE, self.to_string()),
            UploadError::MultipartError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        };

        Json(serde_json::json!({
            "error": message,
            "status": status.as_u16()
        })).into_response()
    }
}
```

## Additional Dependencies Required

```toml
# Add to backend/Cargo.toml
[dependencies]
axum-typed-multipart = "0.12.0"  # Type-safe multipart handling
tempfile = "3.0"                 # Temporary file management
image = "0.25.0"                 # Image format validation
infer = "0.16.0"                 # Magic byte detection
thiserror = "1.0"                # Error handling
serde_json = "1.0"               # JSON responses
```

## Configuration Environment Variables

The following environment variables will be supported:

- `MAX_FILE_SIZE_BYTES`: Maximum size per image file (default: 2097152 = 2MB)
- `MAX_IMAGE_COUNT`: Maximum number of images per request (default: 10)

## Security Considerations

1. **File Size Limits**: Both per-file and total request size limits
2. **Format Validation**: Magic byte + actual image parsing validation
3. **Memory Management**: Stream large files to temporary storage
4. **Request Limits**: Configurable limits to prevent DoS attacks

## Performance Characteristics

- **Memory Usage**: O(1) for streaming, O(file_size) for validation only
- **Processing Time**: O(file_count × avg_file_size) for validation
- **Disk Usage**: Temporary files cleaned up automatically
- **Concurrency**: Non-blocking async processing

---

**All NEEDS CLARIFICATION items resolved**
**Ready for Phase 1: Design & Contracts**