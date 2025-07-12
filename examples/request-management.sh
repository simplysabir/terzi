#!/bin/bash

# Terzi Request Management Examples
# This script demonstrates saving, loading, and managing requests with Terzi

echo "🗂️ Terzi Request Management Examples"
echo "===================================="
echo ""

# Ensure we're using the terzi binary
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
# 1. SAVING REQUESTS
# =============================================================================

echo "💾 1. SAVING REQUESTS"
echo "====================="

echo "🔹 Save a simple GET request:"
echo "Command: $TERZI --save 'get-post' https://jsonplaceholder.typicode.com/posts/1"
$TERZI --save 'get-post' https://jsonplaceholder.typicode.com/posts/1
echo ""

echo "🔹 Save a POST request with JSON data:"
echo "Command: $TERZI --save 'create-post' -m POST -j '{\"title\": \"My Post\", \"body\": \"Post content\", \"userId\": 1}' https://jsonplaceholder.typicode.com/posts"
$TERZI --save 'create-post' -m POST -j '{"title": "My Post", "body": "Post content", "userId": 1}' https://jsonplaceholder.typicode.com/posts
echo ""

echo "🔹 Save an authenticated request:"
echo "Command: $TERZI --save 'github-user' -A 'bearer:fake-token' https://api.github.com/user"
$TERZI --save 'github-user' -A 'bearer:fake-token' https://api.github.com/user
echo ""

echo "🔹 Save a request with custom headers:"
echo "Command: $TERZI --save 'custom-headers' -H 'X-Custom-Header: value' -H 'Accept: application/json' https://httpbin.org/headers"
$TERZI --save 'custom-headers' -H 'X-Custom-Header: value' -H 'Accept: application/json' https://httpbin.org/headers
echo ""

echo "🔹 Save a form data request:"
echo "Command: $TERZI --save 'submit-form' -m POST -f name=John -f email=john@example.com https://httpbin.org/post"
$TERZI --save 'submit-form' -m POST -f name=John -f email=john@example.com https://httpbin.org/post
echo ""

# =============================================================================
# 2. LISTING SAVED REQUESTS
# =============================================================================

echo "📋 2. LISTING SAVED REQUESTS"
echo "============================="

echo "🔹 List all saved requests:"
echo "Command: $TERZI list"
$TERZI list
echo ""

echo "🔹 List requests with filter:"
echo "Command: $TERZI list --filter 'post'"
$TERZI list --filter 'post'
echo ""

echo "🔹 List requests with filter (case-insensitive):"
echo "Command: $TERZI list --filter 'GET'"
$TERZI list --filter 'GET'
echo ""

# =============================================================================
# 3. SHOWING REQUEST DETAILS
# =============================================================================

echo "🔍 3. SHOWING REQUEST DETAILS"
echo "=============================="

echo "🔹 Show details of a saved request:"
echo "Command: $TERZI show 'get-post'"
$TERZI show 'get-post'
echo ""

echo "🔹 Show details of authenticated request:"
echo "Command: $TERZI show 'github-user'"
$TERZI show 'github-user'
echo ""

echo "🔹 Show details of request with form data:"
echo "Command: $TERZI show 'submit-form'"
$TERZI show 'submit-form'
echo ""

# =============================================================================
# 4. LOADING AND EXECUTING SAVED REQUESTS
# =============================================================================

echo "🚀 4. LOADING AND EXECUTING SAVED REQUESTS"
echo "=========================================="

echo "🔹 Load and execute a saved GET request:"
echo "Command: $TERZI --load 'get-post'"
$TERZI --load 'get-post'
echo ""

echo "🔹 Load and execute a saved POST request:"
echo "Command: $TERZI --load 'create-post'"
$TERZI --load 'create-post'
echo ""

echo "🔹 Load and execute request with custom headers:"
echo "Command: $TERZI --load 'custom-headers'"
$TERZI --load 'custom-headers'
echo ""

