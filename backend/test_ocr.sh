#!/bin/bash

# OCR API Test Script
# Tests the Gemini integration without requiring actual API keys

echo "=== OCR API Integration Test ==="
echo "Testing Vietnamese invoice OCR with Gemini AI integration"
echo

# Check if server is running
echo "1. Checking if server is running..."
if curl -s http://localhost:3000/health > /dev/null 2>&1; then
    echo "✅ Server is running on localhost:3000"
else
    echo "❌ Server not running. Start with: cargo run"
    echo "   Make sure to set GEMINI_API_KEY environment variable"
    exit 1
fi

# Create a test image (simple 1x1 PNG for API testing)
echo
echo "2. Creating test image..."
cat > test_image.png << 'EOF'
iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChAGEhUZkNwAAAABJRU5ErkJggg==
EOF

# Convert base64 to binary
base64 -d test_image.png > test.png 2>/dev/null || {
    # Create a simple test file if base64 decode fails
    echo -e "\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x01\x00\x00\x00\x01\x08\x02\x00\x00\x00\x90wS\xde\x00\x00\x00\x0cIDATx\xdac\xf8\x00\x00\x00\x00\x01\x00\x01\x00\x00\x00\x00IEND\xaeB\x60\x82" > test.png
}

echo "✅ Test image created"

# Test the API endpoint
echo
echo "3. Testing OCR API endpoint..."
echo "   Endpoint: POST /api/ocr"
echo "   Expected: SSE stream with processing events"
echo

# Test with curl
echo "4. Sending test request..."
response=$(curl -s -w "\n%{http_code}" -X POST http://localhost:3000/api/ocr \
    -F "images=@test.png" \
    --header "Accept: text/event-stream" \
    -m 30 2>/dev/null)

http_code=$(echo "$response" | tail -n1)
content=$(echo "$response" | head -n -1)

if [ "$http_code" = "200" ]; then
    echo "✅ API endpoint responding (HTTP 200)"
    echo
    echo "📋 Response content preview:"
    echo "$content" | head -20
    if echo "$content" | grep -q "upload_started"; then
        echo "✅ SSE events detected in response"
    else
        echo "⚠️  No SSE events found - check if processing started"
    fi
else
    echo "❌ API returned HTTP $http_code"
    echo "Response: $content"
fi

# Clean up
rm -f test.png test_image.png

echo
echo "=== Test Complete ==="
echo
echo "📝 Manual Testing Steps:"
echo "1. Set GEMINI_API_KEY environment variable"
echo "2. Start server: cargo run"
echo "3. Use browser or curl to test with real Vietnamese invoice images"
echo "4. Monitor logs for Gemini processing details"
echo "5. Check database for saved bill data"
echo
echo "📊 For detailed testing guide, see: TESTING.md"