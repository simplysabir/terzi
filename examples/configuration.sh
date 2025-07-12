#!/bin/bash

# Terzi Configuration Examples
# This script demonstrates configuration management with Terzi

echo "⚙️ Terzi Configuration Examples"
echo "==============================="
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
# 1. VIEWING CURRENT CONFIGURATION
# =============================================================================

echo "👀 1. VIEWING CURRENT CONFIGURATION"
echo "==================================="

echo "🔹 List all configuration settings:"
echo "Command: $TERZI config list"
$TERZI config list
echo ""

echo "🔹 Get a specific configuration value:"
echo "Command: $TERZI config get timeout"
$TERZI config get timeout
echo ""

echo "🔹 Get default output format:"
echo "Command: $TERZI config get output"
$TERZI config get output
echo ""

# =============================================================================
# 2. BASIC CONFIGURATION SETTINGS
# =============================================================================

echo "🔧 2. BASIC CONFIGURATION SETTINGS"
echo "==================================="

echo "🔹 Set default timeout (in seconds):"
echo "Command: $TERZI config set timeout 45"
$TERZI config set timeout 45
echo ""

echo "🔹 Set default output format:"
echo "Command: $TERZI config set output json"
$TERZI config set output json
echo ""

echo "🔹 Enable pretty printing by default:"
echo "Command: $TERZI config set pretty_print true"
$TERZI config set pretty_print true
echo ""

echo "🔹 Show headers by default:"
echo "Command: $TERZI config set show_headers true"
$TERZI config set show_headers true
echo ""

echo "🔹 Enable verbose output by default:"
echo "Command: $TERZI config set verbose false"
$TERZI config set verbose false
echo ""

# =============================================================================
# 3. NETWORK CONFIGURATION
# =============================================================================

echo "🌐 3. NETWORK CONFIGURATION"
echo "============================"

echo "🔹 Set custom User-Agent:"
echo "Command: $TERZI config set user_agent 'MyApp/1.0'"
$TERZI config set user_agent 'MyApp/1.0'
echo ""

echo "🔹 Enable following redirects by default:"
echo "Command: $TERZI config set follow_redirects true"
$TERZI config set follow_redirects true
echo ""

echo "🔹 Set maximum redirects:"
echo "Command: $TERZI config set max_redirects 5"
$TERZI config set max_redirects 5
echo ""

echo "🔹 Set connection timeout:"
echo "Command: $TERZI config set connection_timeout 10"
$TERZI config set connection_timeout 10
echo ""

echo "🔹 Set read timeout:"
echo "Command: $TERZI config set read_timeout 30"
$TERZI config set read_timeout 30
echo ""

echo "🔹 Enable keep-alive connections:"
echo "Command: $TERZI config set keep_alive true"
$TERZI config set keep_alive true
echo ""

# =============================================================================
# 4. SECURITY CONFIGURATION
# =============================================================================

echo "🔒 4. SECURITY CONFIGURATION"
echo "============================"

echo "🔹 Enable SSL verification:"
echo "Command: $TERZI config set verify_ssl true"
$TERZI config set verify_ssl true
echo ""

echo "🔹 Enable sensitive data masking:"
echo "Command: $TERZI config set mask_sensitive_data true"
$TERZI config set mask_sensitive_data true
echo ""

echo "🔹 Mask authentication headers:"
echo "Command: $TERZI config set mask_auth_headers true"
$TERZI config set mask_auth_headers true
echo ""

echo "🔹 Mask tokens in output:"
echo "Command: $TERZI config set mask_tokens true"
$TERZI config set mask_tokens true
echo ""

# =============================================================================
# 5. PROXY CONFIGURATION
# =============================================================================

echo "🔄 5. PROXY CONFIGURATION"
echo "========================="

echo "🔹 Set HTTP proxy (example):"
echo "Command: $TERZI config set proxy_url 'http://proxy.company.com:8080'"
echo "Note: Uncomment if you have a proxy server"
# $TERZI config set proxy_url 'http://proxy.company.com:8080'
echo ""

echo "🔹 Set proxy authentication (example):"
echo "Command: $TERZI config set proxy_auth 'username:password'"
echo "Note: Uncomment if your proxy requires authentication"
# $TERZI config set proxy_auth 'username:password'
echo ""

