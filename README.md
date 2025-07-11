# Terzi

> A blazingly fast, deadly efficient CLI API client - eliminate API complexity

<div align="center">

<!-- [![Crates.io](https://img.shields.io/crates/v/terzi.svg)](https://crates.io/crates/terzi)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Build Status](https://github.com/yourusername/terzi/workflows/CI/badge.svg)](https://github.com/simplysabir/terzi/actions) -->

</div>

Terzi is a deadly efficient CLI API client that eliminates complexity with precision. Perfect for developers who want the power of Postman in their terminal, with ruthless efficiency and exceptional user experience.

## ✨ Features

### 🚀 **Core Functionality**
- **All HTTP Methods**: GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS
- **Multiple Auth Types**: Bearer tokens, Basic auth, API keys, OAuth2
- **Request Bodies**: JSON, form data, raw text, file uploads
- **Beautiful Output**: Syntax highlighting, colored responses, formatted JSON/XML

### 🎨 **Exceptional UX**
- **Interactive Mode**: Guided prompts for beginners
- **Request History**: Automatic saving and replay of requests
- **Smart Collections**: Organize and group related requests
- **Template System**: Dynamic requests with variables and environments
- **Fuzzy Search**: Quick finding of saved requests

### ⚡ **Performance & Developer Experience**
- **Blazingly Fast**: Built in Rust for maximum performance
- **Auto-completion**: Intelligent suggestions and completions
- **Export/Import**: Postman collection compatibility
- **Response Inspection**: Headers, status codes, timing, size analysis
- **Request Chaining**: Use response data in subsequent requests

### 🔧 **Advanced Features**
- **Environment Management**: Dev, staging, prod configurations
- **Request Validation**: Pre-flight checks and validations
- **Response Diffing**: Compare responses across requests
- **Backup & Restore**: Never lose your API workflows
- **Plugin Architecture**: Extensible design for custom functionality

## 🏃‍♂️ Quick Start

### Installation

```bash
# Install from crates.io
cargo install terzi

# Or build from source
git clone https://github.com/simplysabir/terzi
cd terzi
cargo install --path .
```

### Basic Usage

```bash
# Simple GET request
terzi https://api.github.com/users/octocat

# POST with JSON data
terzi post https://httpbin.org/post -j '{"name": "terzi", "type": "cli"}'

# With authentication
terzi get https://api.github.com/user -A "bearer:your_token_here"

# Interactive mode for guided experience
terzi interactive

# Save request for later use
terzi get https://api.example.com/users --save my-users-request

# Load and replay saved request
terzi --load my-users-request
```

## 📚 Documentation

### Command Line Interface

```bash
# Basic request patterns
terzi <method> <url> [options]
terzi <url>  # Defaults to GET

# Request options
-H, --header <HEADER>     Add custom header (key:value)
-j, --json <JSON>         Send JSON data
-f, --form <FORM>         Send form data (key=value)
-A, --auth <AUTH>         Authentication (bearer:token, basic:user:pass)
-b, --body <BODY>         Raw request body
-t, --timeout <SECONDS>   Request timeout (default: 30)
-L, --follow-redirects    Follow HTTP redirects

# Output options
-o, --output <FORMAT>     Output format (auto, json, yaml, table, raw)
-i, --include-headers     Include response headers
-p, --pretty              Pretty print JSON (default: true)
-S, --silent              Silent mode (no formatting)
-v, --verbose             Verbose output with timing info

# Workflow options
--save <NAME>             Save request with a name
--load <NAME>             Load and execute saved request
```

### Interactive Mode

Start the interactive mode for a guided experience:

```bash
terzi interactive
```

The interactive mode provides:
- Step-by-step request building
- Smart defaults and suggestions
- Real-time validation
- Request preview before execution
- Auto-save successful requests

### Authentication

Terzi supports multiple authentication methods:

```bash
# Bearer Token
terzi get https://api.example.com/protected -A "bearer:your_token"

# Basic Authentication
terzi get https://api.example.com/protected -A "basic:username:password"

# API Key in Header
terzi get https://api.example.com/protected -A "api-key:X-API-Key:your_key"

# Custom Header
terzi get https://api.example.com/protected -H "Authorization: Custom your_token"
```

### Request Management

```bash
# List all saved requests
terzi list

# Search requests
terzi list --filter "user"

# Show request details
terzi show my-request

# Edit saved request
terzi edit my-request

# Delete request
terzi delete my-request

# View request history
terzi history --limit 20
```

### Collections and Environments

```bash
# Create a collection
terzi config create-collection "My API Tests"

# Set environment variables
terzi config set-env dev base_url https://dev-api.example.com
terzi config set-env prod base_url https://api.example.com

# Use environment in requests
terzi get "{{base_url}}/users" --env dev
```

### Import/Export

```bash
# Import from Postman collection
terzi import postman-collection.json

# Export your requests
terzi export --output my-requests.json --format json

# Backup all data
terzi config backup
```

## 🎯 Configuration

Terzi stores configuration in your system's config directory:
- Linux: `~/.config/terzi/`
- macOS: `~/Library/Application Support/terzi/`
- Windows: `%APPDATA%\terzi\`

### Settings

```bash
# View all settings
terzi config list

# Set a configuration value
terzi config set output.pretty_print true
terzi config set network.default_timeout 60
terzi config set ui.theme dark

# Reset to defaults
terzi config reset
```

### Key Configuration Options

- **`general.default_timeout`**: Default request timeout in seconds
- **`output.default_format`**: Default output format (auto, json, yaml, table)
- **`output.syntax_highlighting`**: Enable/disable syntax highlighting
- **`network.verify_ssl`**: SSL certificate verification
- **`ui.theme`**: Color theme (dark, light, auto)

## 🔧 Advanced Usage

### Request Templates

Create dynamic requests with variables:

```bash
# Create a template
terzi template create user-lookup \
  --url "https://api.example.com/users/{{user_id}}" \
  --method GET \
  --header "Authorization: Bearer {{token}}"

# Use the template
terzi template run user-lookup --var user_id=123 --var token=abc123
```

### Request Chaining

Use response data in subsequent requests:

```bash
# Save response data
terzi post https://api.example.com/auth -j '{"user":"test"}' --save-response auth_response

# Use in next request
terzi get https://api.example.com/profile -H "Authorization: Bearer {{auth_response.token}}"
```

### Response Processing

```bash
# Filter JSON responses
terzi get https://api.example.com/users --filter ".users[0].name"

# Compare responses
terzi diff request1 request2

# Save response to file
terzi get https://api.example.com/data --output-file data.json
```

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

### Development Setup

```bash
git clone https://github.com/simplysabir/terzi
cd terzi
cargo build
cargo test
cargo run -- --help
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_request_builder
```

## 📝 License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

<!-- ## 🚀 Roadmap

- [ ] **Plugin System**: Custom request processors and formatters
- [ ] **GraphQL Support**: Native GraphQL query support
- [ ] **WebSocket Testing**: Real-time connection testing
- [ ] **Performance Testing**: Load testing capabilities
- [ ] **API Documentation**: Generate docs from requests
- [ ] **Team Collaboration**: Shared collections and environments
- [ ] **Mock Server**: Built-in API mocking
- [ ] **Test Assertions**: Built-in testing framework -->

## 🎉 Acknowledgments

Inspired by excellent tools like:
- [HTTPie](https://httpie.io/) - Beautiful HTTP CLI
- [Postman](https://postman.com/) - Comprehensive API platform
- [Insomnia](https://insomnia.rest/) - Elegant REST client
- [curl](https://curl.se/) - The universal data transfer tool

---

<div align="center">

**[Website](https://terzi.xyz)** • **[Documentation](https://docs.terzi.xyz)** • **[Examples](https://github.com/simplysabir/terzi/tree/main/examples)**

Made with ❤️ by Sabir Khan

</div>