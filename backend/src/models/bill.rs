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

