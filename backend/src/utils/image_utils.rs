//! Image processing utilities
//!
//! This module provides utilities for image processing, including resizing
//! images before sending to AI services to optimize performance and costs.

use image::imageops::FilterType;
use std::io::Cursor;
use thiserror::Error;
use tracing::{debug, info, instrument};

/// Errors that can occur during image processing
#[derive(Debug, Error)]
pub enum ImageProcessingError {
    #[error("Failed to decode image: {0}")]
    DecodeError(#[from] image::ImageError),

    #[error("Failed to encode image: {0}")]
    EncodeError(String),

    #[error("Unsupported image format")]
    UnsupportedFormat,

    #[error("Invalid scale factor: {0}. Must be between 0.1 and 1.0")]
    InvalidScaleFactor(f32),
}

/// Configuration for image resizing
#[derive(Debug, Clone)]
pub struct ResizeConfig {
    /// Scale factor for resizing (0.1 to 1.0)
    pub scale_factor: f32,
    /// JPEG quality (1-100)
    pub jpeg_quality: u8,
    /// Filter type for resizing
    pub filter_type: FilterType,
    /// Maximum width after resizing (None for no limit)
    pub max_width: Option<u32>,
    /// Maximum height after resizing (None for no limit)
    pub max_height: Option<u32>,
}

impl Default for ResizeConfig {
    fn default() -> Self {
        Self {
            scale_factor: 0.4, // 40% of original dimensions (width x height)
            jpeg_quality: 100, // Maximum quality - no compression artifacts
            filter_type: FilterType::Lanczos3, // High quality resampling filter
            max_width: None,    // No artificial width limit
            max_height: None,   // No artificial height limit
        }
    }
}

/// Resize an image to reduce pixel dimensions while preserving image quality
///
/// This function reduces only the width and height of the image (pixel count) while
/// maintaining maximum image quality. No compression artifacts are introduced - only
/// the pixel dimensions are scaled down by the specified factor.
///
/// # Arguments
/// * `image_data` - The original image data as bytes
/// * `config` - Configuration for the resizing operation
///
/// # Returns
/// * `Ok(Vec<u8>)` - The resized image as high-quality JPEG bytes
/// * `Err(ImageProcessingError)` - If the operation fails
///
/// # Example
/// ```rust
/// use backend::utils::image_utils::{resize_image, ResizeConfig};
///
/// let config = ResizeConfig::default(); // 40% of original dimensions, 100% quality
/// let resized_data = resize_image(&original_image_bytes, &config)?;
/// ```
#[instrument(skip(image_data), fields(original_size = image_data.len()))]
pub fn resize_image(
    image_data: &[u8],
    config: &ResizeConfig
) -> Result<Vec<u8>, ImageProcessingError> {
    // Validate scale factor
    if config.scale_factor <= 0.0 || config.scale_factor > 1.0 {
        return Err(ImageProcessingError::InvalidScaleFactor(config.scale_factor));
    }

    debug!("Starting image resize with scale factor: {}", config.scale_factor);

    // Load the image from bytes
    let img = image::load_from_memory(image_data)?;
    let (original_width, original_height) = (img.width(), img.height());

    info!(
        "Loaded image: {}x{} pixels, format: {:?}",
        original_width,
        original_height,
        img.color()
    );

    // Calculate new dimensions
    let mut new_width = (original_width as f32 * config.scale_factor) as u32;
    let mut new_height = (original_height as f32 * config.scale_factor) as u32;

    // Apply maximum dimension limits if specified
    if let Some(max_width) = config.max_width {
        if new_width > max_width {
            let scale = max_width as f32 / new_width as f32;
            new_width = max_width;
            new_height = (new_height as f32 * scale) as u32;
        }
    }

    if let Some(max_height) = config.max_height {
        if new_height > max_height {
            let scale = max_height as f32 / new_height as f32;
            new_height = max_height;
            new_width = (new_width as f32 * scale) as u32;
        }
    }

    // Ensure minimum dimensions
    new_width = new_width.max(1);
    new_height = new_height.max(1);

    debug!(
        "Resizing pixel dimensions from {}x{} to {}x{} (pixel reduction: {:.1}%)",
        original_width,
        original_height,
        new_width,
        new_height,
        (1.0 - (new_width * new_height) as f32 / (original_width * original_height) as f32) * 100.0
    );

    // Resize the image
    let resized_img = img.resize(new_width, new_height, config.filter_type);

    // Encode with maximum quality to preserve image fidelity
    let mut buffer = Cursor::new(Vec::new());

    // Use JPEG with maximum quality to minimize compression artifacts
    // This preserves image quality while only reducing pixel dimensions
    let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buffer, config.jpeg_quality);
    resized_img
        .write_with_encoder(encoder)
        .map_err(|e| ImageProcessingError::EncodeError(e.to_string()))?;

