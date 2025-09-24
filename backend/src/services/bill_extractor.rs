//! Bill Data Extractor Service
//!
//! This module provides functionality to convert Gemini AI API responses
//! into database-compatible bill structures for Vietnamese invoice processing.

use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::models::{CreateBill, GeminiResponse};

/// Service for extracting and converting bill data from Gemini AI responses
///
/// Handles the conversion between Gemini API response format and database schema,
/// including Vietnamese number formatting, date parsing, and field mapping.
pub struct BillDataExtractor;

/// Error types for bill data extraction
#[derive(Debug, thiserror::Error)]
pub enum ExtractionError {
    #[error("Date parsing error: {0}")]
    DateParseError(String),

    #[error("Number parsing error: {0}")]
    NumberParseError(String),

    #[error("Missing essential data: {0}")]
    MissingEssentialData(String),

    #[error("Invalid data format: {0}")]
    InvalidFormat(String),
}

impl BillDataExtractor {
    /// Create a new BillDataExtractor instance
    pub fn new() -> Self {
        Self
    }

    /// Extract and convert GeminiResponse to CreateBill
    ///
    /// Performs data transformation including:
    /// - Vietnamese date format parsing
    /// - Vietnamese number format normalization
    /// - Field mapping between API response and database schema
    /// - Data validation and error handling
    pub fn extract_bill_data(
        &self,
        gemini_response: &GeminiResponse,
    ) -> Result<CreateBill, ExtractionError> {
        // Validate that we have at least some essential data
        if !gemini_response.has_essential_data() {
            return Err(ExtractionError::MissingEssentialData(
                "No essential bill data found in Gemini response".to_string(),
            ));
        }

        // Parse issued date if present
        let issued_date = if let Some(date_str) = &gemini_response.issued_date {
            Some(self.parse_vietnamese_date(date_str)?)
        } else {
            None
        };

        // Convert f64 amounts to Decimal
        let total_amount = gemini_response.get_total_amount_decimal();
        let vat_rate = gemini_response.get_vat_rate_decimal();
        let vat_amount = gemini_response.get_vat_amount_decimal();
        let quantity = gemini_response.get_quantity_decimal();
        let unit_price = gemini_response.get_unit_price_decimal();

        // Create the bill structure
        Ok(CreateBill {
            form_no: gemini_response.form_no.clone(),
            serial_no: gemini_response.serial_no.clone(),
            invoice_no: gemini_response.invoice_no.clone(),
            issued_date,
            seller_name: gemini_response.seller_name.clone(),
            seller_tax_code: gemini_response.seller_tax_code.clone(),
            item_name: gemini_response.item_name.clone(),
            unit: gemini_response.unit.clone(),
            quantity,
            unit_price,
            total_amount,
            vat_rate,
            vat_amount,
        })
    }

    /// Parse Vietnamese date formats
    ///
    /// Supports common Vietnamese date formats:
    /// - DD/MM/YYYY
    /// - DD-MM-YYYY
    /// - DD.MM.YYYY
    /// - ISO format YYYY-MM-DD
    fn parse_vietnamese_date(&self, date_str: &str) -> Result<NaiveDate, ExtractionError> {
        let cleaned = date_str.trim();

        // Try different Vietnamese date formats
        let formats = [
            "%d/%m/%Y", // 31/12/2024
            "%d-%m-%Y", // 31-12-2024
            "%d.%m.%Y", // 31.12.2024
            "%Y-%m-%d", // 2024-12-31 (ISO format)
            "%d/%m/%y", // 31/12/24
            "%d-%m-%y", // 31-12-24
        ];

        for format in &formats {
            if let Ok(date) = NaiveDate::parse_from_str(cleaned, format) {
                return Ok(date);
            }
        }

        Err(ExtractionError::DateParseError(format!(
            "Unable to parse date: {}",
            date_str
        )))
    }

    /// Parse Vietnamese amount formats
    ///
    /// Handles Vietnamese number formatting:
    /// - Removes thousands separators (. or ,)
    /// - Handles decimal places with , or .
    /// - Removes currency symbols and spaces
    fn parse_vietnamese_amount(&self, amount_str: &str) -> Result<Decimal, ExtractionError> {
        let cleaned = amount_str
            .trim()
            .replace("₫", "") // Remove Vietnamese dong symbol
            .replace("VND", "") // Remove VND currency code
            .replace("đ", "") // Remove Vietnamese dong symbol variant
            .replace(" ", ""); // Remove spaces

        // Handle Vietnamese number formatting
        // Vietnamese often uses . as thousands separator and , as decimal
        // But also . as decimal in some cases
        let normalized = if cleaned.matches('.').count() > 1
            || (cleaned.contains('.') && cleaned.contains(','))
        {
            // Multiple dots or both . and , means . is thousands separator
            let parts: Vec<&str> = cleaned.split(',').collect();
            if parts.len() == 2 {
                format!("{}.{}", parts[0].replace(".", ""), parts[1])
            } else {
                cleaned.replace(".", "")
            }
        } else if cleaned.contains(',') && !cleaned.contains('.') {
            // Only comma, might be decimal separator
            cleaned.replace(",", ".")
        } else {
            cleaned
        };

        Decimal::from_str(&normalized).map_err(|e| {
            ExtractionError::NumberParseError(format!(
                "Unable to parse amount '{}': {}",
                amount_str, e
            ))
        })
    }

