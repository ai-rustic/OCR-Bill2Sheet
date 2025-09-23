# Gemini Integration Testing Guide

## T015: Test Gemini integration with sample Vietnamese invoices

This document outlines how to test the Gemini AI integration for Vietnamese invoice OCR processing.

## Prerequisites

1. **Environment Setup**
   ```bash
   # Set your Gemini API key
   export GEMINI_API_KEY="your-gemini-api-key-here"

   # Or add to .env file
   echo "GEMINI_API_KEY=your-gemini-api-key-here" >> .env
   ```

2. **Database Setup**
   ```bash
   # Ensure PostgreSQL is running and database exists
   psql $DATABASE_URL -c "\d bills"  # Verify bills table exists
   ```

3. **Server Running**
   ```bash
   cd backend
   cargo run
   ```

## Test Cases

### 1. Valid Vietnamese Invoice Test

**Test Image Requirements:**
- Vietnamese business invoice (Hóa đơn GTGT)
- Clear text, good quality
- Contains typical Vietnamese invoice fields:
  - Mẫu số (Form number)
  - Số hóa đơn (Invoice number)
  - Ngày lập (Issue date)
  - Người bán (Seller information)
  - Người mua (Buyer information)
  - Tổng tiền (Total amount)
  - VAT information

**Test Steps:**
```bash
# Using curl to test the API
curl -X POST http://localhost:3000/api/ocr \
  -F "images=@sample_vietnamese_invoice.jpg" \
  --header "Accept: text/event-stream" \
  -N

# Expected SSE Event Flow:
# 1. upload_started
# 2. image_received
# 3. image_validation_start
# 4. image_validation_success
# 5. gemini_processing_start
# 6. gemini_processing_success
# 7. bill_data_saved
# 8. all_images_validated
# 9. processing_complete
```

### 2. Multiple Images Test

```bash
curl -X POST http://localhost:3000/api/ocr \
  -F "images=@invoice1.jpg" \
  -F "images=@invoice2.png" \
  -F "images=@invoice3.jpg" \
  --header "Accept: text/event-stream" \
  -N
```

### 3. Error Handling Tests

**Invalid Image Format:**
```bash
curl -X POST http://localhost:3000/api/ocr \
  -F "images=@document.pdf" \
  --header "Accept: text/event-stream" \
  -N
```

**Large Image Test:**
```bash
# Create a large image > 10MB
curl -X POST http://localhost:3000/api/ocr \
  -F "images=@large_image.jpg" \
  --header "Accept: text/event-stream" \
  -N
```

## Expected Log Output

With the new logging in place, you should see logs like:

```
INFO  Starting Gemini processing for file invoice.jpg (index: 0)
DEBUG Initializing Gemini service
INFO  Starting Gemini bill data extraction for image of 245123 bytes
DEBUG Encoding image to base64
DEBUG Successfully encoded image to base64, length: 326831
DEBUG Created GeminiRequest with Vietnamese bill extraction prompt
DEBUG Gemini API request attempt 1 of 4
DEBUG Sending request to Gemini API: https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent
DEBUG Sending HTTP request to Gemini API with timeout of 30 seconds
DEBUG Received HTTP response with status: 200
DEBUG Parsing JSON response from Gemini API
DEBUG Gemini API call completed in 2.34s
INFO  Successfully parsed Gemini response in 2.34s
INFO  Successfully extracted bill data from Gemini API in 2.35s
DEBUG Extracting and validating bill data from Gemini response
DEBUG Successfully extracted bill data: form_no=Some("01-GTKT"), invoice_no=Some("AAA24E-001")
DEBUG Saving extracted bill data to database
INFO  Successfully saved bill data to database with ID: 42
```

## Database Verification

After successful processing, verify data was saved:

```sql
SELECT * FROM bills ORDER BY id DESC LIMIT 5;
```

Expected fields populated:
- `form_no` - Vietnamese form number (e.g., "01-GTKT")
- `invoice_no` - Invoice number
- `issued_date` - Date from invoice
- `seller_name` - Company name
- `seller_tax_code` - Tax identification number
- `total_amount` - Invoice total
- `vat_rate` - VAT percentage
- `vat_amount` - VAT amount

## Performance Metrics

Monitor these metrics during testing:

1. **API Response Times:**
   - Image validation: < 100ms
   - Gemini API call: 1-5 seconds (depending on image complexity)
   - Database save: < 50ms
   - Total per image: 1-6 seconds

2. **Memory Usage:**
   - Should remain stable during multiple image processing
   - No memory leaks from base64 encoding/decoding

3. **Error Recovery:**
   - Failed Gemini calls shouldn't crash the server
   - Rate limiting should trigger automatic retries
   - Database failures shouldn't break SSE stream

## Rate Limiting Test

```bash
# Send multiple requests quickly to test rate limiting
for i in {1..10}; do
  curl -X POST http://localhost:3000/api/ocr \
    -F "images=@invoice.jpg" \
    --header "Accept: text/event-stream" \
    -N &
done
```

Expected behavior:
- Some requests may receive 429 errors
- Automatic retry with exponential backoff
- All requests eventually succeed or fail gracefully

## Sample Test Images

Create test images with these characteristics:

1. **High Quality Invoice** - Clear, well-lit Vietnamese invoice
2. **Poor Quality Invoice** - Blurry, low resolution, or tilted
3. **Non-Invoice Image** - Regular photo to test error handling
4. **Multiple Items Invoice** - Complex invoice with many line items
5. **Handwritten Invoice** - To test OCR limits

## Success Criteria

✅ **T015 Success Criteria:**
- [ ] Valid Vietnamese invoices are processed successfully
- [ ] Extracted data matches invoice content accurately
- [ ] Error handling works for invalid inputs
- [ ] Rate limiting and retries function properly
- [ ] Performance meets acceptable thresholds
- [ ] Logging provides useful debugging information
- [ ] SSE events are emitted in correct sequence

## Common Issues & Solutions

1. **"Environment variable GEMINI_API_KEY is required"**
   - Solution: Set the API key in environment or .env file

2. **"Gemini API authentication failed"**
   - Solution: Verify API key is correct and has proper permissions

3. **"Database error"**
   - Solution: Ensure PostgreSQL is running and bills table exists

4. **"Failed to parse JSON response"**
   - Solution: Check if image contains valid Vietnamese invoice content

5. **Rate limiting errors**
   - Solution: This is expected with high volume - automatic retries will handle it

## Manual Testing Checklist

- [ ] Server starts without errors
- [ ] Single image upload works end-to-end
- [ ] Multiple image upload processes all images
- [ ] Database receives extracted data correctly
- [ ] SSE events stream properly in browser
- [ ] Error cases handled gracefully
- [ ] Logs provide sufficient detail for debugging