# Data Model: Bills Export

**Phase**: 1 - Design & Contracts
**Date**: 2025-09-24
**Feature**: Export Bills Table

## Core Entities

### Bill (Existing Entity)
**Source**: Existing bills table with 14 fields
**Purpose**: Vietnamese invoice data structure for export

```rust
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct Bill {
    pub id: i32,
    pub form_no: Option<String>,           // Số tờ khai
    pub invoice_no: Option<String>,        // Số hóa đơn
    pub invoice_date: Option<chrono::NaiveDate>, // Ngày hóa đơn
    pub seller_name: Option<String>,       // Tên người bán
    pub seller_tax_code: Option<String>,   // Mã số thuế người bán
    pub seller_address: Option<String>,    // Địa chỉ người bán
    pub buyer_name: Option<String>,        // Tên người mua
    pub buyer_tax_code: Option<String>,    // Mã số thuế người mua
    pub buyer_address: Option<String>,     // Địa chỉ người mua
    pub total_amount: Option<rust_decimal::Decimal>, // Tổng tiền (NUMERIC 18,2)
    pub tax_amount: Option<rust_decimal::Decimal>,   // Tiền thuế (NUMERIC 18,2)
    pub discount_amount: Option<rust_decimal::Decimal>, // Tiền giảm giá (NUMERIC 18,2)
    pub vat_rate: Option<rust_decimal::Decimal>,     // Thuế suất VAT (NUMERIC 5,2)
    pub created_at: Option<chrono::NaiveDateTime>,   // Thời gian tạo
}
```

**Field Validation Rules**:
- All text fields support Vietnamese characters (UTF-8)
- Decimal fields use NUMERIC precision for financial accuracy
- Optional fields handle NULL database values gracefully
- Date fields use NaiveDate for timezone-neutral storage

### ExportFormat (New Enum)
**Purpose**: Define supported export file formats

```rust
#[derive(Debug, Clone, serde::Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Csv,
    Xlsx,
}

impl std::fmt::Display for ExportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportFormat::Csv => write!(f, "csv"),
            ExportFormat::Xlsx => write!(f, "xlsx"),
        }
    }
}
```

**Validation Rules**:
- Only "csv" and "xlsx" values accepted (case-insensitive)
- Invalid formats return HTTP 400 Bad Request

### ExportParams (New Request Structure)
**Purpose**: Query parameters for export endpoint

```rust
#[derive(Debug, serde::Deserialize)]
pub struct ExportParams {
    pub format: ExportFormat,
}
```

**Validation Rules**:
- format parameter is required
- Must be valid ExportFormat value

### ExportResponse (New Response Structure)
**Purpose**: File download response structure

```rust
pub struct ExportResponse {
    pub filename: String,
    pub content_type: &'static str,
    pub content: Vec<u8>,
}

impl ExportResponse {
    pub fn csv(content: Vec<u8>) -> Self {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        Self {
            filename: format!("bills_export_{}.csv", timestamp),
            content_type: "text/csv; charset=utf-8",
            content,
        }
    }

    pub fn xlsx(content: Vec<u8>) -> Self {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        Self {
            filename: format!("bills_export_{}.xlsx", timestamp),
            content_type: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            content,
        }
    }
}
```

## Export Field Mappings

### CSV Column Headers (Vietnamese + English)
All 14 bill fields will be exported with descriptive headers:

1. **ID** → "ID"
2. **form_no** → "Số tờ khai / Form No"
3. **invoice_no** → "Số hóa đơn / Invoice No"
4. **invoice_date** → "Ngày hóa đơn / Invoice Date"
5. **seller_name** → "Tên người bán / Seller Name"
6. **seller_tax_code** → "MST người bán / Seller Tax Code"
7. **seller_address** → "Địa chỉ người bán / Seller Address"
8. **buyer_name** → "Tên người mua / Buyer Name"
9. **buyer_tax_code** → "MST người mua / Buyer Tax Code"
10. **buyer_address** → "Địa chỉ người mua / Buyer Address"
11. **total_amount** → "Tổng tiền / Total Amount"
12. **tax_amount** → "Tiền thuế / Tax Amount"
13. **discount_amount** → "Tiền giảm giá / Discount Amount"
14. **vat_rate** → "Thuế suất VAT / VAT Rate (%)"
15. **created_at** → "Ngày tạo / Created At"

### XLSX Worksheet Structure
- **Worksheet name**: "Bills Export"
- **Header row**: Row 1 with bold formatting and background color
- **Data rows**: Starting from row 2
- **Column formatting**: Auto-fit width, proper alignment
- **Number formatting**: Decimal places for amounts, percentage for VAT rate

## Data Transformation Rules

### Text Fields
- **NULL values** → Empty string ""
- **Vietnamese characters** → Preserved in UTF-8 encoding
- **Special characters** → Properly escaped in CSV, native in XLSX

### Date Fields
- **NULL values** → Empty string ""
- **Format**: YYYY-MM-DD (ISO 8601)
- **Timezone**: None (NaiveDate used for storage)

### Decimal Fields
- **NULL values** → Empty string "" in CSV, 0.00 in XLSX
- **Precision**: 2 decimal places for amounts, 2 decimal places for VAT rate
- **Format**: No thousand separators in CSV, formatted in XLSX

### Boolean/Enum Fields
- Not applicable for current Bill structure

## Error Handling Data Flow

### Database Query Errors
```rust
pub enum ExportError {
    DatabaseError(sqlx::Error),
    SerializationError(String),
    IoError(std::io::Error),
}
```

### Empty Dataset Handling
- **CSV**: Headers-only file with UTF-8 BOM
- **XLSX**: Headers-only worksheet with formatting
- **HTTP Response**: 200 OK with empty file (not 204 No Content)

## Performance Considerations

### Large Dataset Strategy
- **Chunked processing**: 1000 records per database query
- **Streaming response**: Constant memory usage regardless of total size
- **Progress indication**: Not implemented in this phase (API only)

### Memory Optimization
- **Database queries**: Use `query_as!` with LIMIT/OFFSET
- **String handling**: Minimize allocations with streaming writers
- **Buffer management**: Reuse buffers where possible

## State Transitions

### Export Request Flow
1. **Request received** → Validate format parameter
2. **Valid format** → Query database for all bills
3. **Data retrieved** → Convert to requested format
4. **File generated** → Return with proper headers
5. **Error occurred** → Return HTTP error with message

**No persistent state changes** - read-only operation

## Relationships

### Current System Integration
- **Bills table** → Source of all export data
- **SQLx connection pool** → Database access layer
- **Axum handlers** → HTTP request/response handling
- **No user authentication** → Public endpoint as per requirements

### Future Extension Points
- **Filtering parameters** → Date range, status, amounts (not in current scope)
- **Pagination** → Large dataset handling (not in current scope)
- **Caching** → Generated file caching (not in current scope)
- **Audit logging** → Export activity tracking (not in current scope)