    /// Parse Vietnamese percentage formats
    ///
    /// Handles percentage values:
    /// - Removes % symbol
    /// - Handles decimal separators (, or .)
    /// - Converts to decimal representation (10% -> 10.00)
    fn parse_vietnamese_percentage(
        &self,
        percentage_str: &str,
    ) -> Result<Decimal, ExtractionError> {
        let cleaned = percentage_str
            .trim()
            .replace("%", "")
            .replace(" ", "")
            .replace(",", "."); // Normalize decimal separator

        Decimal::from_str(&cleaned).map_err(|e| {
            ExtractionError::NumberParseError(format!(
                "Unable to parse percentage '{}': {}",
                percentage_str, e
            ))
        })
    }

    /// Validate extracted bill data
    ///
    /// Performs additional validation to ensure data quality:
    /// - Date ranges are reasonable
    /// - Financial amounts are positive
    /// - Required fields are present
    pub fn validate_extracted_data(&self, bill: &CreateBill) -> Result<(), ExtractionError> {
        // Validate date is not in the future
        if let Some(date) = &bill.issued_date {
            let today = chrono::Utc::now().date_naive();
            if date > &today {
                return Err(ExtractionError::InvalidFormat(
                    "Invoice date cannot be in the future".to_string(),
                ));
            }
        }

        // Validate amounts are positive
        if let Some(amount) = &bill.total_amount {
            if amount.is_sign_negative() {
                return Err(ExtractionError::InvalidFormat(
                    "Total amount cannot be negative".to_string(),
                ));
            }
        }

        if let Some(amount) = &bill.vat_amount {
            if amount.is_sign_negative() {
                return Err(ExtractionError::InvalidFormat(
                    "VAT amount cannot be negative".to_string(),
                ));
            }
        }

        // Validate VAT rate is reasonable (0-100%)
        if let Some(rate) = &bill.vat_rate {
            if rate.is_sign_negative() || *rate > Decimal::from(100) {
                return Err(ExtractionError::InvalidFormat(
                    "VAT rate must be between 0 and 100%".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Extract bill data with validation
    ///
    /// Combines extraction and validation in a single operation
    pub fn extract_and_validate(
        &self,
        gemini_response: &GeminiResponse,
    ) -> Result<CreateBill, ExtractionError> {
        let bill = self.extract_bill_data(gemini_response)?;
        self.validate_extracted_data(&bill)?;
        Ok(bill)
    }
}

impl Default for BillDataExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_vietnamese_date() {
        let extractor = BillDataExtractor::new();

        // Test various Vietnamese date formats
        assert!(extractor.parse_vietnamese_date("31/12/2024").is_ok());
        assert!(extractor.parse_vietnamese_date("31-12-2024").is_ok());
        assert!(extractor.parse_vietnamese_date("31.12.2024").is_ok());
        assert!(extractor.parse_vietnamese_date("2024-12-31").is_ok());

        // Test invalid date
        assert!(extractor.parse_vietnamese_date("invalid").is_err());
    }

    #[test]
    fn test_parse_vietnamese_amount() {
        let extractor = BillDataExtractor::new();

        // Test Vietnamese number formats
        assert_eq!(
            extractor.parse_vietnamese_amount("1.000.000").unwrap(),
            Decimal::from(1000000)
        );
        assert_eq!(
            extractor.parse_vietnamese_amount("1.000.000,50").unwrap(),
            Decimal::new(100000050, 2)
        );
        assert_eq!(
            extractor.parse_vietnamese_amount("1000000 VND").unwrap(),
            Decimal::from(1000000)
        );
    }

    #[test]
    fn test_parse_vietnamese_percentage() {
        let extractor = BillDataExtractor::new();

        assert_eq!(
            extractor.parse_vietnamese_percentage("10%").unwrap(),
            Decimal::from(10)
        );
        assert_eq!(
            extractor.parse_vietnamese_percentage("8,5%").unwrap(),
            Decimal::new(85, 1)
        );
    }

    #[test]
    fn test_extract_bill_data() {
        let extractor = BillDataExtractor::new();
        let gemini_response = GeminiResponse {
            form_no: Some("01-GTKT".to_string()),
            serial_no: Some("AA/24E".to_string()),
            invoice_no: Some("00000001".to_string()),
            issued_date: Some("31/12/2024".to_string()),
            seller_name: Some("CÔNG TY ABC".to_string()),
            seller_tax_code: Some("0123456789".to_string()),
            item_name: Some("Hàng hóa".to_string()),
            unit: Some("Chiếc".to_string()),
            quantity: Some(1.0),
            unit_price: Some(909090.91),
            total_amount: Some(1000000.0),
            vat_rate: Some(10.0),
            vat_amount: Some(100000.0),
        };

        let result = extractor.extract_bill_data(&gemini_response);
        assert!(result.is_ok());

        let bill = result.unwrap();
        assert_eq!(bill.form_no, Some("01-GTKT".to_string()));
        assert_eq!(bill.invoice_no, Some("00000001".to_string()));
        assert_eq!(bill.serial_no, Some("AA/24E".to_string()));
        assert_eq!(bill.seller_name, Some("CÔNG TY ABC".to_string()));
        assert_eq!(bill.total_amount, Some(Decimal::from(1000000)));
        assert_eq!(bill.vat_rate, Some(Decimal::from(10)));
        assert_eq!(bill.vat_amount, Some(Decimal::from(100000)));
        assert_eq!(bill.item_name, Some("Hàng hóa".to_string()));
        assert_eq!(bill.unit, Some("Chiếc".to_string()));
        assert_eq!(bill.quantity, Some(Decimal::from(1)));
        assert_eq!(bill.unit_price, Some(Decimal::new(90909091, 2)));
    }
}
