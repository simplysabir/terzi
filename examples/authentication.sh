#!/bin/bash

# Terzi Authentication Examples
# This script demonstrates various authentication methods with Terzi

echo "🔐 Terzi Authentication Examples"
echo "================================="
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
# 1. BEARER TOKEN AUTHENTICATION
# =============================================================================

echo "🎫 1. BEARER TOKEN AUTHENTICATION"
echo "=================================="

echo "🔹 Using --auth flag with bearer token:"
echo "Command: $TERZI -A 'bearer:ghp_example_token' https://api.github.com/user"
echo "Note: Replace 'ghp_example_token' with your actual GitHub token"
echo ""

echo "🔹 Using header directly:"
echo "Command: $TERZI -H 'Authorization: Bearer ghp_example_token' https://api.github.com/user"
echo ""

echo "🔹 Testing with HTTPBin (shows headers):"
echo "Command: $TERZI -A 'bearer:my-secret-token' https://httpbin.org/headers"
$TERZI -A 'bearer:my-secret-token' https://httpbin.org/headers
echo ""

# =============================================================================
# 2. BASIC AUTHENTICATION
# =============================================================================

echo "🔑 2. BASIC AUTHENTICATION"
echo "=========================="

echo "🔹 Using --auth flag with basic auth:"
echo "Command: $TERZI -A 'basic:username:password' https://httpbin.org/basic-auth/username/password"
$TERZI -A 'basic:username:password' https://httpbin.org/basic-auth/username/password
echo ""

echo "🔹 Using header directly (base64 encoded):"
echo "Command: $TERZI -H 'Authorization: Basic dXNlcm5hbWU6cGFzc3dvcmQ=' https://httpbin.org/basic-auth/username/password"
$TERZI -H 'Authorization: Basic dXNlcm5hbWU6cGFzc3dvcmQ=' https://httpbin.org/basic-auth/username/password
echo ""

echo "🔹 Testing with wrong credentials (should fail):"
echo "Command: $TERZI -A 'basic:wrong:credentials' https://httpbin.org/basic-auth/username/password"
$TERZI -A 'basic:wrong:credentials' https://httpbin.org/basic-auth/username/password
echo ""

# =============================================================================
# 3. API KEY AUTHENTICATION
# =============================================================================

echo "🔐 3. API KEY AUTHENTICATION"
echo "============================"

echo "🔹 API Key in header (X-API-Key):"
echo "Command: $TERZI -A 'apikey:X-API-Key:my-api-key-123' https://httpbin.org/headers"
$TERZI -A 'apikey:X-API-Key:my-api-key-123' https://httpbin.org/headers
echo ""

echo "🔹 API Key in custom header:"
echo "Command: $TERZI -A 'apikey:Authorization:Bearer my-api-key' https://httpbin.org/headers"
$TERZI -A 'apikey:Authorization:Bearer my-api-key' https://httpbin.org/headers
echo ""

echo "🔹 API Key using direct header:"
echo "Command: $TERZI -H 'X-API-Key: my-secret-api-key' https://httpbin.org/headers"
$TERZI -H 'X-API-Key: my-secret-api-key' https://httpbin.org/headers
echo ""

echo "🔹 Multiple API keys:"
echo "Command: $TERZI -H 'X-API-Key: key1' -H 'X-Secondary-Key: key2' https://httpbin.org/headers"
$TERZI -H 'X-API-Key: key1' -H 'X-Secondary-Key: key2' https://httpbin.org/headers
echo ""

# =============================================================================
# 4. AUTHENTICATION WITH DIFFERENT APIS
# =============================================================================

echo "🌐 4. REAL-WORLD API AUTHENTICATION"
echo "==================================="

echo "🔹 GitHub API (requires personal access token):"
echo "Command: $TERZI -A 'bearer:YOUR_GITHUB_TOKEN' https://api.github.com/user"
echo "Note: Get your token from https://github.com/settings/tokens"
echo ""

echo "🔹 GitHub API - List repositories:"
echo "Command: $TERZI -A 'bearer:YOUR_GITHUB_TOKEN' https://api.github.com/user/repos"
echo ""

echo "🔹 JSONPlaceholder with fake auth (for testing):"
echo "Command: $TERZI -H 'Authorization: Bearer fake-token' https://jsonplaceholder.typicode.com/posts"
$TERZI -H 'Authorization: Bearer fake-token' https://jsonplaceholder.typicode.com/posts
echo ""

# =============================================================================
# 5. AUTHENTICATION WITH POST REQUESTS
# =============================================================================

echo "📤 5. AUTHENTICATION WITH POST REQUESTS"
echo "========================================"

echo "🔹 POST with Bearer token:"
echo "Command: $TERZI -m POST -A 'bearer:my-token' -j '{\"title\": \"New Post\"}' https://httpbin.org/post"
$TERZI -m POST -A 'bearer:my-token' -j '{"title": "New Post"}' https://httpbin.org/post
echo ""

echo "🔹 POST with Basic auth:"
echo "Command: $TERZI -m POST -A 'basic:user:pass' -j '{\"data\": \"test\"}' https://httpbin.org/post"
$TERZI -m POST -A 'basic:user:pass' -j '{"data": "test"}' https://httpbin.org/post
echo ""

echo "🔹 POST with API key and form data:"
echo "Command: $TERZI -m POST -H 'X-API-Key: secret' -f name=John -f email=john@example.com https://httpbin.org/post"
$TERZI -m POST -H 'X-API-Key: secret' -f name=John -f email=john@example.com https://httpbin.org/post
echo ""

