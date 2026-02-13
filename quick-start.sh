#!/usr/bin/env bash
# Invisible - Ultimate One-Click Launcher
# Run this single command and everything happens automatically

set -e

echo "🚀 Invisible - One-Click Start"
echo "=============================="
echo ""

# Run installer if not already built
if [ ! -f "target/release/invisible-relay" ]; then
    echo "📦 First-time setup detected. Running installer..."
    ./install.sh
else
    echo "✓ Already installed. Launching..."
fi

echo ""
echo "🔐 Starting Invisible Relay Node..."
echo ""
echo "Server running on: http://127.0.0.1:8080"
echo "Press Ctrl+C to stop"
echo ""

# Launch the relay node
exec ./target/release/invisible-relay
