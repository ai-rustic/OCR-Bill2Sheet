use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use std::str::FromStr;

/// Structured bill data that maps exactly to the Bills table schema
///
/// This struct is designed for database operations and represents the exact
/// structure of the Bills table. All fields are optional to accommodate
/// partial data from OCR extraction processes.
///
/// Field mappings:
/// - PostgreSQL SERIAL → Rust i32 (for id)
/// - PostgreSQL TEXT → Rust Option<String> (for text fields)
/// - PostgreSQL DATE → Rust Option<chrono::NaiveDate> (for dates)
/// - PostgreSQL NUMERIC(18,2) → Rust Option<rust_decimal::Decimal> (for amounts)
/// - PostgreSQL NUMERIC(5,2) → Rust Option<rust_decimal::Decimal> (for rates)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BillData {
    /// Primary key identifier (SERIAL)
    pub id: i32,
    /// Form number/template identifier (e.g., "Mẫu 01-GTKT")
    pub form_no: Option<String>,
    /// Serial number of the invoice
    pub serial_no: Option<String>,
    /// Invoice number/identifier
    pub invoice_no: Option<String>,
    /// Date when the invoice was issued
    pub issued_date: Option<NaiveDate>,
    /// Name of the selling company
    pub seller_name: Option<String>,
    /// Tax identification code of the seller
    pub seller_tax_code: Option<String>,
    /// Name/description of the item or service
    pub item_name: Option<String>,
    /// Unit of measurement (e.g., "kg", "piece", "hour")
    pub unit: Option<String>,
    /// Quantity of items (NUMERIC(18,2) for precise calculations)
    pub quantity: Option<rust_decimal::Decimal>,
    /// Price per unit (NUMERIC(18,2) for precise financial calculations)
    pub unit_price: Option<rust_decimal::Decimal>,
    /// Total amount before tax (NUMERIC(18,2))
    pub total_amount: Option<rust_decimal::Decimal>,
    /// VAT/tax rate as percentage (NUMERIC(5,2))
    pub vat_rate: Option<rust_decimal::Decimal>,
    /// VAT/tax amount (NUMERIC(18,2))
    pub vat_amount: Option<rust_decimal::Decimal>,
}

/// Create structure for inserting new bill data (without id)
///
/// Used for INSERT operations where the database will auto-generate the id.
/// Maps to all fields except the SERIAL primary key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBillData {
    /// Form number/template identifier (e.g., "Mẫu 01-GTKT")
    pub form_no: Option<String>,
    /// Serial number of the invoice
    pub serial_no: Option<String>,
    /// Invoice number/identifier
    pub invoice_no: Option<String>,
    /// Date when the invoice was issued
    pub issued_date: Option<NaiveDate>,
    /// Name of the selling company
    pub seller_name: Option<String>,
    /// Tax identification code of the seller
    pub seller_tax_code: Option<String>,
    /// Name/description of the item or service
    pub item_name: Option<String>,
    /// Unit of measurement (e.g., "kg", "piece", "hour")
    pub unit: Option<String>,
    /// Quantity of items (NUMERIC(18,2) for precise calculations)
    pub quantity: Option<rust_decimal::Decimal>,
    /// Price per unit (NUMERIC(18,2) for precise financial calculations)
    pub unit_price: Option<rust_decimal::Decimal>,
    /// Total amount before tax (NUMERIC(18,2))
    pub total_amount: Option<rust_decimal::Decimal>,
    /// VAT/tax rate as percentage (NUMERIC(5,2))
    pub vat_rate: Option<rust_decimal::Decimal>,
    /// VAT/tax amount (NUMERIC(18,2))
    pub vat_amount: Option<rust_decimal::Decimal>,
}

