#!/bin/bash

# Database Persistence Verification Script
# T016: Verify database persistence of extracted bill data

echo "=== Database Persistence Verification ==="
echo "Verifying Gemini integration database persistence"
echo

# Check if DATABASE_URL is set
if [ -z "$DATABASE_URL" ]; then
    echo "❌ DATABASE_URL environment variable not set"
    echo "   Set it with: export DATABASE_URL=postgresql://user:password@localhost/bill_ocr"
    exit 1
fi

echo "✅ DATABASE_URL is set"
echo "   Database: $DATABASE_URL"
echo

# Test database connection
echo "1. Testing database connection..."
if psql "$DATABASE_URL" -c "SELECT 1;" > /dev/null 2>&1; then
    echo "✅ Database connection successful"
else
    echo "❌ Cannot connect to database"
    echo "   Make sure PostgreSQL is running and database exists"
    exit 1
fi

# Check bills table exists
echo
echo "2. Checking bills table schema..."
table_exists=$(psql "$DATABASE_URL" -t -c "SELECT EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'bills');" 2>/dev/null)

if echo "$table_exists" | grep -q "t"; then
    echo "✅ Bills table exists"

    # Show table structure
    echo
    echo "📋 Bills table structure:"
    psql "$DATABASE_URL" -c "\d bills"
else
    echo "❌ Bills table does not exist"
    echo "   Run migrations with: sqlx migrate run"
    exit 1
fi

# Check required fields for Gemini integration
echo
echo "3. Verifying required fields for Gemini integration..."

required_fields=(
    "id:integer"
    "form_no:text"
    "serial_no:text"
    "invoice_no:text"
    "issued_date:date"
    "seller_name:text"
    "seller_tax_code:text"
    "item_name:text"
    "unit:text"
    "quantity:numeric"
    "unit_price:numeric"
    "total_amount:numeric"
    "vat_rate:numeric"
    "vat_amount:numeric"
)

for field_def in "${required_fields[@]}"; do
    field_name=$(echo "$field_def" | cut -d: -f1)
    expected_type=$(echo "$field_def" | cut -d: -f2)

    field_exists=$(psql "$DATABASE_URL" -t -c "SELECT data_type FROM information_schema.columns WHERE table_name = 'bills' AND column_name = '$field_name';" 2>/dev/null | tr -d ' ')

    if [ -n "$field_exists" ]; then
        echo "  ✅ $field_name ($field_exists)"
    else
        echo "  ❌ $field_name (missing)"
    fi
done

# Test data insertion and retrieval
echo
echo "4. Testing data insertion and retrieval..."

