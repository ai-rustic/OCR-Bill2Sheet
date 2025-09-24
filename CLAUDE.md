/ru# OCR_Bill2Sheet Development Guidelines

Auto-generated from all feature plans. Last updated: 2025-09-23

## Active Technologies
- Rust 1.75+ (edition = "2024") + Axum 0.8.4, SQLx 0.8.6 (postgres, runtime-tokio, macros), Tokio 1.47.1, dotenvy 0.15.7, csv 1.3, unicode-bom 2.0, rust_xlsxwriter 0.78, tokio-stream 0.1, futures-util 0.3 (HEAD)

## Project Structure
```
backend/
frontend/
tests/
```

## Commands
cargo test && cargo clippy

## Code Style
Rust 1.75+ (edition = "2024"): Follow standard conventions

## Recent Changes
- 008-export-bills-table: CSV/XLSX export API with UTF-8 BOM support, Vietnamese text compatibility, streaming for large datasets
- 002-create-a-bill: Bill table schema with 14 fields for Vietnamese invoices, SQLx migrations
- HEAD: Added Rust 1.75+ (edition = "2024") + Axum 0.8.4, SQLx 0.8.6 (postgres, runtime-tokio, macros), Tokio 1.47.1, dotenvy 0.15.7

<!-- MANUAL ADDITIONS START -->

## Constitutional Requirements

### Backend (Axum + SQLx)
- Use SQLx with compile-time query validation (`query!`, `query_as!`)
- Connection pooling required for PostgreSQL (bill_ocr database)
- Environment-based configuration via DATABASE_URL
- No ORM except SQLx - follow constitution strictly

### Frontend (Shadcn-First)
- ALL UI components must use Shadcn UI
- Verify components with `mcp` before implementation
- No custom UI components allowed
- Mobile-first responsive design

### Development Workflow
- **TDD is PROHIBITED** - Implementation-first approach
- Speed and prototype delivery prioritized
- No test scaffolding during development

## Current Feature: Bills Export API ✅ IN PROGRESS
- GET /api/bills/export?format={csv|xlsx} endpoint
- CSV with UTF-8 BOM for Vietnamese text compatibility
- XLSX with native UTF-8 encoding and formatting
- Streaming export for large datasets (constant memory usage)
- All 14 bill fields with bilingual column headers

## Previous Feature: Bill Table Schema ✅ COMPLETED
- Bill table with 14 fields for Vietnamese invoice data
- SQLx migrations for schema management
- NUMERIC(18,2) precision for financial calculations
- TEXT fields for Vietnamese text support
- Compile-time query validation with SQLx macros

## Migration Patterns and Usage Examples

### Migration Commands
```bash
# Create new migration
cd backend && sqlx migrate add create_bill_table

# Run migrations
cd backend && sqlx migrate run

# Rollback migration
cd backend && sqlx migrate revert

# Check migration status
cd backend && sqlx migrate info
```

### Bill Service Usage Examples
```rust
use backend::config::ConnectionPool;
use backend::services::bill_service::BillService;
use backend::models::{Bill, CreateBill};

// Initialize service
let pool = ConnectionPool::from_env().await?;
let bill_service = BillService::new(pool.pool().clone());

// Get all bills
let bills = bill_service.get_all_bills().await?;

// Create new bill
let new_bill = CreateBill {
    form_no: Some("Mẫu 01-GTKT".to_string()),
    invoice_no: Some("INV-2024-001".to_string()),
    // ... other fields
};
let created = bill_service.create_bill(new_bill).await?;

// Search bills
let results = bill_service.search_bills_by_invoice("2024").await?;
```

### Validated Field Mappings
- PostgreSQL SERIAL → Rust i32
- PostgreSQL TEXT → Rust Option<String>
- PostgreSQL DATE → Rust Option<chrono::NaiveDate>
- PostgreSQL NUMERIC(18,2) → Rust Option<rust_decimal::Decimal>
- PostgreSQL NUMERIC(5,2) → Rust Option<rust_decimal::Decimal>

### Migration Safety Procedures
1. Always test rollback before deploying: `sqlx migrate revert`
2. Verify table structure: `psql $DATABASE_URL -c "\d bills"`
3. Test data integrity with edge cases
4. Use compile-time validation with SQLx macros
5. Validate Vietnamese text and financial precision

## Export Service Patterns

### CSV Export with UTF-8 BOM
```rust
use csv::Writer;
use unicode_bom::Bom;

async fn export_bills_csv(bills: Vec<Bill>) -> Result<Vec<u8>, ExportError> {
    let mut buffer = Vec::new();
    buffer.write_all(Bom::Utf8.as_bytes())?; // UTF-8 BOM for Excel

    let mut writer = Writer::from_writer(&mut buffer);
    writer.write_record(&["ID", "Số tờ khai / Form No", "Số hóa đơn / Invoice No"])?;

    for bill in bills {
        writer.write_record(&[
            bill.id.to_string(),
            bill.form_no.unwrap_or_default(),
            bill.invoice_no.unwrap_or_default(),
        ])?;
    }

    writer.flush()?;
    Ok(buffer)
}
```

### XLSX Export with Formatting
```rust
use rust_xlsxwriter::{Workbook, Format};

async fn export_bills_xlsx(bills: Vec<Bill>) -> Result<Vec<u8>, ExportError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    let header_format = Format::new().set_bold().set_background_color("#D9EDF7");
    let headers = ["ID", "Số tờ khai / Form No", "Số hóa đơn / Invoice No"];

    for (col, header) in headers.iter().enumerate() {
        worksheet.write_string_with_format(0, col as u16, header, &header_format)?;
    }

    for (row, bill) in bills.iter().enumerate() {
        worksheet.write_number((row + 1) as u32, 0, bill.id as f64)?;
        worksheet.write_string((row + 1) as u32, 1, &bill.form_no.clone().unwrap_or_default())?;
    }

    worksheet.autofit();
    let mut buffer = Vec::new();
    workbook.save_to_buffer(&mut buffer)?;
    Ok(buffer)
}
```

### Streaming Export Handler
```rust
async fn export_bills_stream(
    State(pool): State<sqlx::PgPool>,
    Query(params): Query<ExportParams>,
) -> impl IntoResponse {
    let filename = format!("bills_export_{}.{}",
        chrono::Utc::now().format("%Y%m%d_%H%M%S"), params.format);

    let content_type = match params.format {
        ExportFormat::Csv => "text/csv; charset=utf-8",
        ExportFormat::Xlsx => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    };

    let headers = [
        (header::CONTENT_TYPE, content_type),
        (header::CONTENT_DISPOSITION, &format!("attachment; filename=\"{}\"", filename)),
        (header::CACHE_CONTROL, "no-cache, no-store, must-revalidate"),
    ];

    match export_bills(&pool, params.format).await {
        Ok(data) => (StatusCode::OK, headers, data).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, "Export failed").into_response(),
    }
}
```

<!-- MANUAL ADDITIONS END -->




