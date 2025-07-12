# CLI Reference

Complete command-line interface reference for Terzi.

## 📋 Table of Contents

- [Basic Syntax](#basic-syntax)
- [Global Options](#global-options)
- [Commands](#commands)
- [HTTP Methods](#http-methods)
- [Authentication](#authentication)
- [Request Bodies](#request-bodies)
- [Headers](#headers)
- [Output Control](#output-control)
- [Request Management](#request-management)
- [Configuration](#configuration)
- [Examples](#examples)

## Basic Syntax

```bash
terzi [OPTIONS] [URL] [COMMAND]
```

### Direct Request Mode
```bash
terzi [OPTIONS] <URL>
```

### Command Mode
```bash
terzi [COMMAND] [COMMAND_OPTIONS]
```

## Global Options

### Request Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--method <METHOD>` | `-m` | HTTP method | `GET` |
| `--header <HEADER>` | `-H` | Add header (key:value) | None |
| `--body <BODY>` | `-b` | Request body | None |
| `--json <JSON>` | `-j` | JSON body | None |
| `--form <FORM>` | `-f` | Form data (key=value) | None |
| `--auth <AUTH>` | `-A` | Authentication | None |
| `--timeout <SECONDS>` | `-t` | Request timeout | `30` |
| `--follow-redirects` | `-L` | Follow redirects | `false` |

### Output Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--output <FORMAT>` | `-o` | Output format | `auto` |
| `--include-headers` | `-i` | Include headers | `false` |
| `--verbose` | `-v` | Verbose output | `false` |
| `--silent` | `-S` | Silent mode | `false` |
| `--pretty` | `-p` | Pretty print | `true` |

### Request Management

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--save <NAME>` | | Save request | None |
| `--load <NAME>` | | Load request | None |

### Help & Version

| Option | Short | Description |
|--------|-------|-------------|
| `--help` | `-h` | Show help |
| `--version` | `-V` | Show version |

## Commands

### `interactive`
Launch interactive mode for guided request building.

```bash
terzi interactive
```

**Features:**
- Step-by-step request building
- Fuzzy search for saved requests
- Request history browser
- Settings management

### `list`
List saved requests.

```bash
terzi list [OPTIONS]
```

**Options:**
- `--filter <PATTERN>` - Filter requests by pattern

**Examples:**
```bash
terzi list                    # List all requests
terzi list --filter "api"     # Filter by "api"
terzi list --filter "POST"    # Filter by method
```

### `show`
Show details of a saved request.

```bash
terzi show <NAME>
```

**Examples:**
```bash
terzi show "my-request"       # Show request details
```

### `edit`
Edit a saved request interactively.

```bash
terzi edit <NAME>
```

**Examples:**
```bash
terzi edit "my-request"       # Edit request
```

### `delete`
Delete a saved request.

```bash
terzi delete <NAME>
```

**Examples:**
```bash
terzi delete "my-request"     # Delete request (with confirmation)
```

### `history`
Show request history.

```bash
terzi history [OPTIONS]
```

**Options:**
- `--limit <NUMBER>` - Number of entries to show (default: 10)

**Examples:**
```bash
terzi history                 # Show last 10 requests
terzi history --limit 20      # Show last 20 requests
```

### `config`
Manage configuration.

```bash
terzi config <SUBCOMMAND>
```

**Subcommands:**
- `list` - Show all configuration
- `get <KEY>` - Get configuration value
- `set <KEY> <VALUE>` - Set configuration value
- `reset` - Reset to defaults

**Examples:**
```bash
terzi config list                    # Show all config
terzi config get timeout             # Get timeout value
terzi config set timeout 60          # Set timeout to 60s
terzi config reset                   # Reset all config
```

### `export`
Export saved requests.

```bash
terzi export [OPTIONS]
```

**Options:**
- `--output <FILE>` - Output file path
- `--format <FORMAT>` - Export format (json, yaml)

**Examples:**
```bash
terzi export --output requests.json          # Export to JSON
terzi export --format yaml --output req.yaml # Export to YAML
```

### `version`
Show version information.

```bash
terzi version
```

## HTTP Methods

### Supported Methods

| Method | Description |
|--------|-------------|
| `GET` | Retrieve data |
| `POST` | Create/submit data |
| `PUT` | Update/replace data |
| `PATCH` | Partially update data |
| `DELETE` | Delete data |
| `HEAD` | Get headers only |
| `OPTIONS` | Get allowed methods |

### Examples

```bash
# GET request (default)
terzi https://api.example.com/users

# POST request
terzi -m POST https://api.example.com/users

# PUT request
terzi -m PUT https://api.example.com/users/1

# DELETE request
terzi -m DELETE https://api.example.com/users/1
```

## Authentication

### Authentication Types

| Type | Format | Description |
|------|---------|-------------|
| Bearer | `bearer:token` | Bearer token |
| Basic | `basic:user:pass` | Basic authentication |
| API Key | `apikey:header:key` | API key in header |

### Examples

```bash
# Bearer token
terzi -A "bearer:your-token" https://api.example.com/protected

# Basic auth
terzi -A "basic:username:password" https://api.example.com/protected

# API key in X-API-Key header
terzi -A "apikey:X-API-Key:your-key" https://api.example.com/protected

# API key in custom header
terzi -A "apikey:Authorization:Bearer your-key" https://api.example.com/protected
```

## Request Bodies

### Body Types

| Option | Description | Content-Type |
|--------|-------------|--------------|
| `-j, --json` | JSON data | `application/json` |
| `-f, --form` | Form data | `application/x-www-form-urlencoded` |
| `-b, --body` | Raw body | As specified |

### Examples

```bash
# JSON body
terzi -m POST -j '{"name": "John", "age": 30}' https://api.example.com/users

# Form data
terzi -m POST -f name=John -f age=30 https://api.example.com/users

# Raw body
terzi -m POST -b "Plain text content" https://api.example.com/data

# Raw body with custom content type
terzi -m POST -H "Content-Type: text/plain" -b "Text data" https://api.example.com/data
```

## Headers

### Adding Headers

```bash
# Single header
terzi -H "Authorization: Bearer token" https://api.example.com/data

# Multiple headers
terzi -H "Accept: application/json" -H "User-Agent: MyApp" https://api.example.com/data

# Common headers
terzi -H "Content-Type: application/json" https://api.example.com/data
```

### Common Header Examples

```bash
# Authorization
terzi -H "Authorization: Bearer your-token" <url>
terzi -H "Authorization: Basic base64-encoded" <url>

# Content Type
terzi -H "Content-Type: application/json" <url>
terzi -H "Content-Type: application/xml" <url>

# User Agent
terzi -H "User-Agent: MyApp/1.0" <url>

# Accept
terzi -H "Accept: application/json" <url>
terzi -H "Accept: */*" <url>
```

## Output Control

### Output Formats

| Format | Description |
|--------|-------------|
| `auto` | Auto-detect format |
| `json` | Force JSON output |
| `yaml` | Convert to YAML |
| `table` | Tabular format |

### Output Options

```bash
# Different output formats
terzi -o json https://api.example.com/data
terzi -o yaml https://api.example.com/data
terzi -o table https://api.example.com/data

# Include response headers
terzi -i https://api.example.com/data

# Verbose output
terzi -v https://api.example.com/data

# Silent mode (no formatting)
terzi -S https://api.example.com/data

# Disable pretty printing
terzi -p false https://api.example.com/data
```

## Request Management

### Saving Requests

```bash
# Save a simple request
terzi --save "get-users" https://api.example.com/users

# Save a complex request
terzi --save "create-user" -m POST -j '{"name":"John"}' -A "bearer:token" https://api.example.com/users
```

### Loading Requests

```bash
# Load and execute a saved request
terzi --load "get-users"

# Load with output modifications
terzi --load "get-users" -o yaml -v
```

## Configuration

### Configuration Keys

| Key | Description | Default |
|-----|-------------|---------|
| `timeout` | Default timeout (seconds) | `30` |
| `output` | Default output format | `auto` |
| `pretty_print` | Pretty print responses | `true` |
| `show_headers` | Show headers by default | `false` |
| `verbose` | Verbose output | `false` |
| `follow_redirects` | Follow redirects | `false` |
| `user_agent` | User agent string | `Terzi/1.0` |
| `verify_ssl` | Verify SSL certificates | `true` |
| `mask_sensitive_data` | Mask sensitive data | `true` |

### Configuration Examples

```bash
# View all configuration
terzi config list

# Get specific values
terzi config get timeout
terzi config get output

# Set configuration
terzi config set timeout 60
terzi config set output json
terzi config set pretty_print true
terzi config set show_headers true

# Reset configuration
terzi config reset
```

## Examples

### Basic Usage

```bash
# Simple GET request
terzi https://api.github.com/users/octocat

# GET with query parameters
terzi 'https://api.github.com/search/users?q=octocat'

# POST with JSON
terzi -m POST -j '{"title": "Test"}' https://jsonplaceholder.typicode.com/posts

# PUT with authentication
terzi -m PUT -A "bearer:token" -j '{"name": "Updated"}' https://api.example.com/users/1
```

### Advanced Usage

```bash
# Complex request with all options
terzi -m POST \
      -H "Content-Type: application/json" \
      -H "User-Agent: MyApp/1.0" \
      -A "bearer:secret-token" \
      -j '{"name": "John", "email": "john@example.com"}' \
      -t 60 \
      -L \
      -v \
      --save "create-user" \
      https://api.example.com/users

# Load and modify saved request
terzi --load "create-user" -o yaml -i

# Export requests
terzi export --output my-requests.json
```

### Error Handling

```bash
# Test with invalid URL
terzi invalid-url
# Error: Invalid URL format

# Test with timeout
terzi -t 1 https://httpbin.org/delay/2
# Error: Request timed out

# Test with invalid JSON
terzi -m POST -j 'invalid json' https://httpbin.org/post
# Error: Invalid JSON syntax
```

### Configuration Workflow

```bash
# Set up default configuration
terzi config set timeout 45
terzi config set output json
terzi config set pretty_print true
terzi config set show_headers true

# Make requests with defaults
terzi https://api.example.com/data
# Uses configured defaults

# Override defaults
terzi -t 30 -o yaml https://api.example.com/data
# Uses overridden values
```

## Exit Codes

| Code | Description |
|------|-------------|
| `0` | Success |
| `1` | General error |
| `2` | Invalid arguments |
| `3` | Network error |
| `4` | Authentication error |
| `5` | Configuration error |

## Environment Variables

| Variable | Description |
|----------|-------------|
| `TERZI_CONFIG` | Configuration file path |
| `TERZI_TIMEOUT` | Default timeout |
| `TERZI_OUTPUT` | Default output format |
| `TERZI_VERBOSE` | Enable verbose output |

## Shell Completion

Generate shell completion scripts:

```bash
# Bash
terzi completion bash > /etc/bash_completion.d/terzi

# Zsh
terzi completion zsh > /usr/local/share/zsh/site-functions/_terzi

# Fish
terzi completion fish > ~/.config/fish/completions/terzi.fish
```

## Tips & Tricks

### 1. Use Configuration for Defaults
Set up your preferred defaults instead of repeating options:

```bash
terzi config set timeout 45
terzi config set output json
terzi config set pretty_print true
```

### 2. Save Complex Requests
Save frequently used complex requests:

```bash
terzi --save "github-user" -A "bearer:$GITHUB_TOKEN" https://api.github.com/user
```

### 3. Use Environment Variables
Use environment variables for sensitive data:

```bash
export GITHUB_TOKEN="your-token"
terzi -A "bearer:$GITHUB_TOKEN" https://api.github.com/user
```

### 4. Pipe Output
Use with other tools:

```bash
terzi https://api.example.com/users | jq '.[] | .name'
```

### 5. Batch Operations
Use shell loops for batch operations:

```bash
for id in 1 2 3; do
  terzi https://api.example.com/users/$id
done
```

---

For more examples and detailed usage, see the [examples directory](../examples/) and other documentation files. 