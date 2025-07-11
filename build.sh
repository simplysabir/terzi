#!/bin/bash

# Terzi Build Script
# Builds and packages the Terzi CLI tool

set -e

echo "🎯 Building Terzi - The deadly efficient CLI API client"
echo "======================================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Cargo not found. Please install Rust: https://rustup.rs/${NC}"
    exit 1
fi

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}❌ Cargo.toml not found. Please run this script from the project root.${NC}"
    exit 1
fi

echo -e "${BLUE}📋 Rust version:${NC}"
rustc --version
cargo --version

# Clean previous builds
echo -e "${YELLOW}🧹 Cleaning previous builds...${NC}"
cargo clean

# Run tests
echo -e "${YELLOW}🧪 Running tests...${NC}"
cargo test --verbose

# Build in release mode
echo -e "${YELLOW}🔨 Building in release mode...${NC}"
cargo build --release

# Check if build was successful
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Build successful!${NC}"
    
    # Show binary info
    echo -e "${BLUE}📦 Binary information:${NC}"
    ls -lh target/release/terzi
    
    # Show binary size
    BINARY_SIZE=$(du -h target/release/terzi | cut -f1)
    echo -e "${GREEN}📏 Binary size: $BINARY_SIZE${NC}"
    
    # Test the binary
    echo -e "${YELLOW}🔬 Testing binary...${NC}"
    ./target/release/terzi --version
    
    echo -e "${GREEN}🎉 Terzi built successfully!${NC}"
    echo -e "${BLUE}🎯 You can now run: ./target/release/terzi${NC}"
    echo -e "${BLUE}📦 Or install with: cargo install --path .${NC}"
    
else
    echo -e "${RED}❌ Build failed!${NC}"
    exit 1
fi

# Optional: Create installation package
if [ "$1" = "--package" ]; then
    echo -e "${YELLOW}📦 Creating installation package...${NC}"
    
    # Create package directory
    mkdir -p dist/terzi
    
    # Copy binary
    cp target/release/terzi dist/terzi/
    
    # Copy documentation
    cp README.md dist/terzi/
    cp -r examples/ dist/terzi/ 2>/dev/null || true
    
    # Create install script
    cat > dist/terzi/install.sh << 'EOF'
#!/bin/bash
echo "Installing Terzi..."
chmod +x terzi
sudo mv terzi /usr/local/bin/
echo "✅ Terzi installed successfully!"
echo "🎯 Run 'terzi --help' to get started"
EOF
    
    chmod +x dist/terzi/install.sh
    
    # Create archive
    cd dist
    # Get version from Cargo.toml more reliably
    VERSION=$(grep '^version =' ../../Cargo.toml | cut -d'"' -f2)
    tar -czf terzi-${VERSION}-$(uname -s)-$(uname -m).tar.gz terzi/
    cd ..
    
    echo -e "${GREEN}📦 Package created: dist/terzi-${VERSION}-$(uname -s)-$(uname -m).tar.gz${NC}"
fi

echo -e "${GREEN}🎯 Build complete!${NC}"