# =============================================================================
# 5. EDITING SAVED REQUESTS
# =============================================================================

echo "✏️ 5. EDITING SAVED REQUESTS"
echo "============================="

echo "🔹 Edit a saved request (interactive):"
echo "Command: $TERZI edit 'get-post'"
echo "Note: This will open an interactive editor"
echo "For this demo, we'll skip the interactive part"
echo ""

echo "🔹 To edit requests, you can:"
echo "   • Delete the old one and save a new version"
echo "   • Use the interactive editor with 'terzi edit <name>'"
echo "   • Modify and save with the same name (overwrites)"
echo ""

# =============================================================================
# 6. DELETING SAVED REQUESTS
# =============================================================================

echo "🗑️ 6. DELETING SAVED REQUESTS"
echo "=============================="

echo "🔹 Delete a saved request:"
echo "Command: $TERZI delete 'get-post'"
echo "This will ask for confirmation before deleting"
echo ""

echo "🔹 Let's create a temporary request and delete it:"
echo "Command: $TERZI --save 'temp-request' https://httpbin.org/get"
$TERZI --save 'temp-request' https://httpbin.org/get
echo ""

echo "🔹 Now delete the temporary request:"
echo "Command: $TERZI delete 'temp-request'"
# Note: This will require confirmation, so we'll simulate it
echo "Would delete 'temp-request' (skipping confirmation for demo)"
echo ""

# =============================================================================
# 7. REQUEST HISTORY
# =============================================================================

echo "📊 7. REQUEST HISTORY"
echo "====================="

echo "🔹 View request history:"
echo "Command: $TERZI history"
$TERZI history
echo ""

echo "🔹 View limited history:"
echo "Command: $TERZI history --limit 5"
$TERZI history --limit 5
echo ""

echo "🔹 History includes:"
echo "   • Timestamp of each request"
echo "   • HTTP method and URL"
echo "   • Response status code"
echo "   • Request duration"
echo "   • Success/error indicators"
echo ""

# =============================================================================
# 8. ADVANCED REQUEST MANAGEMENT
# =============================================================================

echo "🎯 8. ADVANCED REQUEST MANAGEMENT"
echo "================================="

echo "🔹 Save a complex request with multiple options:"
echo "Command: $TERZI --save 'complex-request' -m POST -H 'Content-Type: application/json' -H 'X-API-Key: secret' -A 'bearer:token' -j '{\"data\": \"test\"}' -t 30 -L https://httpbin.org/post"
$TERZI --save 'complex-request' -m POST -H 'Content-Type: application/json' -H 'X-API-Key: secret' -A 'bearer:token' -j '{"data": "test"}' -t 30 -L https://httpbin.org/post
echo ""

echo "🔹 Show the complex request details:"
echo "Command: $TERZI show 'complex-request'"
$TERZI show 'complex-request'
echo ""

echo "🔹 Execute the complex request:"
echo "Command: $TERZI --load 'complex-request'"
$TERZI --load 'complex-request'
echo ""

# =============================================================================
# 9. ORGANIZING REQUESTS
# =============================================================================

echo "📁 9. ORGANIZING REQUESTS"
echo "========================="

echo "🔹 Save requests with descriptive names:"
echo "Command: $TERZI --save 'api-users-list' https://jsonplaceholder.typicode.com/users"
$TERZI --save 'api-users-list' https://jsonplaceholder.typicode.com/users
echo ""

echo "Command: $TERZI --save 'api-users-create' -m POST -j '{\"name\": \"John Doe\", \"email\": \"john@example.com\"}' https://jsonplaceholder.typicode.com/users"
$TERZI --save 'api-users-create' -m POST -j '{"name": "John Doe", "email": "john@example.com"}' https://jsonplaceholder.typicode.com/users
echo ""