impl BillData {
    /// Creates a new BillData with the given id and all other fields as None
    pub fn new(id: i32) -> Self {
        Self {
            id,
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

    /// Validates that financial data is consistent
    ///
    /// Checks that:
    /// - VAT rate is between 0 and 100%
    /// - Amounts are non-negative
    /// - VAT amount is consistent with total amount and VAT rate (if all present)
    pub fn validate_financial_data(&self) -> Result<(), String> {
        // Validate VAT rate
        if let Some(vat_rate) = &self.vat_rate {
            if *vat_rate < rust_decimal::Decimal::ZERO || *vat_rate > rust_decimal::Decimal::from(100) {
                return Err("VAT rate must be between 0 and 100".to_string());
            }
        }

        // Validate non-negative amounts
        if let Some(quantity) = &self.quantity {
            if *quantity < rust_decimal::Decimal::ZERO {
                return Err("Quantity cannot be negative".to_string());
            }
        }

        if let Some(unit_price) = &self.unit_price {
            if *unit_price < rust_decimal::Decimal::ZERO {
                return Err("Unit price cannot be negative".to_string());
            }
        }

        if let Some(total_amount) = &self.total_amount {
            if *total_amount < rust_decimal::Decimal::ZERO {
                return Err("Total amount cannot be negative".to_string());
            }
        }

        if let Some(vat_amount) = &self.vat_amount {
            if *vat_amount < rust_decimal::Decimal::ZERO {
                return Err("VAT amount cannot be negative".to_string());
            }
        }

        // Validate VAT calculation consistency (if all values are present)
        if let (Some(total_amount), Some(vat_rate), Some(vat_amount)) =
            (&self.total_amount, &self.vat_rate, &self.vat_amount) {
            let expected_vat = total_amount * vat_rate / rust_decimal::Decimal::from(100);
            let tolerance = rust_decimal::Decimal::from_str("0.01").unwrap(); // 1 cent tolerance

            if (expected_vat - vat_amount).abs() > tolerance {
                return Err(format!(
                    "VAT amount {} is inconsistent with total amount {} and VAT rate {}%",
                    vat_amount, total_amount, vat_rate
                ));
            }
        }

        Ok(())
    }

    /// Validates that the issued date is reasonable
    ///
    /// Checks that the date is not in the future and not older than 10 years
    pub fn validate_issued_date(&self) -> Result<(), String> {
        if let Some(issued_date) = &self.issued_date {
            let today = chrono::Utc::now().naive_utc().date();

            // Check if date is in the future
            if *issued_date > today {
                return Err("Issued date cannot be in the future".to_string());
            }

            // Check if date is too old (10 years)
            let ten_years_ago = today - chrono::Duration::days(10 * 365);
            if *issued_date < ten_years_ago {
                return Err("Issued date cannot be older than 10 years".to_string());
            }
        }

        Ok(())
    }

    /// Performs comprehensive validation of all data fields
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if let Err(e) = self.validate_financial_data() {
            errors.push(e);
        }

        if let Err(e) = self.validate_issued_date() {
            errors.push(e);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl CreateBillData {
    /// Creates a new CreateBillData with all fields as None
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

    /// Converts CreateBillData to BillData with the provided id
    pub fn with_id(self, id: i32) -> BillData {
        BillData {
            id,
            form_no: self.form_no,
            serial_no: self.serial_no,
            invoice_no: self.invoice_no,
            issued_date: self.issued_date,
            seller_name: self.seller_name,
            seller_tax_code: self.seller_tax_code,
            item_name: self.item_name,
            unit: self.unit,
            quantity: self.quantity,
            unit_price: self.unit_price,
            total_amount: self.total_amount,
            vat_rate: self.vat_rate,
            vat_amount: self.vat_amount,
        }
    }

    /// Validates the data before insertion
    pub fn validate(&self) -> Result<(), Vec<String>> {
        // Create a temporary BillData with id=0 for validation
        let temp_bill = self.clone().with_id(0);
        temp_bill.validate()
    }
}

impl Default for CreateBillData {
    fn default() -> Self {
        Self::new()
    }
}

impl From<BillData> for CreateBillData {
    fn from(bill_data: BillData) -> Self {
        Self {
            form_no: bill_data.form_no,
            serial_no: bill_data.serial_no,
            invoice_no: bill_data.invoice_no,
            issued_date: bill_data.issued_date,
            seller_name: bill_data.seller_name,
            seller_tax_code: bill_data.seller_tax_code,
            item_name: bill_data.item_name,
            unit: bill_data.unit,
            quantity: bill_data.quantity,
            unit_price: bill_data.unit_price,
            total_amount: bill_data.total_amount,
            vat_rate: bill_data.vat_rate,
            vat_amount: bill_data.vat_amount,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    #[test]
    fn test_bill_data_creation() {
        let bill_data = BillData::new(1);
        assert_eq!(bill_data.id, 1);
        assert_eq!(bill_data.form_no, None);
        assert_eq!(bill_data.total_amount, None);
    }

    #[test]
    fn test_create_bill_data_default() {
        let create_bill_data = CreateBillData::default();
        assert_eq!(create_bill_data.form_no, None);
        assert_eq!(create_bill_data.total_amount, None);
    }

    #[test]
    fn test_with_id_conversion() {
        let mut create_bill_data = CreateBillData::new();
        create_bill_data.invoice_no = Some("INV-001".to_string());
        create_bill_data.total_amount = Some(Decimal::from_str("1000.50").unwrap());

        let bill_data = create_bill_data.with_id(42);
        assert_eq!(bill_data.id, 42);
        assert_eq!(bill_data.invoice_no, Some("INV-001".to_string()));
        assert_eq!(bill_data.total_amount, Some(Decimal::from_str("1000.50").unwrap()));
    }

    #[test]
    fn test_financial_data_validation_success() {
        let mut bill_data = BillData::new(1);
        bill_data.total_amount = Some(Decimal::from_str("1000.00").unwrap());
        bill_data.vat_rate = Some(Decimal::from_str("10.00").unwrap());
        bill_data.vat_amount = Some(Decimal::from_str("100.00").unwrap());

        assert!(bill_data.validate_financial_data().is_ok());
    }

    #[test]
    fn test_financial_data_validation_negative_amount() {
        let mut bill_data = BillData::new(1);
        bill_data.total_amount = Some(Decimal::from_str("-100.00").unwrap());

        assert!(bill_data.validate_financial_data().is_err());
    }

    #[test]
    fn test_financial_data_validation_invalid_vat_rate() {
        let mut bill_data = BillData::new(1);
        bill_data.vat_rate = Some(Decimal::from_str("150.00").unwrap()); // Over 100%

        let result = bill_data.validate_financial_data();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("VAT rate must be between 0 and 100"));
    }

    #[test]
    fn test_financial_data_validation_inconsistent_vat() {
        let mut bill_data = BillData::new(1);
        bill_data.total_amount = Some(Decimal::from_str("1000.00").unwrap());
        bill_data.vat_rate = Some(Decimal::from_str("10.00").unwrap());
        bill_data.vat_amount = Some(Decimal::from_str("200.00").unwrap()); // Should be 100.00

        let result = bill_data.validate_financial_data();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("VAT amount"));
    }

    #[test]
    fn test_issued_date_validation_future_date() {
        let mut bill_data = BillData::new(1);
        let future_date = chrono::Utc::now().naive_utc().date() + chrono::Duration::days(30);
        bill_data.issued_date = Some(future_date);

        let result = bill_data.validate_issued_date();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("future"));
    }

    #[test]
    fn test_issued_date_validation_too_old() {
        let mut bill_data = BillData::new(1);
        let old_date = chrono::Utc::now().naive_utc().date() - chrono::Duration::days(11 * 365);
        bill_data.issued_date = Some(old_date);

        let result = bill_data.validate_issued_date();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("older than 10 years"));
    }

    #[test]
    fn test_comprehensive_validation() {
        let mut bill_data = BillData::new(1);
        bill_data.total_amount = Some(Decimal::from_str("-100.00").unwrap()); // Invalid
        let future_date = chrono::Utc::now().naive_utc().date() + chrono::Duration::days(30);
        bill_data.issued_date = Some(future_date); // Invalid

        let result = bill_data.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 2); // Should have both financial and date errors
    }

    #[test]
    fn test_from_bill_data_conversion() {
        let mut bill_data = BillData::new(1);
        bill_data.invoice_no = Some("INV-001".to_string());
        bill_data.total_amount = Some(Decimal::from_str("1000.50").unwrap());

        let create_bill_data: CreateBillData = bill_data.into();
        assert_eq!(create_bill_data.invoice_no, Some("INV-001".to_string()));
        assert_eq!(create_bill_data.total_amount, Some(Decimal::from_str("1000.50").unwrap()));
    }
}