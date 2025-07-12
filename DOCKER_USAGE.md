# Docker Usage Guide for Terzi

This guide provides comprehensive examples for using Terzi in Docker containers.

## Prerequisites

- Docker installed on your system
- Docker Compose (optional, but recommended)

## Building the Image

### Option 1: Using Docker Compose (Recommended)

```bash
# Build and start the container
docker compose up --build

# Or build without starting
docker compose build
```

### Option 2: Using Docker directly

```bash
# Build the image
docker build -t terzi .
```

## Running Terzi Commands

### Basic API Calls

```bash
# Simple GET request
docker compose run terzi-api-client https://httpbin.org/get

# GET with headers
docker compose run terzi-api-client -H "Authorization: Bearer token123" https://httpbin.org/headers

# POST with JSON
docker compose run terzi-api-client -X POST -j '{"name":"John","age":30}' https://httpbin.org/post

# POST with form data
docker compose run terzi-api-client -X POST -f "name=John" -f "age=30" https://httpbin.org/post
```

### Interactive Mode

```bash
# Start interactive mode
docker compose run terzi-api-client interactive

# Or using Docker directly
docker run -it terzi interactive
```

### Advanced Usage

```bash
# With custom timeout
docker compose run terzi-api-client -t 60 https://httpbin.org/delay/2

# With authentication
docker compose run terzi-api-client -A "bearer:your-token" https://api.example.com/protected

# Save a request
docker compose run terzi-api-client --save "my-request" https://api.example.com/endpoint

# Load a saved request
docker compose run terzi-api-client --load "my-request"
```

## Managing Saved Data

### Persistent Storage

To keep your saved requests and configuration across container restarts:

```bash
# Create a data directory
mkdir -p ./terzi-data

# Run with persistent storage
docker compose run -v $(pwd)/terzi-data:/root/.config/terzi terzi-api-client
```

### Backup and Restore

```bash
# Export saved requests
docker compose run -v $(pwd)/terzi-data:/root/.config/terzi terzi-api-client export --output /root/.config/terzi/backup.json

# Copy backup from container
docker cp terzi:/root/.config/terzi/backup.json ./backup.json
```

## Configuration Management

### Set Configuration

```bash
# Set default timeout
docker compose run terzi-api-client config set general.default_timeout 60

# Set output format
docker compose run terzi-api-client config set output.default_format json

# View all configuration
docker compose run terzi-api-client config list
```

### Environment Variables

You can pass environment variables to configure Terzi:

```bash
# Using Docker Compose
docker compose run -e TERZI_CONFIG_DIR=/custom/path terzi-api-client

# Using Docker directly
docker run -it -e TERZI_CONFIG_DIR=/custom/path terzi
```

## Common Use Cases

### API Testing Workflow

```bash
# 1. Start interactive mode to build your request
docker compose run terzi-api-client interactive

# 2. Save the request for reuse
# (done within interactive mode)

# 3. Load and execute the saved request
docker compose run terzi-api-client --load "my-test-request"

# 4. View request history
docker compose run terzi-api-client history
```

### CI/CD Integration

```bash
# Build the image in CI
docker build -t terzi:latest .

# Run API tests
docker run --rm terzi:latest https://api.example.com/health

# Run with specific configuration
docker run --rm -v $(pwd)/ci-config:/root/.config/terzi terzi:latest --load "health-check"
```

## Troubleshooting

### Common Issues

1. **Permission Issues with Volumes:**
   ```bash
   # Fix permissions
   sudo chown -R $(id -u):$(id -g) ./terzi-data
   ```

2. **Container Not Starting:**
   ```bash
   # Check logs
   docker compose logs terzi-api-client
   
   # Rebuild from scratch
   docker compose down
   docker compose build --no-cache
   ```

3. **Network Issues:**
   ```bash
   # Run with host networking
   docker run --network host -it terzi https://localhost:8080/api
   ```

### Debug Mode

```bash
# Run with verbose output
docker compose run terzi-api-client -v https://httpbin.org/get

# Access the container shell for debugging
docker compose run --entrypoint /bin/bash terzi-api-client
```

## Performance Optimization

### Multi-stage Build (Optional)

For production use, you can create a multi-stage Dockerfile:

```dockerfile
# Build stage
FROM rust:latest AS builder
WORKDIR /usr/src/terzi
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /usr/src/terzi/target/release/terzi /app/terzi
CMD ["./terzi"]
```

### Pre-built Images

You can also create and publish pre-built images:

```bash
# Build and tag
docker build -t yourusername/terzi:latest .

# Push to registry
docker push yourusername/terzi:latest

# Use pre-built image
docker run -it yourusername/terzi:latest
```

## Security Considerations

- Never include sensitive tokens or credentials in the Docker image
- Use environment variables or mounted volumes for sensitive data
- Consider using Docker secrets for production deployments
- Regularly update the base Rust image for security patches

## Next Steps

- Check the main README.md for detailed feature documentation
- Explore the interactive mode for guided API testing
- Set up persistent storage for your frequently used requests
- Integrate Terzi into your development workflow 