echo "🔹 To remove proxy settings:"
echo "Command: $TERZI config set proxy_url ''"
echo "Command: $TERZI config set proxy_auth ''"
echo ""

# =============================================================================
# 6. OUTPUT CONFIGURATION
# =============================================================================

echo "🎨 6. OUTPUT CONFIGURATION"
echo "=========================="

echo "🔹 Set default output format to YAML:"
echo "Command: $TERZI config set default_format yaml"
$TERZI config set default_format yaml
echo ""

echo "🔹 Enable colored output:"
echo "Command: $TERZI config set color_output true"
$TERZI config set color_output true
echo ""

echo "🔹 Set color scheme (dark/light):"
echo "Command: $TERZI config set color_scheme dark"
$TERZI config set color_scheme dark
echo ""

echo "🔹 Enable response time display:"
echo "Command: $TERZI config set show_response_time true"
$TERZI config set show_response_time true
echo ""

echo "🔹 Enable response size display:"
echo "Command: $TERZI config set show_response_size true"
$TERZI config set show_response_size true
echo ""

# =============================================================================
# 7. TESTING CONFIGURATION CHANGES
# =============================================================================

echo "🧪 7. TESTING CONFIGURATION CHANGES"
echo "==================================="

echo "🔹 Test with new timeout setting:"
echo "Command: $TERZI https://httpbin.org/delay/2"
$TERZI https://httpbin.org/delay/2
echo ""

echo "🔹 Test with new output format:"
echo "Command: $TERZI https://jsonplaceholder.typicode.com/posts/1"
$TERZI https://jsonplaceholder.typicode.com/posts/1
echo ""

echo "🔹 Test with headers shown:"
echo "Command: $TERZI https://httpbin.org/headers"
$TERZI https://httpbin.org/headers
echo ""

# =============================================================================
# 8. ENVIRONMENT-SPECIFIC CONFIGURATION
# =============================================================================

echo "🌍 8. ENVIRONMENT-SPECIFIC CONFIGURATION"
echo "========================================="

echo "🔹 Development environment settings:"
echo "Command: $TERZI config set base_url 'https://api-dev.example.com'"
$TERZI config set base_url 'https://api-dev.example.com'
echo ""

echo "Command: $TERZI config set default_timeout 60"
$TERZI config set default_timeout 60
echo ""

echo "Command: $TERZI config set verbose true"
$TERZI config set verbose true
echo ""

echo "🔹 Production environment settings (example):"
echo "# $TERZI config set base_url 'https://api.example.com'"
echo "# $TERZI config set default_timeout 30"
echo "# $TERZI config set verbose false"
echo ""

# =============================================================================
# 9. ADVANCED CONFIGURATION
# =============================================================================

echo "🎯 9. ADVANCED CONFIGURATION"
echo "============================"

echo "🔹 Set custom headers for all requests:"
echo "Command: $TERZI config set default_headers 'X-Client-Version: 1.0'"
$TERZI config set default_headers 'X-Client-Version: 1.0'
echo ""

echo "🔹 Set request retry configuration:"
echo "Command: $TERZI config set retry_attempts 3"
$TERZI config set retry_attempts 3
echo ""

echo "Command: $TERZI config set retry_delay 1000"
$TERZI config set retry_delay 1000
echo ""

echo "🔹 Enable compression:"
echo "Command: $TERZI config set enable_compression true"
$TERZI config set enable_compression true
echo ""

echo "🔹 Set history limit:"
echo "Command: $TERZI config set history_limit 100"
$TERZI config set history_limit 100
echo ""

# =============================================================================
# 10. CONFIGURATION PROFILES
# =============================================================================

echo "📋 10. CONFIGURATION PROFILES"
echo "=============================="

echo "🔹 Create different configuration profiles:"
echo "Note: Configuration profiles are managed through separate config files"
echo ""

echo "🔹 Default profile (current):"
echo "   • Location: ~/.config/terzi/config.toml"
echo "   • For general API testing"
echo ""

echo "🔹 Development profile example:"
echo "   • Copy config to ~/.config/terzi/dev.toml"
echo "   • Use: TERZI_CONFIG=dev terzi <command>"
echo ""

