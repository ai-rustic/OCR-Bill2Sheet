use serde::{Deserialize, Serialize};
use crate::models::image_info::ImageFileInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub success: bool,
    pub message: String,
    pub data: ValidationData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationData {
    pub accepted_images: Vec<ImageFileInfo>,
    pub total_count: usize,
    pub total_size_bytes: usize,
    pub processing_time_ms: u64,
}

impl ValidationResult {
    pub fn success(accepted_images: Vec<ImageFileInfo>, processing_time_ms: u64) -> Self {
        let total_count = accepted_images.len();
        let total_size_bytes = accepted_images.iter().map(|img| img.size_bytes).sum();

        Self {
            success: true,
            message: "Images validated successfully".to_string(),
            data: ValidationData {
                accepted_images,
                total_count,
                total_size_bytes,
                processing_time_ms,
            },
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            data: ValidationData {
                accepted_images: vec![],
                total_count: 0,
                total_size_bytes: 0,
                processing_time_ms: 0,
            },
        }
    }
}