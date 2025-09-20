# Quickstart: OCR Image Upload API

**Phase 1 Implementation Guide** | **Date**: 2025-09-20

## Prerequisites

### Environment Setup
1. **Rust Development Environment**
   ```bash
   # Verify Rust installation
   cargo --version  # Should be 1.75+
   ```

2. **Environment Configuration**
   ```bash
   # Create/update .env file in backend/ directory
   echo "MAX_FILE_SIZE_BYTES=2097152" >> backend/.env
   echo "MAX_IMAGE_COUNT=10" >> backend/.env
   echo "DATABASE_URL=postgresql://username:password@localhost/bill_ocr" >> backend/.env
   ```

3. **Test Images**
   ```bash
   # Prepare test images in project root
   mkdir -p test-images
   # Add sample JPEG/PNG files for testing
   ```

## Implementation Steps

### Step 1: Add Dependencies
Update `backend/Cargo.toml`:
```toml
[dependencies]
# Existing dependencies...
axum = { version = "0.8.4", features = ["multipart"] }
axum-typed-multipart = "0.12.0"
tempfile = "3.0"
image = "0.25.0"
infer = "0.16.0"
thiserror = "1.0"
serde_json = "1.0"
```

### Step 2: Configuration Module
Create `backend/src/config/upload_config.rs`:
```rust
use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct UploadConfig {
    pub max_file_size_bytes: usize,
    pub max_image_count: usize,
}

impl UploadConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();

        let max_file_size = env::var("MAX_FILE_SIZE_BYTES")
            .unwrap_or_else(|_| "2097152".to_string())
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

### Step 3: Error Types
Create `backend/src/errors/upload_error.rs`:
```rust
use axum::{response::IntoResponse, http::StatusCode};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum UploadError {
    #[error("File size {size} exceeds limit {limit}")]
    FileSizeExceeded { size: usize, limit: usize },

    #[error("Image count {count} exceeds limit {limit}")]
    ImageCountExceeded { count: usize, limit: usize },

    #[error("Invalid image format: {0}")]
    InvalidImageFormat(String),

    #[error("Multipart parsing failed: {0}")]
    MultipartError(String),
}

impl IntoResponse for UploadError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            UploadError::FileSizeExceeded { .. } => {
                (StatusCode::PAYLOAD_TOO_LARGE, self.to_string())
            }
            UploadError::ImageCountExceeded { .. } => {
                (StatusCode::UNPROCESSABLE_ENTITY, self.to_string())
            }
            UploadError::InvalidImageFormat(_) => {
                (StatusCode::UNSUPPORTED_MEDIA_TYPE, self.to_string())
            }
            UploadError::MultipartError(_) => {
                (StatusCode::BAD_REQUEST, self.to_string())
            }
        };

        axum::Json(json!({
            "success": false,
            "error": message,
            "status": status.as_u16()
        })).into_response()
    }
}
```

### Step 4: Validation Service
Create `backend/src/services/image_validation.rs`:
```rust
use crate::errors::UploadError;
use bytes::Bytes;

pub async fn validate_image_format(data: &[u8]) -> Result<String, UploadError> {
    // Magic byte validation
    let kind = infer::get(data)
        .ok_or_else(|| UploadError::InvalidImageFormat("Unknown format".to_string()))?;

    if !kind.mime_type().starts_with("image/") {
        return Err(UploadError::InvalidImageFormat("Not an image".to_string()));
    }

    // Image parsing validation
    image::load_from_memory(data)
        .map_err(|_| UploadError::InvalidImageFormat("Corrupted image".to_string()))?;

    Ok(kind.mime_type().to_string())
}

