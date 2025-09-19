use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Bill {
    pub id: i32,
    pub form_no: Option<String>,
    pub serial_no: Option<String>,
    pub invoice_no: Option<String>,
    pub issued_date: Option<NaiveDate>,
    pub seller_name: Option<String>,
    pub seller_tax_code: Option<String>,
    pub item_name: Option<String>,
    pub unit: Option<String>,
    pub quantity: Option<rust_decimal::Decimal>,
    pub unit_price: Option<rust_decimal::Decimal>,
    pub total_amount: Option<rust_decimal::Decimal>,
    pub vat_rate: Option<rust_decimal::Decimal>,
    pub vat_amount: Option<rust_decimal::Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBill {
    pub form_no: Option<String>,
    pub serial_no: Option<String>,
    pub invoice_no: Option<String>,
    pub issued_date: Option<NaiveDate>,
    pub seller_name: Option<String>,
    pub seller_tax_code: Option<String>,
    pub item_name: Option<String>,
    pub unit: Option<String>,
    pub quantity: Option<rust_decimal::Decimal>,
    pub unit_price: Option<rust_decimal::Decimal>,
    pub total_amount: Option<rust_decimal::Decimal>,
    pub vat_rate: Option<rust_decimal::Decimal>,
    pub vat_amount: Option<rust_decimal::Decimal>,
}

impl Bill {
    /// Create a new Bill from CreateBill data
    pub fn from_create_bill(create_bill: CreateBill, id: i32) -> Self {
        Self {
            id,
            form_no: create_bill.form_no,
            serial_no: create_bill.serial_no,
            invoice_no: create_bill.invoice_no,
            issued_date: create_bill.issued_date,
            seller_name: create_bill.seller_name,
            seller_tax_code: create_bill.seller_tax_code,
            item_name: create_bill.item_name,
            unit: create_bill.unit,
            quantity: create_bill.quantity,
            unit_price: create_bill.unit_price,
            total_amount: create_bill.total_amount,
            vat_rate: create_bill.vat_rate,
            vat_amount: create_bill.vat_amount,
        }
    }
}