# Configuration

Complete guide to configuring Terzi for your workflow.

## 📋 Table of Contents

- [Overview](#overview)
- [Configuration File](#configuration-file)
- [Configuration Commands](#configuration-commands)
- [General Settings](#general-settings)
- [Network Settings](#network-settings)
- [Output Settings](#output-settings)
- [Security Settings](#security-settings)
- [Environment Variables](#environment-variables)
- [Profiles](#profiles)
- [Examples](#examples)

## Overview

Terzi can be configured to match your workflow and preferences. Configuration is stored in a TOML file and can be managed through CLI commands or by editing the file directly.

### Configuration Hierarchy

1. **Command-line arguments** (highest priority)
2. **Environment variables**
3. **Configuration file**
4. **Default values** (lowest priority)

## Configuration File

### Location

| OS | Path |
|----|------|
| **Linux** | `~/.config/terzi/config.toml` |
| **macOS** | `~/.config/terzi/config.toml` |
| **Windows** | `%APPDATA%\terzi\config.toml` |

### File Format

The configuration file uses TOML format:

```toml
[general]
default_timeout = 30
follow_redirects = false
max_redirects = 5

[network]
user_agent = "Terzi/1.0"
verify_ssl = true
keep_alive = true

[output]
default_format = "auto"
pretty_print = true
show_headers = false
color_output = true
```

## Configuration Commands

### View Configuration

```bash
# Show all configuration
terzi config list

# Get specific value
terzi config get timeout
terzi config get output.pretty_print
```

### Set Configuration

```bash
# Set a value
terzi config set timeout 60
terzi config set output.show_headers true

# Set nested values
terzi config set network.user_agent "MyApp/1.0"
```

### Reset Configuration

```bash
# Reset all settings to defaults
terzi config reset

# Reset specific section
terzi config reset network
```

## General Settings

### `default_timeout`
- **Type:** Integer
- **Default:** `30`
- **Description:** Default request timeout in seconds

```bash
terzi config set default_timeout 45
```

### `follow_redirects`
- **Type:** Boolean
- **Default:** `false`
- **Description:** Follow HTTP redirects by default

```bash
terzi config set follow_redirects true
```

### `max_redirects`
- **Type:** Integer
- **Default:** `5`
- **Description:** Maximum number of redirects to follow

```bash
terzi config set max_redirects 10
```

### `base_url`
- **Type:** String
- **Default:** `""`
- **Description:** Base URL to prepend to relative URLs

```bash
terzi config set base_url "https://api.example.com"
```

## Network Settings

### `user_agent`
- **Type:** String
- **Default:** `"Terzi/1.0"`
- **Description:** User-Agent header for requests

```bash
terzi config set user_agent "MyApp/2.0"
```

### `verify_ssl`
- **Type:** Boolean
- **Default:** `true`
- **Description:** Verify SSL certificates

```bash
terzi config set verify_ssl false  # Only for development
```

### `keep_alive`
- **Type:** Boolean
- **Default:** `true`
- **Description:** Use HTTP keep-alive connections

```bash
terzi config set keep_alive false
```

### `connection_timeout`
- **Type:** Integer
- **Default:** `10`
- **Description:** Connection timeout in seconds

```bash
terzi config set connection_timeout 15
```

### `read_timeout`
- **Type:** Integer
- **Default:** `30`
- **Description:** Read timeout in seconds

```bash
terzi config set read_timeout 45
```

### `proxy_url`
- **Type:** String
- **Default:** `""`
- **Description:** HTTP proxy URL

```bash
terzi config set proxy_url "http://proxy.company.com:8080"
```

### `proxy_auth`
- **Type:** String
- **Default:** `""`
- **Description:** Proxy authentication (username:password)

```bash
terzi config set proxy_auth "username:password"
```

## Output Settings

### `default_format`
- **Type:** String
- **Default:** `"auto"`
- **Options:** `auto`, `json`, `yaml`, `table`
- **Description:** Default output format

```bash
terzi config set default_format json
```

### `pretty_print`
- **Type:** Boolean
- **Default:** `true`
- **Description:** Pretty print JSON responses

```bash
terzi config set pretty_print false
```

### `show_headers`
- **Type:** Boolean
- **Default:** `false`
- **Description:** Show response headers by default

```bash
terzi config set show_headers true
```

### `color_output`
- **Type:** Boolean
- **Default:** `true`
- **Description:** Use colored output

```bash
terzi config set color_output false
```

### `color_scheme`
- **Type:** String
- **Default:** `"dark"`
- **Options:** `dark`, `light`
- **Description:** Color scheme for output

```bash
terzi config set color_scheme light
```

### `show_response_time`
- **Type:** Boolean
- **Default:** `true`
- **Description:** Show response time in output

```bash
terzi config set show_response_time false
```

### `show_response_size`
- **Type:** Boolean
- **Default:** `true`
- **Description:** Show response size in output

```bash
terzi config set show_response_size false
```

## Security Settings

### `mask_sensitive_data`
- **Type:** Boolean
- **Default:** `true`
- **Description:** Mask sensitive data in output

```bash
terzi config set mask_sensitive_data false
```

### `mask_auth_headers`
- **Type:** Boolean
- **Default:** `true`
- **Description:** Mask authentication headers

```bash
terzi config set mask_auth_headers false
```

### `mask_tokens`
- **Type:** Boolean
- **Default:** `true`
- **Description:** Mask tokens in output

```bash
terzi config set mask_tokens false
```

## Environment Variables

Environment variables override configuration file settings:

| Variable | Description | Example |
|----------|-------------|---------|
| `TERZI_CONFIG` | Configuration file path | `~/.config/terzi/custom.toml` |
| `TERZI_TIMEOUT` | Default timeout | `60` |
| `TERZI_OUTPUT` | Default output format | `json` |
| `TERZI_VERBOSE` | Enable verbose output | `true` |
| `TERZI_USER_AGENT` | User agent string | `MyApp/1.0` |
| `TERZI_PROXY_URL` | Proxy URL | `http://proxy:8080` |
| `HTTP_PROXY` | HTTP proxy (standard) | `http://proxy:8080` |
| `HTTPS_PROXY` | HTTPS proxy (standard) | `https://proxy:8080` |
| `NO_PROXY` | No proxy hosts | `localhost,127.0.0.1` |

### Example Usage

```bash
# Set environment variables
export TERZI_TIMEOUT=45
export TERZI_OUTPUT=json
export TERZI_VERBOSE=true

# Use with requests
terzi https://api.example.com/data
```

## Profiles

### Creating Profiles

Different configuration files for different environments:

```bash
# Development profile
cp ~/.config/terzi/config.toml ~/.config/terzi/dev.toml

# Production profile
cp ~/.config/terzi/config.toml ~/.config/terzi/prod.toml
```

### Using Profiles

```bash
# Use specific profile
TERZI_CONFIG=~/.config/terzi/dev.toml terzi https://api-dev.example.com/data

# Create alias for convenience
alias terzi-dev="TERZI_CONFIG=~/.config/terzi/dev.toml terzi"
alias terzi-prod="TERZI_CONFIG=~/.config/terzi/prod.toml terzi"
```

### Example Profile Configurations

#### Development Profile (`dev.toml`)
```toml
[general]
default_timeout = 60
follow_redirects = true
base_url = "https://api-dev.example.com"

[network]
verify_ssl = false
user_agent = "Terzi-Dev/1.0"

[output]
default_format = "json"
pretty_print = true
show_headers = true
color_output = true

[security]
mask_sensitive_data = false
```

#### Production Profile (`prod.toml`)
```toml
[general]
default_timeout = 30
follow_redirects = false
base_url = "https://api.example.com"

[network]
verify_ssl = true
user_agent = "Terzi-Prod/1.0"

[output]
default_format = "auto"
pretty_print = true
show_headers = false
color_output = true

[security]
mask_sensitive_data = true
mask_auth_headers = true
```

## Examples

### Basic Configuration

```bash
# Set up basic defaults
terzi config set default_timeout 45
terzi config set default_format json
terzi config set pretty_print true
terzi config set show_headers true

# Test configuration
terzi https://api.example.com/data
```

### Development Environment

```bash
# Configure for development
terzi config set base_url "https://api-dev.example.com"
terzi config set default_timeout 60
terzi config set verify_ssl false
terzi config set show_headers true
terzi config set mask_sensitive_data false

# Make requests (will use dev base URL)
terzi /users  # Expands to https://api-dev.example.com/users
```

### Production Environment

```bash
# Configure for production
terzi config set base_url "https://api.example.com"
terzi config set default_timeout 30
terzi config set verify_ssl true
terzi config set show_headers false
terzi config set mask_sensitive_data true
```

### Proxy Configuration

```bash
# Configure HTTP proxy
terzi config set proxy_url "http://proxy.company.com:8080"
terzi config set proxy_auth "username:password"

# Test proxy connection
terzi https://api.example.com/data
```

### Custom User Agent

```bash
# Set custom user agent
terzi config set user_agent "MyApp/2.0 (contact@example.com)"

# Verify in request
terzi https://httpbin.org/headers
```

### Output Customization

```bash
# Customize output
terzi config set default_format yaml
terzi config set pretty_print true
terzi config set show_headers true
terzi config set color_scheme light

# Test output
terzi https://api.example.com/data
```

### Security Settings

```bash
# Enable security features
terzi config set mask_sensitive_data true
terzi config set mask_auth_headers true
terzi config set mask_tokens true
terzi config set verify_ssl true

# Test with sensitive data
terzi -A "bearer:secret-token" https://api.example.com/protected
```

## Configuration Validation

### Check Configuration

```bash
# View all settings
terzi config list

# Check specific values
terzi config get timeout
terzi config get output.pretty_print
terzi config get network.user_agent
```

### Test Configuration

```bash
# Test with configuration
terzi https://httpbin.org/get

# Test specific settings
terzi -v https://httpbin.org/headers  # Test user agent
terzi -t 5 https://httpbin.org/delay/2  # Test timeout
```

## Troubleshooting

### Common Issues

#### Configuration Not Loading
```bash
# Check file location
echo $TERZI_CONFIG
ls -la ~/.config/terzi/

# Check file permissions
chmod 644 ~/.config/terzi/config.toml
```

#### Invalid Configuration
```bash
# Validate TOML syntax
terzi config list

# Reset if corrupted
terzi config reset
```

#### Environment Variables Not Working
```bash
# Check environment variables
env | grep TERZI

# Test with explicit variable
TERZI_TIMEOUT=45 terzi https://api.example.com/data
```

### Best Practices

1. **Backup Configuration**
   ```bash
   cp ~/.config/terzi/config.toml ~/.config/terzi/config.backup.toml
   ```

2. **Use Profiles for Different Environments**
   ```bash
   # Create environment-specific configs
   cp config.toml dev.toml
   cp config.toml prod.toml
   ```

3. **Test Configuration Changes**
   ```bash
   # Test after changes
   terzi config list
   terzi https://httpbin.org/get
   ```

4. **Document Custom Settings**
   ```bash
   # Add comments to config file
   # This is for development environment
   default_timeout = 60
   ```

5. **Use Environment Variables for Sensitive Data**
   ```bash
   # Don't store secrets in config file
   export TERZI_AUTH_TOKEN="secret-token"
   ```

---

For more configuration examples, see the [examples directory](../examples/configuration.sh) and [CLI reference](cli-reference.md). 