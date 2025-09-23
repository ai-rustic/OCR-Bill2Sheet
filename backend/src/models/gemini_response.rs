//! Gemini API response models
//!
//! This module defines the data structures for receiving responses from the Gemini AI API
//! for Vietnamese bill/invoice OCR processing. The structure mirrors the bills database schema.

use serde::{Deserialize, Serialize, Deserializer};
use chrono::NaiveDate;
use rust_decimal::Decimal;

fn deserialize_null_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.filter(|s| s != "null" && !s.is_empty()))
}

fn deserialize_null_number<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<f64>::deserialize(deserializer)
}

/// Response payload from Gemini AI API
///
/// Contains structured bill data extracted from Vietnamese invoices.
/// All fields are optional as extraction may not find all information.
/// Fields mirror the bills database schema exactly and response schema.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GeminiResponse {
    /// Form number (Mẫu số hóa đơn)
    #[serde(deserialize_with = "deserialize_null_string")]
    pub form_no: Option<String>,

    /// Serial number (Ký hiệu hóa đơn)
    #[serde(deserialize_with = "deserialize_null_string")]
    pub serial_no: Option<String>,

    /// Invoice number (Số hóa đơn)
    #[serde(deserialize_with = "deserialize_null_string")]
    pub invoice_no: Option<String>,

    /// Invoice date in string format (will be parsed to NaiveDate)
    #[serde(deserialize_with = "deserialize_null_string")]
    pub issued_date: Option<String>,

    /// Seller company name (Tên người bán)
    #[serde(deserialize_with = "deserialize_null_string")]
    pub seller_name: Option<String>,

    /// Seller tax code (Mã số thuế người bán)
    #[serde(deserialize_with = "deserialize_null_string")]
    pub seller_tax_code: Option<String>,

    /// Item name (Tên hàng hóa/dịch vụ)
    #[serde(deserialize_with = "deserialize_null_string")]
    pub item_name: Option<String>,

    /// Unit (Đơn vị tính)
    #[serde(deserialize_with = "deserialize_null_string")]
    pub unit: Option<String>,

    /// Quantity (Số lượng)
    #[serde(deserialize_with = "deserialize_null_number")]
    pub quantity: Option<f64>,

    /// Unit price (Đơn giá)
    #[serde(deserialize_with = "deserialize_null_number")]
    pub unit_price: Option<f64>,

    /// Total amount (Thành tiền)
    #[serde(deserialize_with = "deserialize_null_number")]
    pub total_amount: Option<f64>,

    /// VAT rate percentage (Thuế suất VAT)
    #[serde(deserialize_with = "deserialize_null_number")]
    pub vat_rate: Option<f64>,

    /// VAT amount (Tiền thuế VAT)
    #[serde(deserialize_with = "deserialize_null_number")]
    pub vat_amount: Option<f64>,
}

impl GeminiResponse {
    /// Create a new empty GeminiResponse
    pub fn new() -> Self {
        Self {
            form_no: None,
            serial_no: None,
            invoice_no: None,
            issued_date: None,
            seller_name: None,
            seller_tax_code: None,
            item_name: None,
            unit: None,
            quantity: None,
            unit_price: None,
            total_amount: None,
            vat_rate: None,
            vat_amount: None,
        }
    }

    /// Parse issued date string to NaiveDate
    ///
    /// Attempts to parse the issued_date field using common date formats.
    /// Returns None if parsing fails or field is empty.
    pub fn parse_issued_date(&self) -> Option<NaiveDate> {
        self.issued_date.as_ref()?.parse().ok()
    }

    /// Convert total amount f64 to Decimal
    ///
    /// Converts the total_amount field to Decimal for precise calculations.
    pub fn get_total_amount_decimal(&self) -> Option<Decimal> {
        self.total_amount.map(|amount| Decimal::try_from(amount).unwrap_or_default())
    }

    /// Convert VAT rate f64 to Decimal
    ///
    /// Converts the vat_rate field to Decimal percentage.
    pub fn get_vat_rate_decimal(&self) -> Option<Decimal> {
        self.vat_rate.map(|rate| Decimal::try_from(rate).unwrap_or_default())
    }

    /// Convert VAT amount f64 to Decimal
    ///
    /// Converts the vat_amount field to Decimal for precise calculations.
    pub fn get_vat_amount_decimal(&self) -> Option<Decimal> {
        self.vat_amount.map(|amount| Decimal::try_from(amount).unwrap_or_default())
    }

    /// Convert quantity f64 to Decimal
    ///
    /// Converts the quantity field to Decimal for precise calculations.
    pub fn get_quantity_decimal(&self) -> Option<Decimal> {
        self.quantity.map(|qty| Decimal::try_from(qty).unwrap_or_default())
    }

    /// Convert unit price f64 to Decimal
    ///
    /// Converts the unit_price field to Decimal for precise calculations.
    pub fn get_unit_price_decimal(&self) -> Option<Decimal> {
        self.unit_price.map(|price| Decimal::try_from(price).unwrap_or_default())
    }

    /// Validate that at least some essential fields are present
    ///
    /// Returns true if the response contains at least one of the core invoice fields.
    pub fn has_essential_data(&self) -> bool {
        self.invoice_no.is_some()
            || self.seller_name.is_some()
            || self.total_amount.is_some()
    }
}

impl Default for GeminiResponse {
    fn default() -> Self {
        Self::new()
    }
}