echo "🔹 Production profile example:"
echo "   • Copy config to ~/.config/terzi/prod.toml"
echo "   • Use: TERZI_CONFIG=prod terzi <command>"
echo ""

# =============================================================================
# 11. CONFIGURATION BACKUP AND RESTORE
# =============================================================================

echo "💾 11. CONFIGURATION BACKUP AND RESTORE"
echo "========================================"

echo "🔹 View current configuration location:"
echo "Linux/macOS: ~/.config/terzi/config.toml"
echo "Windows: %APPDATA%\\terzi\\config.toml"
echo ""

echo "🔹 Backup current configuration:"
echo "Command: cp ~/.config/terzi/config.toml ~/.config/terzi/config.backup.toml"
echo ""

echo "🔹 View configuration file:"
echo "Command: cat ~/.config/terzi/config.toml"
echo ""

# =============================================================================
# 12. RESETTING CONFIGURATION
# =============================================================================

echo "🔄 12. RESETTING CONFIGURATION"
echo "=============================="

echo "🔹 Reset all configuration to defaults:"
echo "Command: $TERZI config reset"
echo "Note: This will ask for confirmation"
echo ""

echo "🔹 Reset specific configuration values:"
echo "Command: $TERZI config set timeout 30"
echo "Command: $TERZI config set output auto"
echo "Command: $TERZI config set pretty_print true"
echo ""

# =============================================================================
# 13. CONFIGURATION VALIDATION
# =============================================================================

echo "✅ 13. CONFIGURATION VALIDATION"
echo "=============================="

echo "🔹 Test configuration with a simple request:"
echo "Command: $TERZI https://httpbin.org/get"
$TERZI https://httpbin.org/get
echo ""

echo "🔹 Verify timeout works:"
echo "Command: $TERZI config get timeout"
$TERZI config get timeout
echo ""

echo "🔹 Verify output format:"
echo "Command: $TERZI config get output"
$TERZI config get output
echo ""

# =============================================================================
# 14. CONFIGURATION BEST PRACTICES
# =============================================================================

echo "💡 14. CONFIGURATION BEST PRACTICES"
echo "===================================="

echo "🔹 Set reasonable defaults for your workflow:"
echo "   • Timeout: 30-60 seconds for most APIs"
echo "   • Output: 'auto' for flexible formatting"
echo "   • Pretty print: true for better readability"
echo ""

echo "🔹 Use environment-specific configurations:"
echo "   • Different timeouts for dev vs prod"
echo "   • Different base URLs for different environments"
echo "   • Different verbosity levels"
echo ""

echo "🔹 Security considerations:"
echo "   • Always verify SSL in production"
echo "   • Enable sensitive data masking"
echo "   • Use environment variables for secrets"
echo ""

echo "🔹 Performance considerations:"
echo "   • Enable compression for large responses"
echo "   • Set appropriate connection timeouts"
echo "   • Use keep-alive for multiple requests"
echo ""

echo "🔹 Regular maintenance:"
echo "   • Review and update configurations periodically"
echo "   • Backup your configuration files"
echo "   • Test configuration changes before deploying"
echo ""

# =============================================================================
# SUMMARY
# =============================================================================

echo "🎉 SUMMARY"
echo "=========="
echo ""
echo "✅ You've learned how to:"
echo "   • View and modify configuration settings"
echo "   • Configure network and security settings"
echo "   • Set up proxy and authentication defaults"
echo "   • Customize output formatting and behavior"
echo "   • Create environment-specific configurations"
echo "   • Backup and restore configuration files"
echo "   • Reset configuration to defaults"
echo "   • Apply configuration best practices"
echo ""
echo "🚀 Next steps:"
echo "   • Use interactive mode: terzi interactive"
echo ""
echo "📖 For more help:"
echo "   • terzi config --help"
echo "   • terzi config list"
echo "   • terzi config get <key>"
echo "   • Check out the documentation in docs/"
echo ""
echo "⚙️ Configuration reminders:"
echo "   • Configuration is stored in ~/.config/terzi/config.toml"
echo "   • Changes take effect immediately"
echo "   • Use 'terzi config list' to see all settings"
echo "   • Back up your configuration before major changes"
echo "" 