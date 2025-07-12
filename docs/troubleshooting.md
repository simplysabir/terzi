# Troubleshooting

Common issues and solutions for Terzi.

## 📋 Table of Contents

- [Installation Issues](#installation-issues)
- [Configuration Problems](#configuration-problems)
- [Network Issues](#network-issues)
- [Authentication Problems](#authentication-problems)
- [Request Failures](#request-failures)
- [Output Issues](#output-issues)
- [Performance Problems](#performance-problems)
- [Common Error Messages](#common-error-messages)
- [Debugging Tips](#debugging-tips)
- [Getting Help](#getting-help)

## Installation Issues

### Cargo Build Fails

**Problem:** Build fails with compilation errors.

**Solution:**
```bash
# Update Rust to latest version
rustup update

# Clean and rebuild
cargo clean
cargo build --release

# Check system dependencies
# On Ubuntu/Debian:
sudo apt update
sudo apt install build-essential pkg-config libssl-dev

# On macOS:
xcode-select --install
```

### Binary Not Found

**Problem:** `terzi: command not found`

**Solution:**
```bash
# Add to PATH
export PATH=$PATH:$(pwd)/target/release

# Or create symlink
sudo ln -s $(pwd)/target/release/terzi /usr/local/bin/terzi

# Or use cargo install
cargo install --path .
```

## Configuration Problems

### Configuration Not Loading

**Problem:** Configuration changes don't take effect.

**Diagnosis:**
```bash
# Check config file location
terzi config list

# Check file permissions
ls -la ~/.config/terzi/config.toml

# Check for typos
cat ~/.config/terzi/config.toml
```

**Solution:**
```bash
# Fix permissions
chmod 644 ~/.config/terzi/config.toml

# Validate and reset if needed
terzi config reset
```

### Invalid Configuration Syntax

**Problem:** TOML syntax errors in configuration.

**Diagnosis:**
```bash
# Check for syntax errors
terzi config list
```

**Solution:**
```bash
# Reset to defaults
terzi config reset

# Or manually fix the file
nano ~/.config/terzi/config.toml
```

### Environment Variables Not Working

**Problem:** Environment variables don't override config.

**Diagnosis:**
```bash
# Check environment variables
env | grep TERZI

# Test with explicit variable
TERZI_TIMEOUT=45 terzi --help
```

**Solution:**
```bash
# Set variables in shell profile
echo 'export TERZI_TIMEOUT=45' >> ~/.bashrc
source ~/.bashrc

# Or use full variable names
export TERZI_CONFIG=~/.config/terzi/dev.toml
```

## Network Issues

### Connection Timeouts

**Problem:** Requests timeout frequently.

**Diagnosis:**
```bash
# Test with longer timeout
terzi -t 60 https://api.example.com/data

# Test network connectivity
ping api.example.com
curl -I https://api.example.com/data
```

**Solution:**
```bash
# Increase default timeout
terzi config set default_timeout 60

# Use per-request timeout
terzi -t 120 https://slow-api.example.com/data
```

### DNS Resolution Failures

**Problem:** "Name resolution failed" errors.

**Diagnosis:**
```bash
# Test DNS resolution
nslookup api.example.com
dig api.example.com
```

**Solution:**
```bash
# Use different DNS server
echo "nameserver 8.8.8.8" | sudo tee -a /etc/resolv.conf

# Or use IP address directly
terzi https://192.168.1.100/api/data
```

### Proxy Issues

**Problem:** Requests fail behind corporate proxy.

**Diagnosis:**
```bash
# Check proxy settings
echo $HTTP_PROXY
echo $HTTPS_PROXY

# Test proxy connection
curl -I --proxy $HTTP_PROXY https://api.example.com
```

**Solution:**
```bash
# Configure proxy in Terzi
terzi config set proxy_url "http://proxy.company.com:8080"
terzi config set proxy_auth "username:password"

# Or use environment variables
export HTTP_PROXY=http://proxy.company.com:8080
export HTTPS_PROXY=https://proxy.company.com:8080
```

### SSL Certificate Issues

**Problem:** SSL certificate verification failures.

**Diagnosis:**
```bash
# Test SSL connection
openssl s_client -connect api.example.com:443

# Check certificate
curl -I https://api.example.com
```

**Solution:**
```bash
# For development only - disable SSL verification
terzi config set verify_ssl false
terzi https://api.example.com/data

# For production - update certificates
sudo apt update && sudo apt install ca-certificates
```

## Authentication Problems

### Invalid Token Errors

**Problem:** "401 Unauthorized" or "403 Forbidden" errors.

**Diagnosis:**
```bash
# Check token format
echo $YOUR_TOKEN | wc -c

# Test token manually
curl -H "Authorization: Bearer $YOUR_TOKEN" https://api.example.com/protected
```

**Solution:**
```bash
# Check token format
terzi -A "bearer:correct-token-here" https://api.example.com/protected

# Use environment variable
export API_TOKEN="your-token"
terzi -A "bearer:$API_TOKEN" https://api.example.com/protected
```

### Basic Auth Issues

**Problem:** Basic authentication not working.

**Diagnosis:**
```bash
# Test with curl
curl -u username:password https://api.example.com/protected
```

**Solution:**
```bash
# Use correct format
terzi -A "basic:username:password" https://api.example.com/protected

# Check for special characters
terzi -A "basic:user%40domain.com:pass%21" https://api.example.com/protected
```

### API Key Problems

**Problem:** API key authentication failing.

**Diagnosis:**
```bash
# Check header name
curl -H "X-API-Key: your-key" https://api.example.com/protected
```

**Solution:**
```bash
# Use correct header name
terzi -A "apikey:X-API-Key:your-key" https://api.example.com/protected
terzi -A "apikey:Authorization:Bearer your-key" https://api.example.com/protected
```

## Request Failures

### Malformed URLs

**Problem:** "Invalid URL" errors.

**Diagnosis:**
```bash
# Test URL format
terzi https://api.example.com/data  # Good
terzi api.example.com/data          # Bad - missing protocol
```

**Solution:**
```bash
# Always include protocol
terzi https://api.example.com/data

# Use base URL configuration
terzi config set base_url "https://api.example.com"
terzi /data  # Will expand to https://api.example.com/data
```

### Invalid JSON

**Problem:** JSON parsing errors.

**Diagnosis:**
```bash
# Validate JSON
echo '{"key": "value"}' | jq .  # Valid
echo '{"key": "value"'  | jq .  # Invalid
```

**Solution:**
```bash
# Use proper JSON format
terzi -m POST -j '{"key": "value"}' https://api.example.com/data

# Use single quotes for JSON
terzi -m POST -j '{"name": "John", "age": 30}' https://api.example.com/users
```

### Form Data Issues

**Problem:** Form data not being sent correctly.

**Diagnosis:**
```bash
# Check form data format
terzi -m POST -f name=John -f age=30 https://httpbin.org/post
```

**Solution:**
```bash
# Use correct format
terzi -m POST -f "name=John Doe" -f "email=john@example.com" https://api.example.com/users

# URL encode special characters
terzi -m POST -f "name=John%20Doe" -f "email=john%40example.com" https://api.example.com/users
```

## Output Issues

### Garbled Output

**Problem:** Output contains strange characters or formatting.

**Diagnosis:**
```bash
# Check terminal encoding
echo $LANG
locale charmap
```

**Solution:**
```bash
# Set proper locale
export LANG=en_US.UTF-8

# Use plain output
terzi -o json https://api.example.com/data

# Disable colors
terzi config set color_output false
```

### Missing Response Data

**Problem:** Response appears empty or truncated.

**Diagnosis:**
```bash
# Check response with verbose output
terzi -v https://api.example.com/data

# Include headers
terzi -i https://api.example.com/data
```

**Solution:**
```bash
# Check content type
terzi -i https://api.example.com/data | grep -i content-type

# Use raw output
terzi -S https://api.example.com/data
```

### Pretty Print Issues

**Problem:** JSON not formatted nicely.

**Diagnosis:**
```bash
# Check pretty print setting
terzi config get pretty_print
```

**Solution:**
```bash
# Enable pretty printing
terzi config set pretty_print true
terzi -p https://api.example.com/data
```

## Performance Problems

### Slow Requests

**Problem:** Requests taking too long.

**Diagnosis:**
```bash
# Test with verbose output
terzi -v https://api.example.com/data

# Check network latency
ping api.example.com
```

**Solution:**
```bash
# Increase timeout
terzi -t 120 https://api.example.com/data

# Use keep-alive
terzi config set keep_alive true

# Disable SSL verification (development only)
terzi config set verify_ssl false
```

### Memory Usage

**Problem:** High memory usage with large responses.

**Diagnosis:**
```bash
# Monitor memory usage
top -p $(pgrep terzi)
```

**Solution:**
```bash
# Stream large responses
terzi -S https://api.example.com/large-data > output.json

# Limit response size
terzi https://api.example.com/data | head -n 100
```

## Common Error Messages

### "Request timed out"

**Cause:** Network timeout or slow server.

**Solution:**
```bash
# Increase timeout
terzi -t 60 https://api.example.com/data

# Set default timeout
terzi config set default_timeout 60
```

### "Invalid URL format"

**Cause:** URL missing protocol or malformed.

**Solution:**
```bash
# Include protocol
terzi https://api.example.com/data

# Check URL format
terzi 'https://api.example.com/search?q=test'
```

### "SSL certificate verification failed"

**Cause:** Invalid or self-signed certificate.

**Solution:**
```bash
# For development only
terzi config set verify_ssl false

# For production, check certificate
openssl s_client -connect api.example.com:443
```

### "Name resolution failed"

**Cause:** DNS resolution issues.

**Solution:**
```bash
# Check DNS
nslookup api.example.com

# Use different DNS
echo "nameserver 8.8.8.8" | sudo tee -a /etc/resolv.conf
```

### "Connection refused"

**Cause:** Server not running or firewall blocking.

**Solution:**
```bash
# Check if server is running
telnet api.example.com 443

# Check firewall
sudo ufw status
```

## Debugging Tips

### Enable Verbose Output

```bash
# See detailed request/response information
terzi -v https://api.example.com/data

# Include headers
terzi -i https://api.example.com/data

# Show all details
terzi -v -i https://api.example.com/data
```

### Test with HTTPBin

```bash
# Test various scenarios
terzi https://httpbin.org/get                    # GET request
terzi -m POST -j '{"test": true}' https://httpbin.org/post  # POST request
terzi -A "bearer:test" https://httpbin.org/headers          # Headers
terzi -t 2 https://httpbin.org/delay/5                     # Timeout
```

### Use curl for Comparison

```bash
# Compare with curl
curl -v https://api.example.com/data
terzi -v https://api.example.com/data

# Same request with different tools
curl -H "Authorization: Bearer token" https://api.example.com/protected
terzi -A "bearer:token" https://api.example.com/protected
```

### Check Network Connectivity

```bash
# Basic connectivity
ping api.example.com

# HTTP connectivity
curl -I https://api.example.com

# Port connectivity
telnet api.example.com 443
```

### Configuration Debugging

```bash
# Check all settings
terzi config list

# Test specific setting
terzi config get timeout
terzi -t 5 https://httpbin.org/delay/2

# Reset if needed
terzi config reset
```

## Getting Help

### Collect Information

When reporting issues, include:

1. **Terzi version**
   ```bash
   terzi --version
   ```

2. **Operating system**
   ```bash
   uname -a
   ```

3. **Configuration**
   ```bash
   terzi config list
   ```

4. **Error message**
   ```bash
   terzi -v https://api.example.com/data 2>&1 | tee error.log
   ```

5. **Working curl command**
   ```bash
   curl -v https://api.example.com/data
   ```

### Create Minimal Reproduction

```bash
# Create minimal example
terzi https://httpbin.org/get  # If this works
terzi https://your-api.com/endpoint  # But this doesn't

# Show the difference
terzi -v https://httpbin.org/get
terzi -v https://your-api.com/endpoint
```

### Check Documentation

1. [CLI Reference](cli-reference.md) - Command options
2. [Configuration](configuration.md) - Settings and setup
3. [Examples](../examples/) - Working examples

### Community Support

- **GitHub Issues** - [https://github.com/simplysabir/terzi/issues](https://github.com/simplysabir/terzi/issues)
- **GitHub Discussions** - [https://github.com/simplysabir/terzi/discussions](https://github.com/simplysabir/terzi/discussions)
- **Email** - simplysabir@gmail.com

### When Creating Issues

1. **Search existing issues** first
2. **Use issue templates** if available
3. **Include reproduction steps**
4. **Provide configuration and environment details**
5. **Include error messages and logs**

---

For more help, see the [documentation](README.md) or [examples](../examples/). 