# Quickstart: Bills Export Feature

**Phase**: 1 - Design & Contracts
**Date**: 2025-09-24
**Feature**: Export Bills Table

## Overview
This quickstart validates the bills export feature by testing both CSV and XLSX export functionality with real data scenarios.

## Prerequisites

### Development Environment
```bash
# Ensure Rust toolchain is available
rustc --version  # Should be 1.75+

# Ensure PostgreSQL is running
psql $DATABASE_URL -c "SELECT version();"

# Ensure dependencies are available
cd backend
cargo check
```

### Test Database Setup
```bash
# Run database migrations
cd backend
sqlx migrate run

# Insert sample test data
psql $DATABASE_URL -c "
INSERT INTO bills (form_no, invoice_no, invoice_date, seller_name, buyer_name, total_amount)
VALUES
('Mẫu 01-GTKT', 'INV-2024-001', '2024-09-15', 'Công ty ABC', 'Khách hàng XYZ', 1000000.50),
('Mẫu 02-GTKT', 'INV-2024-002', '2024-09-16', 'Công ty DEF', 'Khách hàng UVW', 2500000.75);
"
```

## Feature Validation Steps

### Step 1: Start Development Server
```bash
# Terminal 1: Start backend server
cd backend
cargo run

# Verify server is running
curl http://localhost:3000/health
# Expected: {"status":"ok"}
```

### Step 2: Test CSV Export
```bash
# Test CSV export endpoint
curl -v "http://localhost:3000/api/bills/export?format=csv" \
  --output "test_export.csv"

# Verify response headers
# Expected:
# HTTP/1.1 200 OK
# Content-Type: text/csv; charset=utf-8
# Content-Disposition: attachment; filename="bills_export_YYYYMMDD_HHMMSS.csv"
# Cache-Control: no-cache, no-store, must-revalidate

# Verify file content
head -5 test_export.csv
# Expected: UTF-8 BOM + CSV headers + data rows
```

**Expected CSV Content**:
```csv
ID,Số tờ khai / Form No,Số hóa đơn / Invoice No,Ngày hóa đơn / Invoice Date,Tên người bán / Seller Name,Tên người mua / Buyer Name,Tổng tiền / Total Amount
1,Mẫu 01-GTKT,INV-2024-001,2024-09-15,Công ty ABC,Khách hàng XYZ,1000000.50
2,Mẫu 02-GTKT,INV-2024-002,2024-09-16,Công ty DEF,Khách hàng UVW,2500000.75
```

### Step 3: Test XLSX Export
```bash
# Test XLSX export endpoint
curl -v "http://localhost:3000/api/bills/export?format=xlsx" \
  --output "test_export.xlsx"

# Verify response headers
# Expected:
# HTTP/1.1 200 OK
# Content-Type: application/vnd.openxmlformats-officedocument.spreadsheetml.sheet
# Content-Disposition: attachment; filename="bills_export_YYYYMMDD_HHMMSS.xlsx"

# Verify file is valid Excel format
file test_export.xlsx
# Expected: Microsoft Excel 2007+
```

**Manual Verification**: Open `test_export.xlsx` in Excel/LibreOffice and verify:
- ✅ All columns display correctly with Vietnamese text
- ✅ Headers are formatted (bold, background color)
- ✅ Numbers display with proper decimal places
- ✅ Dates display in YYYY-MM-DD format

### Step 4: Test Error Handling
```bash
# Test invalid format parameter
curl -v "http://localhost:3000/api/bills/export?format=pdf"
# Expected: HTTP 400 Bad Request
# Expected JSON: {"error":"Invalid format parameter","message":"Supported formats: csv, xlsx","code":"INVALID_FORMAT"}

# Test missing format parameter
curl -v "http://localhost:3000/api/bills/export"
# Expected: HTTP 400 Bad Request
# Expected JSON: {"error":"Missing required parameter","message":"Format parameter is required","code":"MISSING_PARAMETER"}
```