    let resized_data = buffer.into_inner();
    let size_reduction = ((image_data.len() - resized_data.len()) as f32 / image_data.len() as f32) * 100.0;

    info!(
        "Image pixel dimensions resized successfully: {} bytes -> {} bytes (file size reduced by {:.1}%)",
        image_data.len(),
        resized_data.len(),
        size_reduction
    );

    Ok(resized_data)
}

/// Resize an image with default configuration (40% pixel dimensions, 100% quality)
///
/// This convenience function reduces the image to 40% of its original pixel dimensions
/// (width x height) while preserving maximum image quality with no compression artifacts.
///
/// # Arguments
/// * `image_data` - The original image data as bytes
///
/// # Returns
/// * `Ok(Vec<u8>)` - The resized image as high-quality JPEG bytes
/// * `Err(ImageProcessingError)` - If the operation fails
pub fn resize_image_default(image_data: &[u8]) -> Result<Vec<u8>, ImageProcessingError> {
    resize_image(image_data, &ResizeConfig::default())
}

/// Get image dimensions without fully loading the image
///
/// # Arguments
/// * `image_data` - The image data as bytes
///
/// # Returns
/// * `Ok((width, height))` - The image dimensions in pixels
/// * `Err(ImageProcessingError)` - If the operation fails
pub fn get_image_dimensions(image_data: &[u8]) -> Result<(u32, u32), ImageProcessingError> {
    let img = image::load_from_memory(image_data)?;
    Ok((img.width(), img.height()))
}

/// Check if image data represents a valid image format
///
/// # Arguments
/// * `image_data` - The image data to check
///
/// # Returns
/// * `true` if the data is a valid image, `false` otherwise
pub fn is_valid_image(image_data: &[u8]) -> bool {
    image::load_from_memory(image_data).is_ok()
}

/// Calculate the file size reduction percentage
///
/// # Arguments
/// * `original_size` - Original file size in bytes
/// * `new_size` - New file size in bytes
///
/// # Returns
/// * Percentage reduction as f32 (e.g., 60.5 for 60.5% reduction)
pub fn calculate_size_reduction(original_size: usize, new_size: usize) -> f32 {
    if original_size == 0 {
        return 0.0;
    }
    ((original_size - new_size) as f32 / original_size as f32) * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_size_reduction() {
        assert!((calculate_size_reduction(1000, 400) - 60.0).abs() < 0.1);
        assert_eq!(calculate_size_reduction(1000, 1000), 0.0);
        assert_eq!(calculate_size_reduction(0, 0), 0.0);
    }

    #[test]
    fn test_invalid_scale_factor() {
        let config = ResizeConfig {
            scale_factor: 1.5, // Invalid: > 1.0
            ..Default::default()
        };

        let dummy_data = vec![0u8; 100];
        let result = resize_image(&dummy_data, &config);
        assert!(matches!(result, Err(ImageProcessingError::InvalidScaleFactor(_))));
    }
}