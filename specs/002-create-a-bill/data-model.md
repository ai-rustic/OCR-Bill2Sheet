# Data Model: Bill Table

**Date**: 2025-09-19
**Status**: Phase 1 Design Complete

## Entity Definition

### Bill Entity

**Purpose**: Store comprehensive Vietnamese invoice/bill data from OCR processing
**Table Name**: `bills`
**Primary Key**: `id` (auto-incrementing)

#### Field Specifications

| Field | Type | Constraints | Description | Vietnamese Context |
|-------|------|-------------|-------------|-------------------|
| `id` | SERIAL | PRIMARY KEY, NOT NULL | Auto-incrementing unique identifier | (1) Số thứ tự |
| `form_no` | TEXT | - | Form number from invoice template | (2) Số mẫu |
| `serial_no` | TEXT | - | Serial symbol of invoice series | (3) Ký hiệu |
| `invoice_no` | TEXT | - | Invoice number within series | (4) Số hóa đơn |
| `issued_date` | DATE | - | Date when invoice was issued | (5) Ngày lập |
| `seller_name` | TEXT | - | Name of selling entity | (6) Tên người bán |
| `seller_tax_code` | TEXT | - | Tax identification code of seller | (7) Mã số thuế người bán |
| `item_name` | TEXT | - | Name of goods or service | (8) Tên hàng hóa/dịch vụ |
| `unit` | TEXT | - | Unit of measurement | (9) Đơn vị tính |
| `quantity` | NUMERIC(18,2) | - | Quantity of items | (10) Số lượng |
| `unit_price` | NUMERIC(18,2) | - | Price per unit | (11) Đơn giá |
| `total_amount` | NUMERIC(18,2) | - | Total value excluding VAT | (12) Thành tiền |
| `vat_rate` | NUMERIC(5,2) | - | VAT rate as percentage | (13) Thuế suất VAT (%) |
| `vat_amount` | NUMERIC(18,2) | - | VAT amount calculated | (14) Tiền thuế VAT |

#### Data Type Rationale

**TEXT Fields**:
- Support Unicode Vietnamese characters
- No artificial length limitations
- Flexible for varying invoice formats

**DATE Field**:
- Standard SQL date type for issued_date
- No time component needed for invoice dates
- Supports date range queries for reporting

**NUMERIC Fields**:
- `NUMERIC(18,2)`: Financial precision for monetary values
  - 16 digits before decimal point
  - 2 digits after decimal point
  - Handles amounts up to 999,999,999,999,999.99 VND
- `NUMERIC(5,2)`: Percentage precision for VAT rates
  - 3 digits before decimal point
  - 2 digits after decimal point
  - Handles rates up to 999.99%

## Relationships

### Current Scope
- **Standalone Entity**: No foreign key relationships in this phase
- **Self-contained**: All invoice data within single record

### Future Considerations
- Potential normalization: Extract sellers to separate table
- Item catalog: Standardized product/service references
- Tax rate table: Centralized VAT rate management

## Validation Rules

### Business Logic Constraints
- `quantity` should be positive for normal transactions
- `unit_price` should be non-negative
- `total_amount` = `quantity` × `unit_price` (business rule)
- `vat_amount` = `total_amount` × (`vat_rate` / 100) (business rule)

### Data Integrity
- PostgreSQL handles numeric precision automatically
- UTF-8 encoding for Vietnamese text support
- Date validation through PostgreSQL DATE type

## Indexing Strategy

### Primary Access Patterns
1. **Lookup by ID**: Primary key index (automatic)
2. **Search by seller**: Consider index on `seller_tax_code`
3. **Date range queries**: Consider index on `issued_date`
4. **Invoice identification**: Consider composite index on (`seller_tax_code`, `invoice_no`)

### Recommended Indexes (Future)
```sql
-- For seller-based queries
CREATE INDEX idx_bills_seller_tax_code ON bills(seller_tax_code);

-- For date range reporting
CREATE INDEX idx_bills_issued_date ON bills(issued_date);

-- For unique invoice lookup
CREATE INDEX idx_bills_seller_invoice ON bills(seller_tax_code, invoice_no);
```

## Migration Strategy

### Forward Migration
```sql
CREATE TABLE bills (
    id SERIAL PRIMARY KEY,
    form_no TEXT,
    serial_no TEXT,
    invoice_no TEXT,
    issued_date DATE,
    seller_name TEXT,
    seller_tax_code TEXT,
    item_name TEXT,
    unit TEXT,
    quantity NUMERIC(18,2),
    unit_price NUMERIC(18,2),
    total_amount NUMERIC(18,2),
    vat_rate NUMERIC(5,2),
    vat_amount NUMERIC(18,2)
);
```

### Rollback Migration
```sql
DROP TABLE IF EXISTS bills;
```

## SQLx Integration

### Rust Struct Mapping
```rust
#[derive(sqlx::FromRow)]
pub struct Bill {
    pub id: i32,
    pub form_no: Option<String>,
    pub serial_no: Option<String>,
    pub invoice_no: Option<String>,
    pub issued_date: Option<chrono::NaiveDate>,
    pub seller_name: Option<String>,
    pub seller_tax_code: Option<String>,
    pub item_name: Option<String>,
    pub unit: Option<String>,
    pub quantity: Option<sqlx::types::Decimal>,
    pub unit_price: Option<sqlx::types::Decimal>,
    pub total_amount: Option<sqlx::types::Decimal>,
    pub vat_rate: Option<sqlx::types::Decimal>,
    pub vat_amount: Option<sqlx::types::Decimal>,
}
```

### Query Examples
```rust
// Insert new bill
let bill = sqlx::query!(
    "INSERT INTO bills (form_no, serial_no, invoice_no, issued_date, seller_name, seller_tax_code, item_name, unit, quantity, unit_price, total_amount, vat_rate, vat_amount)
     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
     RETURNING id",
    form_no, serial_no, invoice_no, issued_date, seller_name, seller_tax_code,
    item_name, unit, quantity, unit_price, total_amount, vat_rate, vat_amount
)
.fetch_one(&pool)
.await?;

// Query bills by date range
let bills = sqlx::query_as!(Bill,
    "SELECT * FROM bills WHERE issued_date BETWEEN $1 AND $2 ORDER BY issued_date DESC",
    start_date, end_date
)
.fetch_all(&pool)
.await?;
```

## Constitutional Compliance

✅ **SQLx with compile-time validation**: All queries use `query!` and `query_as!` macros
✅ **PostgreSQL integration**: Native support for Vietnamese text and financial precision
✅ **No ORM violation**: Pure SQLx implementation without additional abstraction layers
✅ **Connection pooling**: Compatible with existing pool configuration

This data model fully satisfies functional requirements FR-001 through FR-010 while maintaining constitutional compliance.