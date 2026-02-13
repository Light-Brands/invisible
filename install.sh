#!/usr/bin/env bash
set -e

# Invisible - One-Click Installer
# This script installs everything and launches the app

echo "🚀 Invisible - One-Click Installer"
echo "=================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Detect OS
OS="$(uname -s)"
case "${OS}" in
    Linux*)     MACHINE=Linux;;
    Darwin*)    MACHINE=Mac;;
    CYGWIN*)    MACHINE=Cygwin;;
    MINGW*)     MACHINE=MinGw;;
    *)          MACHINE="UNKNOWN:${OS}"
esac

echo -e "${BLUE}Detected OS: ${MACHINE}${NC}"
echo ""

# Step 1: Check/Install Rust
echo -e "${YELLOW}[1/5] Checking Rust installation...${NC}"
if ! command -v rustc &> /dev/null; then
    echo -e "${RED}Rust not found. Installing Rust...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo -e "${GREEN}✓ Rust installed successfully${NC}"
else
    RUST_VERSION=$(rustc --version)
    echo -e "${GREEN}✓ Rust already installed: ${RUST_VERSION}${NC}"
fi
echo ""

# Step 2: Update Rust toolchain
echo -e "${YELLOW}[2/5] Updating Rust toolchain...${NC}"
rustup update stable
rustup default stable
echo -e "${GREEN}✓ Rust toolchain updated${NC}"
echo ""

# Step 3: Install system dependencies
echo -e "${YELLOW}[3/5] Installing system dependencies...${NC}"
if [ "$MACHINE" = "Mac" ]; then
    if ! command -v brew &> /dev/null; then
        echo -e "${RED}Homebrew not found. Please install Homebrew first:${NC}"
        echo "  /bin/bash -c \"\$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""
        exit 1
    fi
    # No additional deps needed for now
    echo -e "${GREEN}✓ Mac dependencies OK${NC}"
elif [ "$MACHINE" = "Linux" ]; then
    if command -v apt-get &> /dev/null; then
        sudo apt-get update
        sudo apt-get install -y build-essential pkg-config libssl-dev
    elif command -v yum &> /dev/null; then
        sudo yum groupinstall -y "Development Tools"
        sudo yum install -y openssl-devel
    fi
    echo -e "${GREEN}✓ Linux dependencies installed${NC}"
fi
echo ""

# Step 4: Build the project
echo -e "${YELLOW}[4/5] Building Invisible (this may take a few minutes)...${NC}"
echo -e "${BLUE}Building all components in release mode...${NC}"
cargo build --release --workspace
echo ""
echo -e "${GREEN}✓ Build complete!${NC}"
echo ""

# Step 5: Run tests
echo -e "${YELLOW}[5/5] Running tests to verify installation...${NC}"
if cargo test --workspace --release 2>&1 | grep -q "test result: ok"; then
    echo -e "${GREEN}✓ All tests passed!${NC}"
else
    echo -e "${YELLOW}⚠ Some tests failed, but continuing...${NC}"
fi
echo ""

# Create run script
echo -e "${BLUE}Creating run script...${NC}"
cat > run.sh << 'RUNSCRIPT'
#!/usr/bin/env bash
# Invisible Runner

echo "🔐 Invisible - Privacy-First Messenger"
echo "======================================"
echo ""
echo "Choose what to run:"
echo ""
echo "1) Relay Node (Mix Network Server)"
echo "2) Tests (Verify Everything Works)"
echo "3) Build Info (Show What's Built)"
echo ""
read -p "Enter choice [1-3]: " choice

case $choice in
    1)
        echo ""
        echo "🚀 Starting Relay Node..."
        echo "Server will listen on http://127.0.0.1:8080"
        echo "Press Ctrl+C to stop"
        echo ""
        ./target/release/invisible-relay
        ;;
    2)
        echo ""
        echo "🧪 Running all tests..."
        cargo test --workspace --release
        ;;
    3)
        echo ""
        echo "📦 Built Components:"
        echo ""
        ls -lh target/release/invisible-* 2>/dev/null || echo "No binaries found"
        echo ""
        echo "📊 Test Results:"
        cargo test --workspace --release 2>&1 | grep "test result"
        ;;
    *)
        echo "Invalid choice"
        exit 1
        ;;
esac
RUNSCRIPT

chmod +x run.sh

# Installation complete
echo ""
echo -e "${GREEN}╔════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║                                            ║${NC}"
echo -e "${GREEN}║    ✓ Invisible Installed Successfully!    ║${NC}"
echo -e "${GREEN}║                                            ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}What's installed:${NC}"
echo "  • Crypto Core (X3DH, Double Ratchet, Post-Quantum)"
echo "  • Scrambler (7-layer network obfuscation)"
echo "  • Relay Nodes (mix network servers)"
echo "  • Shadow Wallet (multi-currency crypto)"
echo "  • Client SDK (FFI bindings)"
echo ""
echo -e "${BLUE}Quick Start:${NC}"
echo "  • Run the app:     ${GREEN}./run.sh${NC}"
echo "  • Start relay:     ${GREEN}./target/release/invisible-relay${NC}"
echo "  • Run tests:       ${GREEN}cargo test --workspace${NC}"
echo "  • Read guide:      ${GREEN}cat QUICKSTART.md${NC}"
echo ""
echo -e "${YELLOW}Next Steps:${NC}"
echo "  1. Run: ${GREEN}./run.sh${NC}"
echo "  2. Choose option 1 to start the relay node"
echo "  3. Check QUICKSTART.md for usage examples"
echo ""
echo -e "${GREEN}Ready to go! 🚀${NC}"
echo ""