# =============================================================================
# 6. AUTHENTICATION STORAGE AND REUSE
# =============================================================================

echo "💾 6. AUTHENTICATION STORAGE AND REUSE"
echo "======================================"

echo "🔹 Save authenticated request for reuse:"
echo "Command: $TERZI --save 'github-user' -A 'bearer:YOUR_TOKEN' https://api.github.com/user"
echo "This saves the request with authentication for later use"
echo ""

echo "🔹 Load and execute saved authenticated request:"
echo "Command: $TERZI --load 'github-user'"
echo "This loads and executes the previously saved request"
echo ""

# =============================================================================
# 7. ENVIRONMENT VARIABLES FOR AUTHENTICATION
# =============================================================================

echo "🌍 7. ENVIRONMENT VARIABLES FOR AUTHENTICATION"
echo "=============================================="

echo "🔹 Using environment variables for tokens:"
echo "export GITHUB_TOKEN=your_token_here"
echo "export API_KEY=your_api_key_here"
echo ""

echo "🔹 Then use in requests:"
echo "Command: $TERZI -A 'bearer:\$GITHUB_TOKEN' https://api.github.com/user"
echo "Command: $TERZI -H 'X-API-Key: \$API_KEY' https://api.example.com/data"
echo ""

# =============================================================================
# 8. AUTHENTICATION ERROR HANDLING
# =============================================================================

echo "❌ 8. AUTHENTICATION ERROR HANDLING"
echo "==================================="

echo "🔹 Invalid token (should return 401):"
echo "Command: $TERZI -A 'bearer:invalid-token' https://api.github.com/user"
echo "Expected: 401 Unauthorized"
echo ""

echo "🔹 Missing authentication (should return 401):"
echo "Command: $TERZI https://httpbin.org/basic-auth/user/pass"
$TERZI https://httpbin.org/basic-auth/user/pass
echo ""

echo "🔹 Wrong authentication method:"
echo "Command: $TERZI -A 'bearer:token' https://httpbin.org/basic-auth/user/pass"
$TERZI -A 'bearer:token' https://httpbin.org/basic-auth/user/pass
echo ""

# =============================================================================
# 9. ADVANCED AUTHENTICATION SCENARIOS
# =============================================================================

echo "🎯 9. ADVANCED AUTHENTICATION SCENARIOS"
echo "======================================="

echo "🔹 Multiple auth headers:"
echo "Command: $TERZI -H 'Authorization: Bearer token1' -H 'X-API-Key: key1' https://httpbin.org/headers"
$TERZI -H 'Authorization: Bearer token1' -H 'X-API-Key: key1' https://httpbin.org/headers
echo ""

echo "🔹 Custom auth header format:"
echo "Command: $TERZI -H 'X-Auth-Token: custom-token-format' https://httpbin.org/headers"
$TERZI -H 'X-Auth-Token: custom-token-format' https://httpbin.org/headers
echo ""

echo "🔹 Session-based auth (cookies):"
echo "Command: $TERZI -H 'Cookie: session=abc123; token=xyz789' https://httpbin.org/headers"
$TERZI -H 'Cookie: session=abc123; token=xyz789' https://httpbin.org/headers
echo ""

# =============================================================================
# 10. AUTHENTICATION BEST PRACTICES
# =============================================================================

echo "✅ 10. AUTHENTICATION BEST PRACTICES"
echo "===================================="

echo "🔹 Use environment variables for sensitive tokens:"
echo "   export GITHUB_TOKEN=your_token"
echo "   $TERZI -A 'bearer:\$GITHUB_TOKEN' https://api.github.com/user"
echo ""

echo "🔹 Save authenticated requests to avoid repeating tokens:"
echo "   $TERZI --save 'api-call' -A 'bearer:token' https://api.example.com/endpoint"
echo "   $TERZI --load 'api-call'"
echo ""

echo "🔹 Use configuration for default auth:"
echo "   terzi config set default_auth 'bearer:token'"
echo ""

echo "🔹 Be careful with command history - tokens may be logged"
echo "   Use environment variables or saved requests instead"
echo ""

# =============================================================================
# SUMMARY
# =============================================================================

echo "🎉 SUMMARY"
echo "=========="
echo ""
echo "✅ You've learned how to:"
echo "   • Use Bearer token authentication"
echo "   • Use Basic authentication (username:password)"
echo "   • Use API key authentication (various header formats)"
echo "   • Handle authentication with different HTTP methods"
echo "   • Save and reuse authenticated requests"
echo "   • Use environment variables for tokens"
echo "   • Handle authentication errors gracefully"
echo "   • Apply authentication best practices"
echo ""
echo "🚀 Next steps:"
echo "   • Try request management: ./examples/request-management.sh"
echo "   • Learn about configuration: ./examples/configuration.sh"
echo "   • Explore real-world APIs: ./examples/real-world-apis.sh"
echo "   • Use interactive mode: terzi interactive"
echo ""
echo "📖 For more help:"
echo "   • terzi --help"
echo "   • terzi config --help"
echo "   • Check out the documentation in docs/"
echo ""
echo "🔐 Security reminders:"
echo "   • Never commit tokens to version control"
echo "   • Use environment variables for sensitive data"
echo "   • Rotate tokens regularly"
echo "   • Use least-privilege principle for API keys"
echo "" 