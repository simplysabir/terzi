#!/bin/bash

# Test script for Terzi Docker setup
echo "🐳 Testing Terzi Docker Setup..."

# Test 1: Validate Docker Compose config
echo "✅ Testing Docker Compose configuration..."
if docker compose config > /dev/null 2>&1; then
    echo "   ✓ Docker Compose config is valid"
else
    echo "   ✗ Docker Compose config has issues"
    exit 1
fi

# Test 2: Check Dockerfile syntax
echo "✅ Testing Dockerfile syntax..."
if docker build --no-cache -f Dockerfile . > /dev/null 2>&1; then
    echo "   ✓ Dockerfile builds successfully"
else
    echo "   ✗ Dockerfile has build issues"
    exit 1
fi

# Test 3: Run a simple command to test the binary
echo "✅ Testing Terzi binary in container..."
if docker run --rm terzi --version > /dev/null 2>&1; then
    echo "   ✓ Terzi binary works in container"
else
    echo "   ✗ Terzi binary has issues"
    exit 1
fi

echo "🎉 All Docker tests passed! Terzi is ready to use in Docker."
echo ""
echo "Quick start commands:"
echo "  docker compose run terzi-api-client interactive"
echo "  docker compose run terzi-api-client https://httpbin.org/get"
echo "  docker compose run terzi-api-client --help" 