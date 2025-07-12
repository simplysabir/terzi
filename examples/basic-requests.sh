#!/bin/bash

# Terzi Basic Requests Examples
# This script demonstrates basic HTTP methods with Terzi

echo "🚀 Terzi Basic Requests Examples"
echo "================================"
echo ""

# Ensure we're using the terzi binary
# Adjust the path as needed
TERZI="terzi"

# Check if terzi is available
if ! command -v $TERZI &> /dev/null; then
    echo "❌ Terzi not found. Please install it first."
    echo "   Build from source: cargo build --release"
    exit 1
fi

echo "✅ Using Terzi: $(which $TERZI)"
echo ""

# =============================================================================
# 1. GET REQUESTS
# =============================================================================

echo "📥 1. GET REQUESTS"
echo "=================="

echo "🔹 Simple GET request:"
echo "Command: $TERZI https://jsonplaceholder.typicode.com/posts/1"
$TERZI https://jsonplaceholder.typicode.com/posts/1
echo ""

echo "🔹 GET with query parameters:"
echo "Command: $TERZI 'https://jsonplaceholder.typicode.com/posts?userId=1'"
$TERZI 'https://jsonplaceholder.typicode.com/posts?userId=1'
echo ""

echo "🔹 GET with custom headers:"
echo "Command: $TERZI -H 'User-Agent: Terzi-Example/1.0' https://httpbin.org/headers"
$TERZI -H 'User-Agent: Terzi-Example/1.0' https://httpbin.org/headers
echo ""

echo "🔹 GET with multiple headers:"
echo "Command: $TERZI -H 'Accept: application/json' -H 'User-Agent: MyApp' https://httpbin.org/headers"
$TERZI -H 'Accept: application/json' -H 'User-Agent: MyApp' https://httpbin.org/headers
echo ""

# =============================================================================
# 2. POST REQUESTS
# =============================================================================

echo "📤 2. POST REQUESTS"
echo "==================="

echo "🔹 POST with JSON data:"
echo "Command: $TERZI -m POST -j '{\"title\": \"foo\", \"body\": \"bar\", \"userId\": 1}' https://jsonplaceholder.typicode.com/posts"
$TERZI -m POST -j '{"title": "foo", "body": "bar", "userId": 1}' https://jsonplaceholder.typicode.com/posts
echo ""

echo "🔹 POST with form data:"
echo "Command: $TERZI -m POST -f title=foo -f body=bar -f userId=1 https://httpbin.org/post"
$TERZI -m POST -f title=foo -f body=bar -f userId=1 https://httpbin.org/post
echo ""

echo "🔹 POST with raw body:"
echo "Command: $TERZI -m POST -b 'Hello World' https://httpbin.org/post"
$TERZI -m POST -b 'Hello World' https://httpbin.org/post
echo ""

echo "🔹 POST with custom content type:"
echo "Command: $TERZI -m POST -H 'Content-Type: text/plain' -b 'Plain text data' https://httpbin.org/post"
$TERZI -m POST -H 'Content-Type: text/plain' -b 'Plain text data' https://httpbin.org/post
echo ""

# =============================================================================
# 3. PUT REQUESTS
# =============================================================================

echo "🔄 3. PUT REQUESTS"
echo "=================="

echo "🔹 PUT with JSON data:"
echo "Command: $TERZI -m PUT -j '{\"id\": 1, \"title\": \"updated\", \"body\": \"updated body\", \"userId\": 1}' https://jsonplaceholder.typicode.com/posts/1"
$TERZI -m PUT -j '{"id": 1, "title": "updated", "body": "updated body", "userId": 1}' https://jsonplaceholder.typicode.com/posts/1
echo ""

echo "🔹 PUT with form data:"
echo "Command: $TERZI -m PUT -f title=updated -f body='updated body' https://httpbin.org/put"
$TERZI -m PUT -f title=updated -f body='updated body' https://httpbin.org/put
echo ""

# =============================================================================
# 4. PATCH REQUESTS
# =============================================================================

echo "🔧 4. PATCH REQUESTS"
echo "===================="

echo "🔹 PATCH with JSON data:"
echo "Command: $TERZI -m PATCH -j '{\"title\": \"patched\"}' https://jsonplaceholder.typicode.com/posts/1"
$TERZI -m PATCH -j '{"title": "patched"}' https://jsonplaceholder.typicode.com/posts/1
echo ""

# =============================================================================
# 5. DELETE REQUESTS
# =============================================================================

echo "🗑️ 5. DELETE REQUESTS"
echo "====================="