### Step 5: Test Vietnamese Text Handling
```bash
# Insert bill with complex Vietnamese text
psql $DATABASE_URL -c "
INSERT INTO bills (form_no, seller_name, buyer_name, seller_address)
VALUES ('Mẫu 03-GTKT',
        'Công ty TNHH Xuất Nhập Khẩu Đông Nam Á',
        'Khách hàng Nguyễn Thị Hương Giang',
        'Số 123, Phố Trần Hưng Đạo, Quận Hoàn Kiếm, TP. Hà Nội');
"

# Export and verify Vietnamese text
curl "http://localhost:3000/api/bills/export?format=csv" --output "vietnamese_test.csv"

# Check encoding (should show UTF-8)
file vietnamese_test.csv
# Expected: UTF-8 Unicode (with BOM) text

# Verify content displays correctly
cat vietnamese_test.csv | grep "Công ty TNHH"
# Should display Vietnamese characters correctly
```

### Step 6: Test Empty Database
```bash
# Clear all bills temporarily
psql $DATABASE_URL -c "DELETE FROM bills;"

# Test export with empty data
curl "http://localhost:3000/api/bills/export?format=csv" --output "empty_test.csv"

# Verify headers-only file
cat empty_test.csv
# Expected: Just CSV headers, no data rows

# Restore test data
psql $DATABASE_URL -c "
INSERT INTO bills (form_no, invoice_no, total_amount)
VALUES ('Mẫu 01-GTKT', 'INV-2024-001', 1000000.50);
"
```

### Step 7: Performance Test (Optional)
```bash
# Generate larger dataset for performance testing
psql $DATABASE_URL -c "
INSERT INTO bills (form_no, invoice_no, total_amount)
SELECT
    'Mẫu-' || generate_series,
    'INV-2024-' || LPAD(generate_series::text, 6, '0'),
    (generate_series * 1000.0 + random() * 10000)::NUMERIC(18,2)
FROM generate_series(1, 5000);
"

# Time the export
time curl -s "http://localhost:3000/api/bills/export?format=csv" > large_export.csv

# Verify export completed and file size is reasonable
ls -lh large_export.csv
wc -l large_export.csv
# Expected: 5001 lines (header + 5000 data rows)
```

## Success Criteria

### ✅ Functional Requirements Validation
- [x] **FR-001**: Export endpoint accepts format parameter (csv/xlsx)
- [x] **FR-002**: CSV format generates proper UTF-8 with BOM file
- [x] **FR-003**: XLSX format generates valid Excel file
- [x] **FR-004**: All 14 bill fields included with headers
- [x] **FR-005**: File download with proper headers
- [x] **FR-006**: Invalid format returns error
- [x] **FR-007**: Empty database returns headers-only file
- [x] **FR-008**: Proper Content-Type and Content-Disposition headers
- [x] **FR-009**: Vietnamese text preserved correctly
- [x] **FR-010**: Unrestricted access (no authentication)

### ✅ Technical Validation
- [x] **Response Time**: Export completes within reasonable time (<10s for 5k records)
- [x] **Memory Usage**: Constant memory regardless of dataset size
- [x] **File Format**: Generated files open correctly in Excel/LibreOffice
- [x] **Encoding**: Vietnamese characters display properly
- [x] **Headers**: HTTP response headers appropriate for file download
- [x] **Error Handling**: Proper error responses for invalid requests

### ✅ User Experience Validation
- [x] **File Download**: Browser initiates download with meaningful filename
- [x] **Vietnamese Support**: All Vietnamese text renders correctly
- [x] **Excel Compatibility**: XLSX files open in Excel without warnings
- [x] **CSV Compatibility**: CSV files open properly in Excel with BOM
- [x] **Empty Handling**: Graceful handling of empty dataset

## Cleanup
```bash
# Remove test files
rm -f test_export.csv test_export.xlsx vietnamese_test.csv empty_test.csv large_export.csv

# Reset database to clean state if desired
psql $DATABASE_URL -c "DELETE FROM bills WHERE form_no LIKE 'Mẫu-%';"
```

## Troubleshooting

### Common Issues
1. **UTF-8 BOM not working**: Ensure `unicode-bom` crate is properly integrated
2. **Vietnamese text corruption**: Verify database connection uses UTF-8 encoding
3. **Excel doesn't recognize CSV**: Confirm BOM is written before CSV content
4. **XLSX won't open**: Check `rust_xlsxwriter` is generating valid format
5. **Performance issues**: Verify streaming implementation is active

### Debug Commands
```bash
# Check server logs
tail -f backend/server.log

# Verify database connection
psql $DATABASE_URL -c "SHOW server_encoding;"
# Expected: UTF8

# Test dependencies
cargo test --lib export_service
```

**Quickstart Success**: All validation steps pass, confirming the export feature is ready for production use.