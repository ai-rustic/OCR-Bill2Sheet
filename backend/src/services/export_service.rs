// Export service implementation
// This service will handle CSV/XLSX generation and file exports

use crate::models::export::{ExportError, ExportFormat, ExportResponse};
use crate::models::bill::Bill;
use csv::Writer;
use rust_xlsxwriter::{Format, Workbook};
use sqlx::PgPool;
use std::io::Write;
use chrono::NaiveDate;
use rust_decimal::Decimal;

pub struct ExportService {
    pool: PgPool,
}

impl ExportService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // Field transformation utilities for proper data formatting

    /// Transform Option<String> to safe string for export with Vietnamese text support
    fn transform_optional_string(value: &Option<String>) -> String {
        value.as_ref().map_or(String::new(), |s| {
            // Ensure proper Unicode normalization and sanitization for Vietnamese text
            Self::sanitize_vietnamese_text(s)
        })
    }

    /// Transform Option<NaiveDate> to formatted date string for export
    fn transform_optional_date(value: &Option<NaiveDate>) -> String {
        value.map_or(String::new(), |date| date.format("%Y-%m-%d").to_string())
    }

    /// Transform Option<Decimal> to formatted currency string for CSV export
    fn transform_optional_decimal_to_string(value: &Option<Decimal>) -> String {
        value.map_or(String::new(), |decimal| decimal.to_string())
    }

    /// Transform Option<Decimal> to formatted currency string with proper formatting for CSV
    fn transform_optional_currency_to_string(value: &Option<Decimal>) -> String {
        value.map_or(String::new(), |decimal| {
            // Format with 2 decimal places and thousands separator for readability
            format!("{:.2}", decimal)
        })
    }

    /// Transform Option<Decimal> VAT rate to percentage string for CSV export
    fn transform_optional_vat_rate_to_string(value: &Option<Decimal>) -> String {
        value.map_or(String::new(), |rate| format!("{}%", rate))
    }

    /// Transform Option<Decimal> to f64 for XLSX export with error handling
    fn transform_optional_decimal_to_f64(value: &Option<Decimal>) -> f64 {
        value.map_or(0.0, |decimal| {
            decimal.to_string().parse::<f64>().unwrap_or(0.0)
        })
    }

    /// Transform Option<Decimal> VAT rate to decimal percentage for XLSX export
    fn transform_optional_vat_rate_to_decimal(value: &Option<Decimal>) -> f64 {
        value.map_or(0.0, |rate| {
            let rate_f64 = rate.to_string().parse::<f64>().unwrap_or(0.0);
            rate_f64 / 100.0 // Convert percentage to decimal (e.g., 10% -> 0.10)
        })
    }

    /// Validate and sanitize Vietnamese text for export compatibility
    fn sanitize_vietnamese_text(text: &str) -> String {
        // Remove any potential BOM characters that might interfere
        text.trim_start_matches('\u{FEFF}')
            .trim()
            .to_string()
    }

    /// Get Vietnamese-only column headers for both CSV and XLSX exports
    fn get_export_headers() -> [&'static str; 14] {
        [
            "ID",
            "Số tờ khai",
            "Ký hiệu",
            "Số hóa đơn",
            "Ngày phát hành",
            "Tên người bán",
            "Mã số thuế",
            "Tên hàng hóa",
            "Đơn vị tính",
            "Số lượng",
            "Đơn giá",
            "Thành tiền",
            "Thuế suất VAT",
            "Tiền thuế VAT",
        ]
    }

    /// Transform a Bill record to a CSV row with proper field formatting
    fn transform_bill_to_csv_row(&self, bill: &Bill) -> Vec<String> {
        vec![
            bill.id.to_string(),
            Self::transform_optional_string(&bill.form_no),
            Self::transform_optional_string(&bill.serial_no),
            Self::transform_optional_string(&bill.invoice_no),
            Self::transform_optional_date(&bill.issued_date),
            Self::transform_optional_string(&bill.seller_name),
            Self::transform_optional_string(&bill.seller_tax_code),
            Self::transform_optional_string(&bill.item_name),
            Self::transform_optional_string(&bill.unit),
            Self::transform_optional_decimal_to_string(&bill.quantity),
            Self::transform_optional_currency_to_string(&bill.unit_price),
            Self::transform_optional_currency_to_string(&bill.total_amount),
            Self::transform_optional_vat_rate_to_string(&bill.vat_rate),
            Self::transform_optional_currency_to_string(&bill.vat_amount),
        ]
    }

    /// Write a Bill record to XLSX worksheet row with proper formatting
    fn write_bill_to_xlsx_row(
        &self,
        worksheet: &mut rust_xlsxwriter::Worksheet,
        row: u32,
        bill: &Bill,
        currency_format: &Format,
        percentage_format: &Format,
    ) -> Result<(), ExportError> {
        // ID column (numeric)
        worksheet.write_number(row, 0, bill.id as f64)?;

        // String columns with Vietnamese text support
        worksheet.write_string(row, 1, &Self::transform_optional_string(&bill.form_no))?;
        worksheet.write_string(row, 2, &Self::transform_optional_string(&bill.serial_no))?;
        worksheet.write_string(row, 3, &Self::transform_optional_string(&bill.invoice_no))?;

        // Date column
        worksheet.write_string(row, 4, &Self::transform_optional_date(&bill.issued_date))?;

        // More string columns
        worksheet.write_string(row, 5, &Self::transform_optional_string(&bill.seller_name))?;
        worksheet.write_string(row, 6, &Self::transform_optional_string(&bill.seller_tax_code))?;
        worksheet.write_string(row, 7, &Self::transform_optional_string(&bill.item_name))?;
        worksheet.write_string(row, 8, &Self::transform_optional_string(&bill.unit))?;

        // Numeric columns with proper formatting
        let quantity = Self::transform_optional_decimal_to_f64(&bill.quantity);
        if quantity != 0.0 {
            worksheet.write_number(row, 9, quantity)?;
        } else {
            worksheet.write_string(row, 9, "")?;
        }

        let unit_price = Self::transform_optional_decimal_to_f64(&bill.unit_price);
        if unit_price != 0.0 {
            worksheet.write_number_with_format(row, 10, unit_price, currency_format)?;
        } else {
            worksheet.write_string(row, 10, "")?;
        }

        let total_amount = Self::transform_optional_decimal_to_f64(&bill.total_amount);
        if total_amount != 0.0 {
            worksheet.write_number_with_format(row, 11, total_amount, currency_format)?;
        } else {
            worksheet.write_string(row, 11, "")?;
        }

        let vat_rate_decimal = Self::transform_optional_vat_rate_to_decimal(&bill.vat_rate);
        if vat_rate_decimal != 0.0 {
            worksheet.write_number_with_format(row, 12, vat_rate_decimal, percentage_format)?;
        } else {
            worksheet.write_string(row, 12, "")?;
        }

        let vat_amount = Self::transform_optional_decimal_to_f64(&bill.vat_amount);
        if vat_amount != 0.0 {
            worksheet.write_number_with_format(row, 13, vat_amount, currency_format)?;
        } else {
            worksheet.write_string(row, 13, "")?;
        }

        Ok(())
    }

    /// Generate CSV export with UTF-8 BOM for Vietnamese text compatibility
    pub async fn bills_to_csv(&self, bills: Vec<Bill>) -> Result<Vec<u8>, ExportError> {
        let mut buffer = Vec::new();

        // Add UTF-8 BOM for proper Excel rendering of Vietnamese text
        buffer.write_all(&[0xEF, 0xBB, 0xBF])?; // UTF-8 BOM bytes

        let mut writer = Writer::from_writer(&mut buffer);

        // Write Vietnamese headers using centralized utility
        let headers = Self::get_export_headers();
        writer.write_record(&headers)?;

        // Write bill data using transformation utilities
        for bill in bills {
            let row = self.transform_bill_to_csv_row(&bill);
            writer.write_record(&row)?;
        }

        writer.flush()?;
        drop(writer); // Explicitly drop the writer to release the borrow on buffer
        Ok(buffer)
    }

    /// Generate XLSX export with professional formatting for Vietnamese text compatibility
    pub async fn bills_to_xlsx(&self, bills: Vec<Bill>) -> Result<Vec<u8>, ExportError> {
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();

        // Set worksheet name for professional appearance
        worksheet.set_name("Bills Export")?;

        // Create professional header format with enhanced styling
        let header_format = Format::new()
            .set_bold()
            .set_background_color("#D9EDF7")
            .set_font_size(12)
            .set_font_color("#1F4E79")
            .set_align(rust_xlsxwriter::FormatAlign::Center)
            .set_border(rust_xlsxwriter::FormatBorder::Thin);

        // Create enhanced number formats for financial values with proper currency display
        let currency_format = Format::new()
            .set_num_format("#,##0.00")
            .set_align(rust_xlsxwriter::FormatAlign::Right);

        let percentage_format = Format::new()
            .set_num_format("0.00%")
            .set_align(rust_xlsxwriter::FormatAlign::Center);

        // Create alternating row format for better readability
        let alt_row_format = Format::new()
            .set_background_color("#F8F9FA");

        // Write Vietnamese headers using centralized utility
        let headers = Self::get_export_headers();
        for (col, header) in headers.iter().enumerate() {
            worksheet.write_string_with_format(0, col as u16, *header, &header_format)?;
        }

        // Freeze the header row for easier navigation in large datasets
        worksheet.set_freeze_panes(1, 0)?;

        // Write bill data with alternating row formatting
        for (row_idx, bill) in bills.iter().enumerate() {
            let row = (row_idx + 1) as u32;

            // Apply alternating row background for better readability
            if row_idx % 2 == 1 {
                // Apply light background to alternate rows
                for col in 0..14 {
                    worksheet.write_blank(row, col as u16, &alt_row_format)?;
                }
            }

            self.write_bill_to_xlsx_row(worksheet, row, bill, &currency_format, &percentage_format)?;
        }

        // Auto-fit columns for initial optimal sizing
        worksheet.autofit();

        // Set optimized column widths for Vietnamese text display with professional spacing
        worksheet.set_column_width(0, 8.0)?;   // ID - compact
        worksheet.set_column_width(1, 16.0)?;  // Form No - wider for Vietnamese text
        worksheet.set_column_width(2, 12.0)?;  // Serial No
        worksheet.set_column_width(3, 16.0)?;  // Invoice No - wider for longer numbers
        worksheet.set_column_width(4, 12.0)?;  // Issued Date
        worksheet.set_column_width(5, 25.0)?;  // Seller Name - wider for Vietnamese names
        worksheet.set_column_width(6, 14.0)?;  // Tax Code
        worksheet.set_column_width(7, 30.0)?;  // Item Name - widest for Vietnamese product names
        worksheet.set_column_width(8, 10.0)?;  // Unit
        worksheet.set_column_width(9, 10.0)?;  // Quantity
        worksheet.set_column_width(10, 12.0)?; // Unit Price
        worksheet.set_column_width(11, 14.0)?; // Total Amount
        worksheet.set_column_width(12, 10.0)?; // VAT Rate
        worksheet.set_column_width(13, 12.0)?; // VAT Amount

        // Set row height for better text visibility with Vietnamese characters
        worksheet.set_row_height(0, 20.0)?; // Header row slightly taller

        // Save to buffer
        let buffer = workbook.save_to_buffer()?;
        Ok(buffer)
    }

    /// Get all bills from database for export using SQLx query_as! macro
    pub async fn get_all_bills(&self) -> Result<Vec<Bill>, ExportError> {
        let bills = sqlx::query_as!(
            Bill,
            r#"
            SELECT
                id,
                form_no,
                serial_no,
                invoice_no,
                issued_date,
                seller_name,
                seller_tax_code,
                item_name,
                unit,
                quantity,
                unit_price,
                total_amount,
                vat_rate,
                vat_amount
            FROM bills
            ORDER BY id ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        if bills.is_empty() {
            return Err(ExportError::NoData);
        }

        Ok(bills)
    }

    /// Main export method that handles format routing and error handling
    pub async fn export_bills(&self, format: ExportFormat) -> Result<ExportResponse, ExportError> {
        // Get all bills from database
        let bills = self.get_all_bills().await?;

        // Generate export content based on format
        let content = match format {
            ExportFormat::Csv => self.bills_to_csv(bills).await?,
            ExportFormat::Xlsx => self.bills_to_xlsx(bills).await?,
        };

        // Create response with proper metadata
        let response = ExportResponse::new(&format, content);

        Ok(response)
    }
}