echo "Command: $TERZI --save 'api-users-update' -m PUT -j '{\"name\": \"Jane Doe\", \"email\": \"jane@example.com\"}' https://jsonplaceholder.typicode.com/users/1"
$TERZI --save 'api-users-update' -m PUT -j '{"name": "Jane Doe", "email": "jane@example.com"}' https://jsonplaceholder.typicode.com/users/1
echo ""

echo "Command: $TERZI --save 'api-users-delete' -m DELETE https://jsonplaceholder.typicode.com/users/1"
$TERZI --save 'api-users-delete' -m DELETE https://jsonplaceholder.typicode.com/users/1
echo ""

echo "🔹 List requests with API filter:"
echo "Command: $TERZI list --filter 'api-users'"
$TERZI list --filter 'api-users'
echo ""

# =============================================================================
# 10. EXPORT AND IMPORT
# =============================================================================

echo "📤 10. EXPORT AND IMPORT"
echo "======================="

echo "🔹 Export all requests to JSON:"
echo "Command: $TERZI export --output my-requests.json"
$TERZI export --output my-requests.json
echo ""

echo "🔹 Export requests to YAML:"
echo "Command: $TERZI export --format yaml --output my-requests.yaml"
$TERZI export --format yaml --output my-requests.yaml
echo ""

echo "🔹 Check exported files:"
echo "Command: ls -la my-requests.*"
ls -la my-requests.* 2>/dev/null || echo "Files exported to current directory"
echo ""

echo "🔹 View exported JSON (first few lines):"
echo "Command: head -20 my-requests.json"
head -20 my-requests.json 2>/dev/null || echo "Export file not found"
echo ""

# =============================================================================
# 11. BEST PRACTICES
# =============================================================================

echo "✅ 11. BEST PRACTICES"
echo "====================="

echo "🔹 Use descriptive names for saved requests:"
echo "   • Good: 'github-user-profile', 'api-create-user', 'auth-login'"
echo "   • Bad: 'req1', 'test', 'api'"
echo ""

echo "🔹 Organize requests by service or functionality:"
echo "   • 'github-*' for GitHub API requests"
echo "   • 'auth-*' for authentication requests"
echo "   • 'user-*' for user management requests"
echo ""

echo "🔹 Save frequently used requests:"
echo "   • API endpoints you test regularly"
echo "   • Complex requests with many parameters"
echo "   • Authenticated requests to avoid retyping tokens"
echo ""

echo "🔹 Use history to track API usage:"
echo "   • Monitor response times and success rates"
echo "   • Debug API issues by checking recent requests"
echo "   • Identify patterns in API usage"
echo ""

echo "🔹 Export your requests for backup:"
echo "   • Regular backups of your request collection"
echo "   • Share request collections with team members"
echo "   • Version control your API testing suite"
echo ""

# =============================================================================
# SUMMARY
# =============================================================================

echo "🎉 SUMMARY"
echo "=========="
echo ""
echo "✅ You've learned how to:"
echo "   • Save requests with meaningful names"
echo "   • List and filter saved requests"
echo "   • Show detailed information about requests"
echo "   • Load and execute saved requests"
echo "   • Edit existing requests"
echo "   • Delete unwanted requests"
echo "   • View request history and analytics"
echo "   • Export and import request collections"
echo "   • Organize requests effectively"
echo "   • Apply best practices for request management"
echo ""
echo "🚀 Next steps:"
echo "   • Try configuration examples: ./examples/configuration.sh"
echo "   • Explore advanced features: ./examples/advanced-usage.sh"
echo "   • Work with real APIs: ./examples/real-world-apis.sh"
echo "   • Use interactive mode: terzi interactive"
echo ""
echo "📖 For more help:"
echo "   • terzi --help"
echo "   • terzi list --help"
echo "   • terzi show --help"
echo "   • Check out the documentation in docs/"
echo ""
echo "💡 Pro tips:"
echo "   • Use tab completion for request names"
echo "   • Regular backup of your request collection"
echo "   • Use meaningful names for better organization"
echo "   • Check history regularly to track API usage"
echo "" 