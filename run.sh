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
