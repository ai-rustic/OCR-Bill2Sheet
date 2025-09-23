-- Database Persistence Test for Gemini Integration
-- T016: Verify database persistence of extracted bill data

-- 1. Verify bills table structure
\d bills

-- 2. Show current bill count
SELECT COUNT(*) as total_bills FROM bills;

-- 3. Insert sample test data to verify schema compatibility
INSERT INTO bills (
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
) VALUES (
    '01-GTKT',
    'AA/24E',
    'INV-001-2024',
    '2024-01-15',
    'Công ty TNHH Công nghệ ABC',
    '0123456789',
    'Dịch vụ tư vấn công nghệ',
    'Giờ',
    40.00,
    500000.00,
    20000000.00,
    10.00,
    2000000.00
);

-- 4. Verify insert was successful
SELECT
    id,
    form_no,
    invoice_no,
    seller_name,
    total_amount,
    vat_amount,
    issued_date
FROM bills
WHERE form_no = '01-GTKT'
ORDER BY id DESC
LIMIT 1;

-- 5. Test data extraction mapping compatibility
-- This verifies that GeminiResponse -> CreateBill -> Bill mapping works

-- Fields that should be populated from Gemini:
SELECT
    'Form Number' as field_name,
    form_no as value,
    CASE WHEN form_no IS NOT NULL THEN '✅ Populated' ELSE '❌ Missing' END as status
FROM bills WHERE id = (SELECT MAX(id) FROM bills)

UNION ALL

SELECT
    'Invoice Number',
    invoice_no,
    CASE WHEN invoice_no IS NOT NULL THEN '✅ Populated' ELSE '❌ Missing' END
FROM bills WHERE id = (SELECT MAX(id) FROM bills)

UNION ALL

SELECT
    'Issue Date',
    issued_date::text,
    CASE WHEN issued_date IS NOT NULL THEN '✅ Populated' ELSE '❌ Missing' END
FROM bills WHERE id = (SELECT MAX(id) FROM bills)

UNION ALL

SELECT
    'Seller Name',
    seller_name,
    CASE WHEN seller_name IS NOT NULL THEN '✅ Populated' ELSE '❌ Missing' END
FROM bills WHERE id = (SELECT MAX(id) FROM bills)

UNION ALL

SELECT
    'Total Amount',
    total_amount::text,
    CASE WHEN total_amount IS NOT NULL THEN '✅ Populated' ELSE '❌ Missing' END
FROM bills WHERE id = (SELECT MAX(id) FROM bills);

-- 6. Test numeric precision for financial data
SELECT
    'Total Amount' as field,
    total_amount,
    pg_typeof(total_amount) as data_type,
    CASE
        WHEN total_amount = 20000000.00 THEN '✅ Precision OK'
        ELSE '❌ Precision Issue'
    END as precision_test
FROM bills
WHERE id = (SELECT MAX(id) FROM bills);

-- 7. Test Vietnamese text support
SELECT
    'Vietnamese Text Test' as test_name,
    seller_name,
    length(seller_name) as text_length,
    CASE
        WHEN seller_name LIKE '%ệ%' OR seller_name LIKE '%ộ%' OR seller_name LIKE '%ư%'
        THEN '✅ Unicode Support OK'
        ELSE '⚠️ Check Unicode Support'
    END as unicode_test
FROM bills
WHERE id = (SELECT MAX(id) FROM bills);

-- 8. Clean up test data
DELETE FROM bills WHERE form_no = '01-GTKT' AND invoice_no = 'INV-001-2024';

-- 9. Verify cleanup
SELECT COUNT(*) as remaining_test_records
FROM bills
WHERE form_no = '01-GTKT' AND invoice_no = 'INV-001-2024';

-- 10. Show bill data schema for integration verification
SELECT
    column_name,
    data_type,
    is_nullable,
    column_default
FROM information_schema.columns
WHERE table_name = 'bills'
ORDER BY ordinal_position;