#!/bin/bash

# Test Gemini API with response schema for Vietnamese bill OCR
# This demonstrates the new structured response format that matches our bills table schema

echo "🔥 Testing Gemini API with Response Schema for Vietnamese Bills"
echo "================================================================"

# Check if GEMINI_API_KEY is set
if [ -z "$GEMINI_API_KEY" ]; then
    echo "❌ Error: GEMINI_API_KEY environment variable is not set"
    echo "   Please set it with: export GEMINI_API_KEY=your_api_key_here"
    exit 1
fi

# Create a test prompt for Vietnamese bill
TEST_PROMPT="Phân tích hóa đơn Việt Nam trong hình ảnh này và trích xuất thông tin theo định dạng JSON với các trường: form_no (mẫu số), serial_no (ký hiệu), invoice_no (số hóa đơn), issued_date (ngày lập), seller_name (tên người bán), seller_tax_code (mã số thuế), item_name (tên hàng hóa), unit (đơn vị), quantity (số lượng), unit_price (đơn giá), total_amount (thành tiền), vat_rate (thuế suất VAT %), vat_amount (tiền thuế VAT)."

# Create test base64 image (minimal 1x1 PNG for testing)
TEST_IMAGE_B64="iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg=="

echo "📤 Sending request to Gemini API with response schema..."
echo ""

# Make the API call with response schema
curl -H 'Content-Type: application/json' \
     -H "x-goog-api-key: ${GEMINI_API_KEY}" \
     -d "{
  \"contents\": [
    {
      \"parts\": [
        { \"text\": \"$TEST_PROMPT\" },
        {
          \"inlineData\": {
            \"mimeType\": \"image/png\",
            \"data\": \"$TEST_IMAGE_B64\"
          }
        }
      ]
    }
  ],
  \"generationConfig\": {
    \"temperature\": 0.1,
    \"topK\": 1,
    \"topP\": 0.8,
    \"maxOutputTokens\": 2048,
    \"responseMimeType\": \"application/json\",
    \"responseSchema\": {
      \"type\": \"object\",
      \"properties\": {
        \"form_no\": {
          \"type\": \"string\",
          \"description\": \"Mẫu số hóa đơn (ví dụ: 01-GTKT, 02-GTTT)\"
        },
        \"serial_no\": {
          \"type\": \"string\",
          \"description\": \"Ký hiệu hóa đơn (ví dụ: AA/24E, BB/25F)\"
        },
        \"invoice_no\": {
          \"type\": \"string\",
          \"description\": \"Số hóa đơn\"
        },
        \"issued_date\": {
          \"type\": \"string\",
          \"format\": \"date\",
          \"description\": \"Ngày lập hóa đơn (YYYY-MM-DD)\"
        },
        \"seller_name\": {
          \"type\": \"string\",
          \"description\": \"Tên người bán/công ty\"
        },
        \"seller_tax_code\": {
          \"type\": \"string\",
          \"description\": \"Mã số thuế của người bán\"
        },
        \"item_name\": {
          \"type\": \"string\",
          \"description\": \"Tên hàng hóa/dịch vụ\"
        },
        \"unit\": {
          \"type\": \"string\",
          \"description\": \"Đơn vị tính (ví dụ: cái, kg, giờ, m2)\"
        },
        \"quantity\": {
          \"type\": \"number\",
          \"description\": \"Số lượng hàng hóa/dịch vụ\"
        },
        \"unit_price\": {
          \"type\": \"number\",
          \"description\": \"Đơn giá (VND)\"
        },
        \"total_amount\": {
          \"type\": \"number\",
          \"description\": \"Thành tiền trước thuế (VND)\"
        },
        \"vat_rate\": {
          \"type\": \"number\",
          \"description\": \"Thuế suất VAT (%) - ví dụ: 0, 5, 8, 10\"
        },
        \"vat_amount\": {
          \"type\": \"number\",
          \"description\": \"Tiền thuế VAT (VND)\"
        }
      },
      \"required\": []
    }
  },
  \"safetySettings\": [
    {
      \"category\": \"HARM_CATEGORY_HARASSMENT\",
      \"threshold\": \"BLOCK_MEDIUM_AND_ABOVE\"
    },
    {
      \"category\": \"HARM_CATEGORY_HATE_SPEECH\",
      \"threshold\": \"BLOCK_MEDIUM_AND_ABOVE\"
    },
    {
      \"category\": \"HARM_CATEGORY_SEXUALLY_EXPLICIT\",
      \"threshold\": \"BLOCK_MEDIUM_AND_ABOVE\"
    },
    {
      \"category\": \"HARM_CATEGORY_DANGEROUS_CONTENT\",
      \"threshold\": \"BLOCK_MEDIUM_AND_ABOVE\"
    }
  ]
}" \
"https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent" | jq .

echo ""
echo "✅ Response Schema Benefits:"
echo "   - Gemini returns structured JSON matching bills table schema"
echo "   - All 13 fields defined with Vietnamese descriptions"
echo "   - Reduced parsing errors and improved data consistency"
echo "   - Better field mapping to database columns"
echo ""
echo "📋 Fields included in response schema:"
echo "   1. form_no (Mẫu số hóa đơn)"
echo "   2. serial_no (Ký hiệu hóa đơn)"
echo "   3. invoice_no (Số hóa đơn)"
echo "   4. issued_date (Ngày lập)"
echo "   5. seller_name (Tên người bán)"
echo "   6. seller_tax_code (Mã số thuế)"
echo "   7. item_name (Tên hàng hóa)"
echo "   8. unit (Đơn vị tính)"
echo "   9. quantity (Số lượng)"
echo "   10. unit_price (Đơn giá)"
echo "   11. total_amount (Thành tiền)"
echo "   12. vat_rate (Thuế suất VAT)"
echo "   13. vat_amount (Tiền thuế VAT)"