//! Gemini API request models
//!
//! This module defines the data structures for making requests to the Gemini AI API
//! for Vietnamese bill/invoice OCR processing.

use serde::Serialize;

/// Request payload for Gemini AI API
///
/// Contains the image data and prompt for structured bill data extraction.
#[derive(Debug, Serialize)]
pub struct GeminiRequest {
    /// Base64 encoded image data
    pub image_data: String,

    /// Structured output request prompt for Vietnamese bill extraction
    pub prompt: String,
}

impl GeminiRequest {
    /// Create a new Gemini request for bill extraction
    ///
    /// # Arguments
    /// * `image_data` - Base64 encoded image content
    /// * `prompt` - Structured prompt for bill data extraction
    ///
    /// # Returns
    /// A new GeminiRequest instance
    pub fn new(image_data: String, prompt: String) -> Self {
        Self {
            image_data,
            prompt,
        }
    }

    /// Create a default prompt for Vietnamese bill extraction
    ///
    /// Returns a structured prompt that instructs Gemini to extract specific
    /// bill fields in JSON format matching the database schema.
    pub fn default_bill_extraction_prompt() -> String {
        r#"Extract structured data from this Vietnamese invoice/bill image.
Return ONLY a JSON object with these exact fields (use null for missing values):

{
  "form_no": "Form number (Số/Mẫu hóa đơn)",
  "invoice_no": "Invoice number (Số hóa đơn)",
  "invoice_series": "Invoice series (Ký hiệu hóa đơn)",
  "invoice_date": "Invoice date in YYYY-MM-DD format",
  "seller_name": "Seller company name (Tên người bán)",
  "seller_tax_code": "Seller tax code (Mã số thuế người bán)",
  "seller_address": "Seller address (Địa chỉ người bán)",
  "buyer_name": "Buyer name (Tên người mua)",
  "buyer_tax_code": "Buyer tax code (Mã số thuế người mua)",
  "buyer_address": "Buyer address (Địa chỉ người mua)",
  "total_amount": "Total amount as string (Tổng tiền)",
  "tax_rate": "Tax rate percentage as string (Thuế suất %)",
  "tax_amount": "Tax amount as string (Tiền thuế)",
  "payment_method": "Payment method (Hình thức thanh toán)"
}

Extract text exactly as shown in the image. Use null for any field not clearly visible."#.to_string()
    }

    /// Create a GeminiRequest for bill extraction with default prompt
    ///
    /// # Arguments
    /// * `image_data` - Base64 encoded image content
    ///
    /// # Returns
    /// A GeminiRequest configured for Vietnamese bill extraction
    pub fn for_bill_extraction(image_data: String) -> Self {
        Self::new(image_data, Self::default_bill_extraction_prompt())
    }
}