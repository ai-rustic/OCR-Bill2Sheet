use sqlx::PgPool;
use crate::models::{Bill, CreateBill};
use crate::api::ApiError;

pub struct BillService {
    pool: PgPool,
}

impl BillService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get all bills from the database
    /// Uses compile-time query validation with sqlx::query_as!
    pub async fn get_all_bills(&self) -> Result<Vec<Bill>, ApiError> {
        let bills = sqlx::query_as!(
            Bill,
            r#"
            SELECT id, form_no, serial_no, invoice_no, issued_date,
                   seller_name, seller_tax_code, item_name, unit,
                   quantity, unit_price, total_amount, vat_rate, vat_amount
            FROM bills
            ORDER BY id ASC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(format!("Database error: {}", e)))?;

        Ok(bills)
    }

    /// Get a bill by ID
    /// Uses compile-time query validation with sqlx::query_as!
    pub async fn get_bill_by_id(&self, id: i32) -> Result<Option<Bill>, ApiError> {
        let bill = sqlx::query_as!(
            Bill,
            r#"
            SELECT id, form_no, serial_no, invoice_no, issued_date,
                   seller_name, seller_tax_code, item_name, unit,
                   quantity, unit_price, total_amount, vat_rate, vat_amount
            FROM bills
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(format!("Database error: {}", e)))?;

        Ok(bill)
    }

    /// Create a new bill
    /// Uses compile-time query validation with sqlx::query!
    pub async fn create_bill(&self, create_bill: CreateBill) -> Result<Bill, ApiError> {
        let bill = sqlx::query_as!(
            Bill,
            r#"
            INSERT INTO bills (
                form_no, serial_no, invoice_no, issued_date,
                seller_name, seller_tax_code, item_name, unit,
                quantity, unit_price, total_amount, vat_rate, vat_amount
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING id, form_no, serial_no, invoice_no, issued_date,
                      seller_name, seller_tax_code, item_name, unit,
                      quantity, unit_price, total_amount, vat_rate, vat_amount
            "#,
            create_bill.form_no,
            create_bill.serial_no,
            create_bill.invoice_no,
            create_bill.issued_date,
            create_bill.seller_name,
            create_bill.seller_tax_code,
            create_bill.item_name,
            create_bill.unit,
            create_bill.quantity,
            create_bill.unit_price,
            create_bill.total_amount,
            create_bill.vat_rate,
            create_bill.vat_amount
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(format!("Database error: {}", e)))?;

        Ok(bill)
    }

    /// Update a bill by ID
    /// Uses compile-time query validation with sqlx::query!
    pub async fn update_bill(&self, id: i32, update_bill: CreateBill) -> Result<Option<Bill>, ApiError> {
        let bill = sqlx::query_as!(
            Bill,
            r#"
            UPDATE bills SET
                form_no = $2, serial_no = $3, invoice_no = $4, issued_date = $5,
                seller_name = $6, seller_tax_code = $7, item_name = $8, unit = $9,
                quantity = $10, unit_price = $11, total_amount = $12, vat_rate = $13, vat_amount = $14
            WHERE id = $1
            RETURNING id, form_no, serial_no, invoice_no, issued_date,
                      seller_name, seller_tax_code, item_name, unit,
                      quantity, unit_price, total_amount, vat_rate, vat_amount
            "#,
            id,
            update_bill.form_no,
            update_bill.serial_no,
            update_bill.invoice_no,
            update_bill.issued_date,
            update_bill.seller_name,
            update_bill.seller_tax_code,
            update_bill.item_name,
            update_bill.unit,
            update_bill.quantity,
            update_bill.unit_price,
            update_bill.total_amount,
            update_bill.vat_rate,
            update_bill.vat_amount
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(format!("Database error: {}", e)))?;

        Ok(bill)
    }

    /// Delete a bill by ID
    /// Uses compile-time query validation with sqlx::query!
    pub async fn delete_bill(&self, id: i32) -> Result<bool, ApiError> {
        let result = sqlx::query!(
            "DELETE FROM bills WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(format!("Database error: {}", e)))?;

        Ok(result.rows_affected() > 0)
    }

    /// Search bills by invoice number
    /// Demonstrates pattern matching with LIKE
    pub async fn search_bills_by_invoice(&self, pattern: &str) -> Result<Vec<Bill>, ApiError> {
        let bills = sqlx::query_as!(
            Bill,
            r#"
            SELECT id, form_no, serial_no, invoice_no, issued_date,
                   seller_name, seller_tax_code, item_name, unit,
                   quantity, unit_price, total_amount, vat_rate, vat_amount
            FROM bills
            WHERE invoice_no ILIKE $1
            ORDER BY issued_date DESC
            "#,
            format!("%{}%", pattern)
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(format!("Database error: {}", e)))?;

        Ok(bills)
    }

    /// Get bills count - demonstrates simple aggregate query
    pub async fn get_bills_count(&self) -> Result<i64, ApiError> {
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM bills"
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(format!("Database error: {}", e)))?;

        Ok(count.count.unwrap_or(0))
    }
}