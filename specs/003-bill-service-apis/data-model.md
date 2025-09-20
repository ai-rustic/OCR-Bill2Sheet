# Data Model: Bill Service APIs

## Entity: Bill

The Bill entity already exists in `backend/src/models/bill.rs` with complete Vietnamese invoice structure.

### Core Fields
- **id**: i32 (auto-generated primary key)
- **form_no**: Option<String> (Mẫu số - Form number)
- **serial_no**: Option<String> (Ký hiệu - Serial number)
- **invoice_no**: Option<String> (Số hóa đơn - Invoice number)
- **issued_date**: Option<NaiveDate> (Ngày phát hành)

### Seller Information
- **seller_name**: Option<String> (Tên người bán)
- **seller_tax_code**: Option<String> (Mã số thuế người bán)

### Item Details
- **item_name**: Option<String> (Tên hàng hóa/dịch vụ)
- **unit**: Option<String> (Đơn vị tính)
- **quantity**: Option<rust_decimal::Decimal> (Số lượng)
- **unit_price**: Option<rust_decimal::Decimal> (Đơn giá)

### Financial Calculations
- **total_amount**: Option<rust_decimal::Decimal> (Thành tiền)
- **vat_rate**: Option<rust_decimal::Decimal> (Thuế suất VAT %)
- **vat_amount**: Option<rust_decimal::Decimal> (Tiền thuế VAT)

## API Response Model

### Standard Response Wrapper
```rust
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}
```

### Response Types
- **Single Bill**: `ApiResponse<Bill>`
- **Bill List**: `ApiResponse<Vec<Bill>>`
- **Bill Count**: `ApiResponse<i64>`
- **Delete Confirmation**: `ApiResponse<bool>`

## Validation Rules

### Financial Precision
- All financial fields use `rust_decimal::Decimal` for exact precision
- Database storage: NUMERIC(18,2) for amounts, NUMERIC(5,2) for rates
- Prevents floating-point rounding errors in financial calculations

### Vietnamese Text Support
- All text fields support full Unicode/UTF-8 encoding
- Proper handling of Vietnamese diacritics and special characters
- Database TEXT fields with collation support

### Business Rules
- **ID**: Auto-generated, immutable
- **Invoice Number**: Should be unique when provided (business rule, not enforced at DB level)
- **Dates**: Standard date format, no future dates validation
- **Financial Fields**: Non-negative values preferred but not enforced

## State Transitions

Bills are simple entities with basic CRUD operations:
- **Created**: New bill with generated ID
- **Updated**: Modified bill with same ID
- **Deleted**: Removed from system (hard delete)

No complex state machine or workflow states required.

## Relationships

Currently, Bills are standalone entities with no foreign key relationships to other tables. Future enhancements might include:
- Customer/Supplier entities
- Product catalog relationships
- Payment tracking

## Data Access Patterns

### Existing Service Methods
The BillService already provides all required database operations:
- `get_all_bills()` - Full list retrieval
- `get_bill_by_id(id)` - Single bill lookup
- `create_bill(data)` - New bill creation
- `update_bill(id, data)` - Bill modification
- `delete_bill(id)` - Bill removal
- `search_bills_by_invoice(pattern)` - Invoice pattern search
- `get_bills_count()` - Total count retrieval

All methods use SQLx compile-time query validation and return `Result<T, ApiError>` for proper error handling.