# Quickstart: Bill Service APIs

## Overview
This quickstart guide demonstrates how to manually test all 7 Bill Service API endpoints using curl commands.

## Prerequisites
- Backend server running on `http://localhost:3000`
- PostgreSQL database with bills table populated
- curl or similar HTTP client tool

## Test Scenarios

### 1. Get All Bills
```bash
curl -X GET http://localhost:3000/bills \
  -H "Content-Type: application/json"
```

**Expected Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "form_no": "Mẫu 01-GTKT",
      "invoice_no": "INV-2024-001",
      // ... other bill fields
    }
  ],
  "error": null
}
```

### 2. Get Bill by ID
```bash
curl -X GET http://localhost:3000/bills/1 \
  -H "Content-Type: application/json"
```

**Expected Response:**
```json
{
  "success": true,
  "data": {
    "id": 1,
    "form_no": "Mẫu 01-GTKT",
    "invoice_no": "INV-2024-001",
    // ... complete bill data
  },
  "error": null
}
```

### 3. Create New Bill
```bash
curl -X POST http://localhost:3000/bills \
  -H "Content-Type: application/json" \
  -d '{
    "form_no": "Mẫu 01-GTKT",
    "serial_no": "AA/24E",
    "invoice_no": "INV-2024-002",
    "issued_date": "2024-09-19",
    "seller_name": "Công ty TNHH ABC",
    "seller_tax_code": "0123456789",
    "item_name": "Dịch vụ tư vấn",
    "unit": "Giờ",
    "quantity": 10.0,
    "unit_price": 500000.00,
    "total_amount": 5000000.00,
    "vat_rate": 10.0,
    "vat_amount": 500000.00
  }'
```

**Expected Response:**
```json
{
  "success": true,
  "data": {
    "id": 2,
    "form_no": "Mẫu 01-GTKT",
    // ... created bill with generated ID
  },
  "error": null
}
```

### 4. Update Existing Bill
```bash
curl -X PUT http://localhost:3000/bills/1 \
  -H "Content-Type: application/json" \
  -d '{
    "form_no": "Mẫu 01-GTKT",
    "serial_no": "AA/24E",
    "invoice_no": "INV-2024-001-UPDATED",
    "issued_date": "2024-09-19",
    "seller_name": "Công ty TNHH ABC - Updated",
    "seller_tax_code": "0123456789",
    "item_name": "Dịch vụ tư vấn - Updated",
    "unit": "Giờ",
    "quantity": 15.0,
    "unit_price": 600000.00,
    "total_amount": 9000000.00,
    "vat_rate": 10.0,
    "vat_amount": 900000.00
  }'
```

**Expected Response:**
```json
{
  "success": true,
  "data": {
    "id": 1,
    "invoice_no": "INV-2024-001-UPDATED",
    // ... updated bill data
  },
  "error": null
}
```

### 5. Search Bills by Invoice Number
```bash
curl -X GET "http://localhost:3000/bills/search?invoice=2024" \
  -H "Content-Type: application/json"
```

**Expected Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "invoice_no": "INV-2024-001-UPDATED",
      // ... matching bills
    },
    {
      "id": 2,
      "invoice_no": "INV-2024-002",
      // ... other matching bills
    }
  ],
  "error": null
}
```

### 6. Get Bills Count
```bash
curl -X GET http://localhost:3000/bills/count \
  -H "Content-Type: application/json"
```

**Expected Response:**
```json
{
  "success": true,
  "data": 2,
  "error": null
}
```

### 7. Delete Bill
```bash
curl -X DELETE http://localhost:3000/bills/2 \
  -H "Content-Type: application/json"
```

**Expected Response:**
```json
{
  "success": true,
  "data": true,
  "error": null
}
```

## Error Scenarios

### 404 - Bill Not Found
```bash
curl -X GET http://localhost:3000/bills/999
```

**Expected Response:**
```json
{
  "success": false,
  "data": null,
  "error": "Resource not found"
}
```

### 400 - Invalid Request Data
```bash
curl -X POST http://localhost:3000/bills \
  -H "Content-Type: application/json" \
  -d '{ "invalid": "json structure" }'
```

**Expected Response:**
```json
{
  "success": false,
  "data": null,
  "error": "Bad request: Invalid bill data"
}
```

### 400 - Missing Search Parameter
```bash
curl -X GET "http://localhost:3000/bills/search" \
  -H "Content-Type: application/json"
```

**Expected Response:**
```json
{
  "success": false,
  "data": null,
  "error": "Bad request: Missing invoice search parameter"
}
```

## Verification Steps

1. **Start Backend Server:**
   ```bash
   cd backend
   cargo run
   ```

2. **Verify Database Connection:**
   Check server logs for successful database connection and migration status.

3. **Execute Test Scenarios:**
   Run all curl commands above in sequence to verify full API functionality.

4. **Check Response Format:**
   Ensure all responses follow the consistent JSON structure with `success`, `data`, and `error` fields.

5. **Verify Vietnamese Text Support:**
   Test with Vietnamese characters in bill data fields to ensure proper encoding.

6. **Validate Financial Precision:**
   Verify that decimal calculations are accurate and maintain proper precision.

## Success Criteria

- ✅ All endpoints return proper HTTP status codes
- ✅ All responses use consistent JSON structure
- ✅ Vietnamese text fields display correctly
- ✅ Financial calculations maintain decimal precision
- ✅ Search functionality works with partial matches
- ✅ Error scenarios return appropriate error messages
- ✅ CRUD operations complete successfully