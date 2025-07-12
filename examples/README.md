# Terzi Examples

This directory contains comprehensive examples showing how to use all features of Terzi.

## 📋 Table of Contents

- [Basic Usage](#basic-usage) - Simple GET, POST, PUT, DELETE requests
- [Authentication](#authentication) - Bearer tokens, Basic auth, API keys
- [Request Management](#request-management) - Saving, loading, editing requests
- [Configuration](#configuration) - Settings, environments, profiles
- [Advanced Features](#advanced-features) - Complex scenarios and workflows
- [Interactive Mode](#interactive-mode) - Guided request building
- [Real-world APIs](#real-world-apis) - Examples with popular APIs

## 🚀 Quick Start

Before running examples, ensure Terzi is installed:

```bash
# Build from source
git clone https://github.com/simplysabir/terzi
cd terzi
cargo build --release

# Add to PATH or use directly
export PATH=$PATH:$(pwd)/target/release
```

## 📁 Example Files

| File | Description |
|------|-------------|
| [`basic-requests.sh`](basic-requests.sh) | Basic HTTP methods (GET, POST, PUT, DELETE) |
| [`authentication.sh`](authentication.sh) | Various authentication methods |
| [`request-management.sh`](request-management.sh) | Saving, loading, and managing requests |
| [`configuration.sh`](configuration.sh) | Configuration management and settings |
| [`advanced-usage.sh`](advanced-usage.sh) | Advanced features and complex scenarios |
| [`interactive-examples.md`](interactive-examples.md) | Interactive mode walkthrough |
| [`real-world-apis.sh`](real-world-apis.sh) | Examples with GitHub, JSONPlaceholder, etc. |
| [`json-examples.sh`](json-examples.sh) | Working with JSON data |
| [`form-data-examples.sh`](form-data-examples.sh) | Form data submissions |
| [`error-handling.sh`](error-handling.sh) | Error handling and troubleshooting |

## 🔧 Configuration Files

| File | Description |
|------|-------------|
| [`config-examples/`](config-examples/) | Sample configuration files |
| [`environments/`](environments/) | Environment-specific configurations |
| [`collections/`](collections/) | Request collection examples |

## 🌐 Test Servers

For testing purposes, we use these public APIs:

- **JSONPlaceholder** (`https://jsonplaceholder.typicode.com`) - Fake REST API
- **GitHub API** (`https://api.github.com`) - Real GitHub API
- **HTTPBin** (`https://httpbin.org`) - HTTP testing service
- **ReqRes** (`https://reqres.in`) - REST API testing

## 📝 Running Examples

Each example file is a shell script that can be run directly:

```bash
# Make executable
chmod +x examples/basic-requests.sh

# Run examples
./examples/basic-requests.sh
```

Or run individual commands:

```bash
# Copy commands from examples and run them
terzi https://jsonplaceholder.typicode.com/posts/1
```

## 💡 Tips

1. **Start with basic examples** - Get familiar with simple requests first
2. **Use interactive mode** - Great for learning and building complex requests
3. **Save frequently used requests** - Build your personal API collection
4. **Configure defaults** - Set up your preferred settings
5. **Check history** - Track your API usage and debug issues

## 🆘 Getting Help

If you encounter issues with examples:

1. Check the [troubleshooting guide](../docs/troubleshooting.md)
2. Review the [configuration documentation](../docs/configuration.md)
3. Open an issue on GitHub

## 📖 Next Steps

After trying these examples:

1. Read the [complete documentation](../docs/)
2. Explore the [advanced features](../docs/advanced.md)
3. Check out the [configuration guide](../docs/configuration.md)
4. Join the community discussions 