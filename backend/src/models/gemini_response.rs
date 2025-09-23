//! Gemini API response models
//!
//! This module defines the data structures for receiving responses from the Gemini AI API
//! for Vietnamese bill/invoice OCR processing. The structure mirrors the bills database schema.

use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use rust_decimal::Decimal;

/// Response payload from Gemini AI API
///
/// Contains structured bill data extracted from Vietnamese invoices.
/// All fields are optional as extraction may not find all information.
/// Fields mirror the bills database schema exactly.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GeminiResponse {
    /// Form number (Số/Mẫu hóa đơn)
    pub form_no: Option<String>,

    /// Invoice number (Số hóa đơn)
    pub invoice_no: Option<String>,

    /// Invoice series (Ký hiệu hóa đơn)
    pub invoice_series: Option<String>,

    /// Invoice date in string format (will be parsed to NaiveDate)
    pub invoice_date: Option<String>,

    /// Seller company name (Tên người bán)
    pub seller_name: Option<String>,

    /// Seller tax code (Mã số thuế người bán)
    pub seller_tax_code: Option<String>,

    /// Seller address (Địa chỉ người bán)
    pub seller_address: Option<String>,

    /// Buyer name (Tên người mua)
    pub buyer_name: Option<String>,

    /// Buyer tax code (Mã số thuế người mua)
    pub buyer_tax_code: Option<String>,

    /// Buyer address (Địa chỉ người mua)
    pub buyer_address: Option<String>,

    /// Total amount as string (will be parsed to Decimal)
    pub total_amount: Option<String>,

    /// Tax rate percentage as string (will be parsed to Decimal)
    pub tax_rate: Option<String>,

    /// Tax amount as string (will be parsed to Decimal)
    pub tax_amount: Option<String>,

    /// Payment method (Hình thức thanh toán)
    pub payment_method: Option<String>,
}

impl GeminiResponse {
    /// Create a new empty GeminiResponse
    pub fn new() -> Self {
        Self {
            form_no: None,
            invoice_no: None,
            invoice_series: None,
            invoice_date: None,
            seller_name: None,
            seller_tax_code: None,
            seller_address: None,
            buyer_name: None,
            buyer_tax_code: None,
            buyer_address: None,
            total_amount: None,
            tax_rate: None,
            tax_amount: None,
            payment_method: None,
        }
    }

    /// Parse invoice date string to NaiveDate
    ///
    /// Attempts to parse the invoice_date field using common Vietnamese date formats.
    /// Returns None if parsing fails or field is empty.
    pub fn parse_invoice_date(&self) -> Option<NaiveDate> {
        self.invoice_date.as_ref()?.parse().ok()
    }

    /// Parse total amount string to Decimal
    ///
    /// Attempts to parse the total_amount field to Decimal for precise calculations.
    /// Handles Vietnamese number formatting (removes separators).
    pub fn parse_total_amount(&self) -> Option<Decimal> {
        self.total_amount.as_ref()?
            .replace(",", "")
            .replace(".", "")
            .replace(" ", "")
            .parse()
            .ok()
    }

    /// Parse tax rate string to Decimal
    ///
    /// Attempts to parse the tax_rate field to Decimal percentage.
    /// Handles Vietnamese percentage formatting.
    pub fn parse_tax_rate(&self) -> Option<Decimal> {
        self.tax_rate.as_ref()?
            .replace("%", "")
            .replace(",", ".")
            .replace(" ", "")
            .parse()
            .ok()
    }

    /// Parse tax amount string to Decimal
    ///
    /// Attempts to parse the tax_amount field to Decimal for precise calculations.
    /// Handles Vietnamese number formatting (removes separators).
    pub fn parse_tax_amount(&self) -> Option<Decimal> {
        self.tax_amount.as_ref()?
            .replace(",", "")
            .replace(".", "")
            .replace(" ", "")
            .parse()
            .ok()
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