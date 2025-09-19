# Quickstart: Bill Table Implementation

**Date**: 2025-09-19
**Estimated Time**: 30 minutes
**Prerequisites**: PostgreSQL database running, SQLx CLI installed

## Overview

This quickstart validates the bill table creation and basic operations for the OCR invoice processing system.

## Setup Steps

### 1. Environment Verification
```bash
# Verify database connection
psql $DATABASE_URL -c "SELECT version();"

# Verify SQLx CLI
sqlx --version
```

### 2. Run Migration
```bash
# Navigate to backend directory
cd backend

# Create migration (if not exists)
sqlx migrate add create_bill_table

# Apply migration
sqlx migrate run
```

### 3. Verify Table Creation
```bash
# Connect to database and verify schema
psql $DATABASE_URL -c "\d bills"
```

Expected output should show:
- `id` column as SERIAL PRIMARY KEY
- 13 additional columns with correct types
- Text fields as `text` type
- Numeric fields as `numeric(18,2)` or `numeric(5,2)`
- Date field as `date` type

## Validation Tests

### Test 1: Basic Data Insertion
```sql
INSERT INTO bills (
    form_no, serial_no, invoice_no, issued_date,
    seller_name, seller_tax_code, item_name, unit,
    quantity, unit_price, total_amount, vat_rate, vat_amount
) VALUES (
    'Form-001', 'AB/2024', 'INV-00001', '2024-01-15',
    'Công ty ABC', '0123456789', 'Laptop Dell', 'cái',
    1.00, 15000000.00, 15000000.00, 10.00, 1500000.00
);
```

### Test 2: Query Data Back
```sql
SELECT
    id, form_no, invoice_no, seller_name,
    quantity, unit_price, total_amount, vat_rate, vat_amount
FROM bills
WHERE invoice_no = 'INV-00001';
```

### Test 3: Numeric Precision Verification
```sql
-- Verify decimal precision is maintained
SELECT
    quantity, unit_price, total_amount, vat_rate, vat_amount,
    (quantity * unit_price) AS calculated_total,
    (total_amount * vat_rate / 100) AS calculated_vat
FROM bills
WHERE id = 1;
```

### Test 4: Vietnamese Text Support
```sql
INSERT INTO bills (
    seller_name, item_name, unit
) VALUES (
    'Công ty TNHH Phần mềm Việt Nam',
    'Dịch vụ tư vấn công nghệ thông tin',
    'giờ'
);

SELECT seller_name, item_name, unit
FROM bills
WHERE seller_name LIKE '%Việt Nam%';
```

## Rollback Test

### Test Migration Rollback
```bash
# Test rollback capability
sqlx migrate revert

# Verify table is removed
psql $DATABASE_URL -c "\d bills"
# Should return "relation does not exist"

# Re-apply migration
sqlx migrate run
```

## Compile-time Validation

### Test SQLx Query Macros
Create a test file `src/models/bill.rs`:

```rust
use sqlx::{PgPool, FromRow};

#[derive(FromRow)]
pub struct Bill {
    pub id: i32,
    pub form_no: Option<String>,
    pub invoice_no: Option<String>,
    pub seller_name: Option<String>,
    pub quantity: Option<sqlx::types::Decimal>,
    pub unit_price: Option<sqlx::types::Decimal>,
    pub total_amount: Option<sqlx::types::Decimal>,
}

pub async fn test_query_compilation(pool: &PgPool) -> Result<Vec<Bill>, sqlx::Error> {
    sqlx::query_as!(
        Bill,
        "SELECT id, form_no, invoice_no, seller_name, quantity, unit_price, total_amount
         FROM bills ORDER BY id LIMIT 10"
    )
    .fetch_all(pool)
    .await
}
```

Run compile-time validation:
```bash
cargo check
```

## Success Criteria

✅ **Migration Success**: Table created without errors
✅ **Data Insertion**: Vietnamese text and numeric precision preserved
✅ **Query Operations**: All field types return correctly
✅ **Rollback Safety**: Migration can be reverted cleanly
✅ **Compile-time Validation**: SQLx macros validate schema
✅ **Performance**: Basic operations complete under 100ms

## Troubleshooting

### Common Issues

**Issue**: Migration fails with permission error
**Solution**: Verify database user has CREATE TABLE permissions

**Issue**: Numeric precision errors
**Solution**: Ensure PostgreSQL version supports NUMERIC(18,2)

**Issue**: Vietnamese text displays incorrectly
**Solution**: Verify database encoding is UTF-8

**Issue**: SQLx compile errors
**Solution**: Run `sqlx prepare` to generate query metadata

## Next Steps

After successful quickstart:
1. Integrate with OCR processing pipeline
2. Add indexes for performance optimization
3. Implement business logic for bill validation
4. Add monitoring for table usage patterns

This quickstart validates all functional requirements FR-001 through FR-010 are properly implemented.