echo "🔹 Simple DELETE:"
echo "Command: $TERZI -m DELETE https://jsonplaceholder.typicode.com/posts/1"
$TERZI -m DELETE https://jsonplaceholder.typicode.com/posts/1
echo ""

echo "🔹 DELETE with headers:"
echo "Command: $TERZI -m DELETE -H 'X-Custom-Header: delete-operation' https://httpbin.org/delete"
$TERZI -m DELETE -H 'X-Custom-Header: delete-operation' https://httpbin.org/delete
echo ""

# =============================================================================
# 6. HEAD REQUESTS
# =============================================================================

echo "📋 6. HEAD REQUESTS"
echo "==================="

echo "🔹 HEAD request (headers only):"
echo "Command: $TERZI -m HEAD https://jsonplaceholder.typicode.com/posts/1"
$TERZI -m HEAD https://jsonplaceholder.typicode.com/posts/1
echo ""

# =============================================================================
# 7. OPTIONS REQUESTS
# =============================================================================

echo "⚙️ 7. OPTIONS REQUESTS"
echo "======================"

echo "🔹 OPTIONS request:"
echo "Command: $TERZI -m OPTIONS https://httpbin.org/get"
$TERZI -m OPTIONS https://httpbin.org/get
echo ""

# =============================================================================
# 8. OUTPUT FORMATTING
# =============================================================================

echo "🎨 8. OUTPUT FORMATTING"
echo "======================="

echo "🔹 JSON output format:"
echo "Command: $TERZI -o json https://jsonplaceholder.typicode.com/posts/1"
$TERZI -o json https://jsonplaceholder.typicode.com/posts/1
echo ""

echo "🔹 Include response headers:"
echo "Command: $TERZI -i https://jsonplaceholder.typicode.com/posts/1"
$TERZI -i https://jsonplaceholder.typicode.com/posts/1
echo ""

echo "🔹 Verbose output:"
echo "Command: $TERZI -v https://jsonplaceholder.typicode.com/posts/1"
$TERZI -v https://jsonplaceholder.typicode.com/posts/1
echo ""

echo "🔹 Silent mode:"
echo "Command: $TERZI -S https://jsonplaceholder.typicode.com/posts/1"
$TERZI -S https://jsonplaceholder.typicode.com/posts/1
echo ""

# =============================================================================
# 9. TIMEOUTS AND REDIRECTS
# =============================================================================

echo "⏱️ 9. TIMEOUTS AND REDIRECTS"
echo "============================"

echo "🔹 Custom timeout (5 seconds):"
echo "Command: $TERZI -t 5 https://jsonplaceholder.typicode.com/posts/1"
$TERZI -t 5 https://jsonplaceholder.typicode.com/posts/1
echo ""

echo "🔹 Follow redirects:"
echo "Command: $TERZI -L https://httpbin.org/redirect/2"
$TERZI -L https://httpbin.org/redirect/2
echo ""

# =============================================================================
# 10. ERROR HANDLING
# =============================================================================

echo "❌ 10. ERROR HANDLING"
echo "====================="

echo "🔹 404 Not Found:"
echo "Command: $TERZI https://jsonplaceholder.typicode.com/posts/999999"
$TERZI https://jsonplaceholder.typicode.com/posts/999999
echo ""

echo "🔹 Invalid URL (will show validation error):"
echo "Command: $TERZI not-a-valid-url"
$TERZI not-a-valid-url 2>&1 || echo "✅ Error handled gracefully"
echo ""

echo "🔹 Connection timeout (short timeout on slow endpoint):"
echo "Command: $TERZI -t 1 https://httpbin.org/delay/2"
$TERZI -t 1 https://httpbin.org/delay/2 2>&1 || echo "✅ Timeout handled gracefully"
echo ""

# =============================================================================
# SUMMARY
# =============================================================================

echo "🎉 SUMMARY"
echo "=========="
echo ""
echo "✅ You've learned how to:"
echo "   • Make GET, POST, PUT, PATCH, DELETE, HEAD, and OPTIONS requests"
echo "   • Add custom headers and authentication"
echo "   • Send JSON, form data, and raw body content"
echo "   • Control output formatting and verbosity"
echo "   • Handle timeouts and redirects"
echo "   • Work with error responses"
echo ""
echo "🚀 Next steps:"
echo "   • Try the authentication examples: ./examples/authentication.sh"
echo "   • Learn about request management: ./examples/request-management.sh"
echo "   • Explore advanced features: ./examples/advanced-usage.sh"
echo "   • Use interactive mode: terzi interactive"
echo ""
echo "📖 For more help:"
echo "   • terzi --help"
echo "   • terzi <command> --help"
echo "   • Check out the documentation in docs/"
echo "" 