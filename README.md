<div align="center">

```
████████╗███████╗██████╗ ███████╗██╗
╚══██╔══╝██╔════╝██╔══██╗╚══███╔╝██║
   ██║   █████╗  ██████╔╝  ███╔╝ ██║
   ██║   ██╔══╝  ██╔══██╗ ███╔╝  ██║
   ██║   ███████╗██║  ██║███████╗██║
   ╚═╝   ╚══════╝╚═╝  ╚═╝╚══════╝╚═╝
```

# 🚀 Terzi

### Modern CLI API client designed for developer productivity

**Build, test, and manage your API requests with ease.**

---

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg?style=for-the-badge)](https://github.com/simplysabir/terzi)

</div>

---

## 🎯 **What is Terzi?**

**Terzi** is a modern CLI API client built for developers who value simplicity, speed, and precision. Whether you're testing APIs, building integrations, or debugging endpoints, Terzi provides an intuitive command-line interface that makes HTTP requests effortless.

Built with Rust for maximum performance and reliability, Terzi combines the power of curl with the convenience of modern tooling to streamline your API development workflow.

---

## ✨ **Key Features**

### **🚀 Core Functionality**
- **Lightning-fast HTTP requests** with minimal overhead
- **Interactive mode** for guided API exploration
- **Smart request building** with built-in validation
- **Beautiful output formatting** with syntax highlighting
- **Request history & analytics** for tracking API usage

### **🛠️ Developer-Friendly**
- **Zero-config setup** - works out of the box
- **Intuitive CLI interface** with helpful error messages
- **Rich visual feedback** with color-coded responses
- **Comprehensive help system** with examples
- **Cross-platform compatibility** (Windows, macOS, Linux)

### **📊 Professional Features**
- **Save & organize** request collections
- **Export/import** configurations
- **Environment management** for different setups
- **Authentication handling** (Bearer, Basic, API keys)
- **Response analysis** with detailed metrics
- **Sensitive data masking** for security
- **Configurable timeouts and redirects**

---

## 🚀 **Quick Start**

### **Installation**

```bash
# Build from source
git clone https://github.com/simplysabir/terzi
cd terzi
cargo build --release
./target/release/terzi --help
```

### **Your First API Request**

```bash
# Simple GET request
terzi https://api.github.com/users/octocat

# Interactive mode for guided experience
terzi interactive

# Save a request for later
terzi --save "github-user" https://api.github.com/users/octocat

# Load and execute saved request
terzi --load "github-user"
```

---

## 📋 **Complete Usage Guide**

### **Basic Commands**

```bash
# Direct requests
terzi https://api.example.com/users              # GET request
terzi -m POST https://api.example.com/users      # POST request
terzi -H "Authorization: Bearer token" https://api.example.com/protected

# Request building
terzi --method POST \
      --header "Content-Type: application/json" \
      --json '{"name": "John", "email": "john@example.com"}' \
      https://api.example.com/users

# Form data
terzi --form name=John --form email=john@example.com https://api.example.com/users

# Authentication
terzi --auth "bearer:your-token" https://api.example.com/protected
terzi --auth "basic:user:pass" https://api.example.com/protected
terzi --auth "apikey:X-API-Key:your-key" https://api.example.com/protected
```

### **All CLI Options**

```bash
# HTTP Methods
terzi -m GET|POST|PUT|DELETE|PATCH|HEAD|OPTIONS <url>

# Headers
terzi -H "Content-Type: application/json" -H "User-Agent: MyApp" <url>

# Request Body
terzi -b "raw body content" <url>                    # Raw body
terzi -j '{"key": "value"}' <url>                    # JSON body
terzi -f key=value -f name=John <url>                # Form data

# Authentication
terzi -A "bearer:token" <url>                        # Bearer token
terzi -A "basic:user:pass" <url>                     # Basic auth
terzi -A "apikey:X-API-Key:key" <url>               # API key

# Request Options
terzi -t 60 <url>                                    # Timeout (seconds)
terzi -L <url>                                       # Follow redirects
terzi -v <url>                                       # Verbose output
terzi -S <url>                                       # Silent mode

# Output Options
terzi -o json <url>                                  # Force JSON output
terzi -o yaml <url>                                  # YAML output
terzi -o table <url>                                 # Table format
terzi -o auto <url>                                  # Auto-detect (default)
terzi -i <url>                                       # Include headers
terzi -p <url>                                       # Pretty print

# Save/Load
terzi --save "name" <url>                           # Save request
terzi --load "name"                                 # Load request
```

### **Request Management**

```bash
# List and manage saved requests
terzi list                                          # List all requests
terzi list --filter "api"                          # Filter by pattern
terzi show "my-request"                             # Show request details
terzi edit "my-request"                             # Edit saved request
terzi delete "my-request"                           # Delete request

# History and analytics
terzi history                                       # View request history
terzi history --limit 20                           # Show last 20 requests

# Export/Import
terzi export --output my-requests.json             # Export all requests
terzi export --format yaml --output requests.yaml  # Export as YAML
```

### **Configuration Management**

```bash
# View configuration
terzi config list                                   # Show all settings
terzi config get timeout                           # Get specific setting

# Set configuration
terzi config set timeout 60                        # Set timeout
terzi config set output json                       # Set default output
terzi config set verbose true                      # Enable verbose mode
terzi config set show_headers true                 # Show headers by default
terzi config set pretty_print true                 # Pretty print by default

# Reset configuration
terzi config reset                                  # Reset to defaults
```

### **Interactive Mode**

Launch the interactive mode for a guided experience:

```bash
terzi interactive
```

**Interactive features:**
- 🎯 **Guided request building** with step-by-step prompts
- 📋 **Request collection browser** with fuzzy search
- 🔍 **History explorer** with detailed analytics
- ✏️ **Request editor** for fine-tuning
- 💾 **Smart saving** with automatic organization

---

## 🔧 **Configuration**

Terzi can be configured to match your workflow. Configuration files are stored in:

- **Linux/macOS:** `~/.config/terzi/config.toml`
- **Windows:** `%APPDATA%\terzi\config.toml`

### **General Settings**

```bash
# Request defaults
terzi config set default_timeout 30                # Default timeout in seconds
terzi config set follow_redirects true             # Follow redirects by default
terzi config set max_redirects 5                   # Maximum redirects to follow
terzi config set verify_ssl true                   # Verify SSL certificates

# Output settings
terzi config set default_format "auto"             # Default output format
terzi config set pretty_print true                 # Pretty print responses
terzi config set show_headers false                # Show headers in output
terzi config set color_output true                 # Enable colored output
```

### **Network Settings**

```bash
# Proxy configuration
terzi config set proxy_url "http://proxy:8080"     # Set HTTP proxy
terzi config set proxy_auth "user:pass"            # Proxy authentication

# Connection settings
terzi config set user_agent "Terzi/1.0"           # Custom user agent
terzi config set keep_alive true                   # Enable keep-alive
terzi config set connection_timeout 10             # Connection timeout
terzi config set read_timeout 30                   # Read timeout
```

### **Security Settings**

```bash
# Data masking
terzi config set mask_sensitive_data true          # Mask sensitive data
terzi config set mask_auth_headers true            # Mask auth headers
terzi config set mask_tokens true                  # Mask tokens in output
```

---

## 🎨 **Beautiful Output**

Terzi produces clean, readable output with:

- **Color-coded status indicators** (🟢 success, 🔴 error, 🟡 warning)
- **Syntax highlighting** for JSON, XML, and HTML responses
- **Professional formatting** with clean typography
- **Smart content detection** with automatic formatting
- **Responsive design** that adapts to terminal width
- **Sensitive data masking** for security

### **Example Output**

```
🟢 GET https://api.github.com/users/octocat 200 (342ms) 1.2KB
┌─────────────────────────────────────────────────────────────────┐
│ Request Details                                                 │
├─────────────────────────────────────────────────────────────────┤
│ Method: GET                                                     │
│ URL:    https://api.github.com/users/octocat                    │
│ Status: 200 OK                                                  │
│ Time:   342ms                                                   │
│ Size:   1.2KB                                                   │
└─────────────────────────────────────────────────────────────────┘

{
  "login": "octocat",
  "id": 1,
  "name": "The Octocat",
  "company": "GitHub",
  "public_repos": 8,
  "followers": 9001,
  "created_at": "2011-01-25T18:44:36Z"
}
```

---

## 🔐 **Security Features**

### **Sensitive Data Masking**

Terzi automatically masks sensitive data in output and history:

```bash
# Headers with sensitive data are masked
Authorization: Bearer to****xe
X-API-Key: ab****89
Cookie: session=ab****cd

# JSON responses with sensitive fields are masked
{
  "token": "ey****JV",
  "api_key": "sk****23",
  "password": "**masked**"
}
```

### **Authentication Methods**

```bash
# Bearer Token
terzi -A "bearer:your-token" <url>

# Basic Authentication
terzi -A "basic:username:password" <url>

# API Key (Header)
terzi -A "apikey:X-API-Key:your-key" <url>

# API Key (Custom Header)
terzi -A "apikey:Authorization:Bearer your-token" <url>
```

---

## 📊 **Request History & Analytics**

Terzi tracks all your requests and provides detailed analytics:

```bash
# View request history
terzi history                          # Show recent requests
terzi history --limit 50               # Show last 50 requests

# History includes:
# - Request method and URL
# - Response status and duration
# - Timestamp and request size
# - Success/error indicators
```

### **History Output Example**

```
┌─────────┬────────┬─────────────────────────────────────┬────────────┬──────────┐
│ Time    │ Method │ URL                                 │ Status     │ Duration │
├─────────┼────────┼─────────────────────────────────────┼────────────┼──────────┤
│ 14:23:45│ GET    │ https://api.github.com/users/oct... │ 🟢 200     │ 342ms    │
│ 14:22:10│ POST   │ https://api.example.com/users       │ 🟢 201     │ 156ms    │
│ 14:21:33│ GET    │ https://api.broken.com/endpoint     │ 🔴 404     │ 89ms     │
└─────────┴────────┴─────────────────────────────────────┴────────────┴──────────┘
```

---

## 🗂️ **Request Collections**

Organize your API requests into collections:

```bash
# Save requests with meaningful names
terzi --save "get-user-profile" https://api.example.com/users/me
terzi --save "create-user" -m POST -j '{"name":"John"}' https://api.example.com/users

# List and manage collections
terzi list                             # List all saved requests
terzi list --filter "user"             # Filter by name
terzi show "get-user-profile"          # Show request details
terzi edit "get-user-profile"          # Edit request
terzi delete "get-user-profile"        # Delete request
```

---

## 🌐 **Environment Management**

Different environments for different stages:

```bash
# Development environment
terzi config set base_url "https://api-dev.example.com"
terzi config set auth_token "dev-token"

# Production environment
terzi config set base_url "https://api.example.com"
terzi config set auth_token "prod-token"

# Use environment variables
export TERZI_AUTH_TOKEN="your-token"
terzi https://api.example.com/protected
```

---

## 📤 **Export & Import**

Share your request collections:

```bash
# Export requests
terzi export --output requests.json                # Export as JSON
terzi export --format yaml --output requests.yaml  # Export as YAML

# Import requests (coming soon)
terzi import --input requests.json                 # Import from JSON
```

---

## 🛠️ **Development**

### **Building from Source**

```bash
git clone https://github.com/simplysabir/terzi
cd terzi
cargo build --release
```

### **Running Tests**

```bash
cargo test                             # Run all tests
cargo test --lib                       # Run library tests
cargo test --bins                      # Run binary tests
```

### **Development Mode**

```bash
cargo run -- --help                   # Run with cargo
cargo run -- https://api.example.com  # Test API request
```

---

## 📖 **Documentation**

Comprehensive documentation is available:

- **[Examples](examples/)** - Real-world usage examples
- **[API Reference](docs/cli-reference.md)** - Complete API documentation
- **[Configuration Guide](docs/configuration.md)** - Configuration options
- **[Troubleshooting](docs/troubleshooting.md)** - Common issues

---

## 🤝 **Contributing**

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

### **Development Setup**

```bash
git clone https://github.com/simplysabir/terzi
cd terzi
cargo build
cargo test
```

### **Code Style**

```bash
cargo fmt                              # Format code
cargo clippy                           # Lint code
cargo check                            # Check compilation
```

---


## Docker Usage

Terzi can be run in a Docker container for easy installation and usage without requiring Rust to be installed locally.

### Quick Start with Docker

1. **Clone the repository:**
   ```bash
   git clone https://github.com/simplysabir/terzi.git
   cd terzi
   ```

2. **Build and run with Docker Compose:**
   ```bash
   docker compose up --build
   ```

3. **Use Terzi interactively:**
   ```bash
   # For interactive mode
   docker compose run terzi-api-client interactive
   
   # For direct API calls
   docker compose run terzi-api-client https://api.example.com/users
   
   # With custom options
   docker compose run terzi-api-client -X POST -j '{"name":"John"}' https://api.example.com/users
   ```

### Alternative Docker Commands

You can also use Docker directly:

```bash
# Build the image
docker build -t terzi .

# Run Terzi
docker run -it terzi https://api.example.com/users

# Run in interactive mode
docker run -it terzi interactive
```

### Benefits of Using Docker

- **No Rust installation required** - Everything runs in the container
- **Consistent environment** - Same behavior across different systems
- **Easy deployment** - Share the Docker image with your team
- **Isolated execution** - No conflicts with local system dependencies

### Data Persistence

To persist configuration and saved requests across container restarts, mount a volume:

```bash
# Using Docker Compose (recommended)
docker compose run -v $(pwd)/data:/root/.config/terzi terzi-api-client

# Using Docker directly
docker run -it -v $(pwd)/data:/root/.config/terzi terzi
```

## 📄 **License**

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 📞 **Support**

- 🐛 **Issues:** [GitHub Issues](https://github.com/simplysabir/terzi/issues)
- 💬 **Discussions:** [GitHub Discussions](https://github.com/simplysabir/terzi/discussions)
- 📧 **Email:** simplysabir@gmail.com

---

Made with ❤️ in Rust

</div>