pub fn validate_file_size(size: usize, limit: usize) -> Result<(), UploadError> {
    if size > limit {
        return Err(UploadError::FileSizeExceeded { size, limit });
    }
    Ok(())
}
```

### Step 5: API Handler
Create `backend/src/api/ocr.rs`:
```rust
use axum::{
    extract::{Multipart, State},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::sync::Arc;

use crate::{
    config::UploadConfig,
    errors::UploadError,
    services::image_validation::{validate_image_format, validate_file_size},
};

pub async fn upload_images(
    State(config): State<Arc<UploadConfig>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, UploadError> {
    let mut image_count = 0;
    let mut accepted_images = Vec::new();
    let mut total_size = 0;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| UploadError::MultipartError(e.to_string()))?
    {
        if field.name() == Some("images") {
            image_count += 1;

            if image_count > config.max_image_count {
                return Err(UploadError::ImageCountExceeded {
                    count: image_count,
                    limit: config.max_image_count,
                });
            }

            let file_name = field.file_name().map(|s| s.to_string());
            let data = field
                .bytes()
                .await
                .map_err(|e| UploadError::MultipartError(e.to_string()))?;

            validate_file_size(data.len(), config.max_file_size_bytes)?;
            let content_type = validate_image_format(&data).await?;

            total_size += data.len();

            accepted_images.push(json!({
                "file_name": file_name,
                "content_type": content_type,
                "size_bytes": data.len(),
                "format": content_type.split('/').nth(1).unwrap_or("unknown").to_uppercase(),
                "validation_status": "Valid"
            }));
        }
    }

    if image_count == 0 {
        return Err(UploadError::MultipartError("No images provided".to_string()));
    }

    Ok(Json(json!({
        "success": true,
        "message": "Images validated successfully",
        "data": {
            "accepted_images": accepted_images,
            "total_count": image_count,
            "total_size_bytes": total_size,
            "processing_time_ms": 0 // Placeholder
        }
    })))
}
```

### Step 6: Route Integration
Update `backend/src/main.rs`:
```rust
mod api;
mod config;
mod errors;
mod services;

use axum::{routing::post, Router};
use std::sync::Arc;
use tower_http::limit::RequestBodyLimitLayer;

use config::UploadConfig;

#[tokio::main]
async fn main() {
    // Load configuration
    let upload_config = Arc::new(
        UploadConfig::from_env().expect("Failed to load upload configuration")
    );

    // Build router
    let app = Router::new()
        .route("/api/ocr", post(api::ocr::upload_images))
        .with_state(upload_config.clone())
        .layer(RequestBodyLimitLayer::new(50 * 1024 * 1024)); // 50MB total

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
```

## Testing the Implementation

### Test 1: Single Image Upload
```bash
cd backend
cargo run &
sleep 2

# Test with a valid image
curl -X POST http://localhost:3000/api/ocr \
  -F "images=@../test-images/sample.jpg" \
  -v
```

**Expected Output**: 200 OK with validation success JSON

### Test 2: Multiple Images
```bash
curl -X POST http://localhost:3000/api/ocr \
  -F "images=@../test-images/image1.jpg" \
  -F "images=@../test-images/image2.png" \
  -v
```

**Expected Output**: 200 OK with multiple images in response

### Test 3: File Size Limit
```bash
# Create a large test file (>2MB)
dd if=/dev/zero of=../test-images/large.jpg bs=1M count=3

curl -X POST http://localhost:3000/api/ocr \
  -F "images=@../test-images/large.jpg" \
  -v
```

**Expected Output**: 413 Payload Too Large error

### Test 4: Invalid File Type
```bash
echo "Not an image" > ../test-images/text.txt

curl -X POST http://localhost:3000/api/ocr \
  -F "images=@../test-images/text.txt" \
  -v
```

**Expected Output**: 415 Unsupported Media Type error

### Test 5: Too Many Images
```bash
# Create script to upload 11 images (exceeds default limit of 10)
for i in {1..11}; do
  cp ../test-images/sample.jpg ../test-images/image$i.jpg
done

curl -X POST http://localhost:3000/api/ocr \
  $(for i in {1..11}; do echo -n "-F images=@../test-images/image$i.jpg "; done) \
  -v
```

**Expected Output**: 422 Unprocessable Entity error

## Validation Checklist

### ✅ Functional Requirements
- [ ] POST /api/ocr endpoint accepts multipart/form-data
- [ ] Multiple image files supported in single request
- [ ] File size validation per configured limit
- [ ] Image count validation per configured limit
- [ ] Environment variable configuration working
- [ ] Proper HTTP status codes returned
- [ ] Clear error messages for validation failures
- [ ] Image format validation (magic bytes + parsing)
- [ ] No persistence/storage (temporary processing only)

### ✅ Error Handling
- [ ] File size exceeded → 413 status
- [ ] Too many images → 422 status
- [ ] Invalid format → 415 status
- [ ] Parsing errors → 400 status
- [ ] Server errors → 500 status

### ✅ Configuration
- [ ] MAX_FILE_SIZE_BYTES environment variable
- [ ] MAX_IMAGE_COUNT environment variable
- [ ] Default values when env vars missing
- [ ] Configuration loaded at startup

### ✅ Performance
- [ ] Memory usage stays reasonable during upload
- [ ] Temporary files cleaned up automatically
- [ ] Processing completes within expected timeframes
- [ ] Multiple concurrent requests handled

## Troubleshooting

### Common Issues

**Error: "Failed to load upload configuration"**
- Check `.env` file exists in backend/ directory
- Verify environment variable values are valid numbers
- Ensure proper file permissions on `.env`

**Error: "Connection refused"**
- Verify server is running with `cargo run`
- Check port 3000 is not in use by another process
- Confirm firewall allows connections on port 3000

**Error: "Request entity too large"**
- Check Axum body limit configuration
- Verify total request size within limits
- Consider increasing RequestBodyLimitLayer value

**Images not validating properly**
- Verify test images are valid JPEG/PNG files
- Check file permissions on test images
- Ensure infer and image crates properly installed

### Debug Commands
```bash
# Check server logs
RUST_LOG=debug cargo run

# Test with verbose curl output
curl -v -X POST http://localhost:3000/api/ocr -F "images=@test.jpg"

# Verify environment variables
cargo run -- --version  # Should show config loaded

# Check file formats
file test-images/*  # Should show image types
```

## Next Steps

After successful quickstart validation:
1. **Phase 2**: Run `/tasks` command to generate detailed implementation tasks
2. **Implementation**: Execute tasks in order for production-ready code
3. **Integration**: Connect to existing Axum server setup
4. **Testing**: Add comprehensive test suite
5. **Documentation**: Update API documentation and deployment guides

---

**Implementation Status**: Ready for Phase 2 task generation
**Estimated Completion Time**: 2-4 hours for full implementation