# Insert test data
test_id=$(psql "$DATABASE_URL" -t -c "
INSERT INTO bills (
    form_no, serial_no, invoice_no, issued_date,
    seller_name, seller_tax_code, item_name, unit,
    quantity, unit_price, total_amount, vat_rate, vat_amount
) VALUES (
    'TEST-FORM',
    'TEST-SERIAL',
    'TEST-INV-$(date +%s)',
    '2024-01-15',
    'Test Company Việt Nam',
    '0123456789',
    'Test Service',
    'Hour',
    10.00,
    500000.00,
    5000000.00,
    10.00,
    500000.00
) RETURNING id;
" 2>/dev/null | tr -d ' ')

if [ -n "$test_id" ]; then
    echo "✅ Test data inserted successfully (ID: $test_id)"

    # Verify data can be retrieved
    retrieved_data=$(psql "$DATABASE_URL" -t -c "SELECT form_no, seller_name, total_amount FROM bills WHERE id = $test_id;" 2>/dev/null)

    if echo "$retrieved_data" | grep -q "TEST-FORM"; then
        echo "✅ Test data retrieved successfully"
        echo "   Data: $retrieved_data"
    else
        echo "❌ Failed to retrieve test data"
    fi

    # Clean up test data
    psql "$DATABASE_URL" -c "DELETE FROM bills WHERE id = $test_id;" > /dev/null 2>&1
    echo "✅ Test data cleaned up"
else
    echo "❌ Failed to insert test data"
fi

# Test Vietnamese text support
echo
echo "5. Testing Vietnamese text support..."

vietnamese_test_id=$(psql "$DATABASE_URL" -t -c "
INSERT INTO bills (
    form_no, invoice_no, seller_name
) VALUES (
    '01-GTKT',
    'VN-TEST-$(date +%s)',
    'Công ty TNHH Công nghệ Việt Nam - Test Unicode: àáãạăắằẳẵặâấầẩẫậèéẹêếềểễệìíĩịòóõọôốồổỗộơớờởỡợùúũụưứừửữựỳýỹỵđ'
) RETURNING id;
" 2>/dev/null | tr -d ' ')

if [ -n "$vietnamese_test_id" ]; then
    echo "✅ Vietnamese text inserted successfully"

    # Verify Vietnamese characters are preserved
    vietnamese_data=$(psql "$DATABASE_URL" -t -c "SELECT seller_name FROM bills WHERE id = $vietnamese_test_id;" 2>/dev/null)

    if echo "$vietnamese_data" | grep -q "Công"; then
        echo "✅ Vietnamese text preserved correctly"
    else
        echo "⚠️  Vietnamese text may have encoding issues"
    fi

    # Clean up
    psql "$DATABASE_URL" -c "DELETE FROM bills WHERE id = $vietnamese_test_id;" > /dev/null 2>&1
else
    echo "❌ Failed to insert Vietnamese text"
fi

# Test numeric precision for financial data
echo
echo "6. Testing numeric precision for financial data..."

precision_test_id=$(psql "$DATABASE_URL" -t -c "
INSERT INTO bills (
    form_no, invoice_no, total_amount, vat_rate, vat_amount
) VALUES (
    'PRECISION-TEST',
    'PREC-$(date +%s)',
    123456789.12,
    8.75,
    10802468.55
) RETURNING id;
" 2>/dev/null | tr -d ' ')

if [ -n "$precision_test_id" ]; then
    precision_data=$(psql "$DATABASE_URL" -t -c "SELECT total_amount, vat_rate, vat_amount FROM bills WHERE id = $precision_test_id;" 2>/dev/null)

    if echo "$precision_data" | grep -q "123456789.12"; then
        echo "✅ Financial precision preserved correctly"
        echo "   Data: $precision_data"
    else
        echo "⚠️  Financial precision may have issues"
        echo "   Data: $precision_data"
    fi

    # Clean up
    psql "$DATABASE_URL" -c "DELETE FROM bills WHERE id = $precision_test_id;" > /dev/null 2>&1
else
    echo "❌ Failed to test numeric precision"
fi

# Show current bill count
echo
echo "7. Current database statistics..."
bill_count=$(psql "$DATABASE_URL" -t -c "SELECT COUNT(*) FROM bills;" 2>/dev/null | tr -d ' ')
echo "   Total bills in database: $bill_count"

# Show recent bills (if any)
if [ "$bill_count" -gt 0 ]; then
    echo
    echo "📋 Recent bills (last 3):"
    psql "$DATABASE_URL" -c "
    SELECT
        id,
        form_no,
        invoice_no,
        seller_name,
        total_amount,
        issued_date
    FROM bills
    ORDER BY id DESC
    LIMIT 3;
    "
fi

echo
echo "=== Database Persistence Verification Complete ==="
echo
echo "✅ Success Criteria for T016:"
echo "   - Database connection established"
echo "   - Bills table exists with correct schema"
echo "   - Data insertion and retrieval working"
echo "   - Vietnamese text support verified"
echo "   - Numeric precision for financial data confirmed"
echo "   - Ready for Gemini integration data persistence"
echo
echo "🔗 Next steps:"
echo "   - Test with actual Gemini API responses"
echo "   - Verify BillDataExtractor mapping"
echo "   - Monitor logs during OCR processing"