#!/bin/bash
# Invisible Platform Health Check Script
# Verifies all services are operational

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "═══════════════════════════════════════════════════"
echo "   INVISIBLE PLATFORM - HEALTH CHECK"
echo "═══════════════════════════════════════════════════"
echo ""

ERRORS=0
WARNINGS=0

# Function to print status
print_status() {
    local status=$1
    local service=$2
    local message=$3

    if [ "$status" = "OK" ]; then
        echo -e "${GREEN}✅${NC} $service: $message"
    elif [ "$status" = "WARN" ]; then
        echo -e "${YELLOW}⚠️${NC}  $service: $message"
        WARNINGS=$((WARNINGS + 1))
    else
        echo -e "${RED}❌${NC} $service: $message"
        ERRORS=$((ERRORS + 1))
    fi
}

echo "CORE SERVICES (Critical)"
echo "───────────────────────────────────────────────────"

# Check 1: VPN (Layer 0) - CRITICAL
echo -n "Checking VPN... "
if sudo wg show 2>/dev/null | grep -q "interface: wg0"; then
    HANDSHAKE=$(sudo wg show wg0 latest-handshakes | head -n1 | awk '{print $2}')
    NOW=$(date +%s)
    AGE=$((NOW - HANDSHAKE))

    if [ $AGE -lt 180 ]; then
        print_status "OK" "VPN (Layer 0)" "Connected (handshake ${AGE}s ago)"
    else
        print_status "WARN" "VPN (Layer 0)" "Connected but stale handshake (${AGE}s)"
    fi
else
    print_status "ERROR" "VPN (Layer 0)" "Not connected - PLATFORM WILL NOT WORK!"
fi

# Check 2: WireGuard Tools
echo -n "Checking WireGuard tools... "
if command -v wg &> /dev/null; then
    print_status "OK" "WireGuard" "Installed ($(wg --version 2>&1 | head -n1))"
else
    print_status "ERROR" "WireGuard" "Not installed - run: brew install wireguard-tools"
fi

# Check 3: Rust Build
echo -n "Checking Rust workspace... "
if [ -f "Cargo.toml" ]; then
    if cargo --version &> /dev/null; then
        RUST_VERSION=$(cargo --version | awk '{print $2}')
        print_status "OK" "Rust Workspace" "Build ready (rustc $RUST_VERSION)"
    else
        print_status "ERROR" "Rust Workspace" "Rust not installed - run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    fi
else
    print_status "ERROR" "Rust Workspace" "Not in project root - cd to invisible/"
fi

echo ""
echo "NETWORK INFRASTRUCTURE"
echo "───────────────────────────────────────────────────"

# Check 4: Mix Nodes (Layer 2)
# Note: In production, replace with actual node addresses
echo -n "Checking mix network... "
if [ -n "$MIX_NODE_1" ]; then
    REACHABLE=0
    for i in 1 2 3 4 5; do
        NODE_VAR="MIX_NODE_$i"
        NODE=${!NODE_VAR}
        if [ -n "$NODE" ]; then
            if timeout 2 bash -c "cat < /dev/null > /dev/tcp/${NODE%:*}/${NODE##*:}" 2>/dev/null; then
                REACHABLE=$((REACHABLE + 1))
            fi
        fi
    done

    if [ $REACHABLE -eq 5 ]; then
        print_status "OK" "Mix Network (Layer 2)" "All 5 layers reachable"
    elif [ $REACHABLE -ge 3 ]; then
        print_status "WARN" "Mix Network (Layer 2)" "Only $REACHABLE/5 layers reachable (min 3)"
    else
        print_status "ERROR" "Mix Network (Layer 2)" "Only $REACHABLE/5 layers reachable (need min 3)"
    fi
else
    print_status "WARN" "Mix Network (Layer 2)" "Mix nodes not configured (set MIX_NODE_1-5 env vars)"
fi

# Check 5: Dead Drop Relays (Layer 6)
echo -n "Checking dead drops... "
if [ -n "$DEAD_DROP_1" ]; then
    AVAILABLE=0
    for i in 1 2 3; do
        DROP_VAR="DEAD_DROP_$i"
        DROP=${!DROP_VAR}
        if [ -n "$DROP" ]; then
            if timeout 2 bash -c "cat < /dev/null > /dev/tcp/${DROP%:*}/${DROP##*:}" 2>/dev/null; then
                AVAILABLE=$((AVAILABLE + 1))
            fi
        fi
    done

    if [ $AVAILABLE -gt 0 ]; then
        print_status "OK" "Dead Drops (Layer 6)" "$AVAILABLE/3 nodes available"
    else
        print_status "WARN" "Dead Drops (Layer 6)" "No nodes available (fallback to direct routing)"
    fi
else
    print_status "WARN" "Dead Drops (Layer 6)" "Not configured (set DEAD_DROP_1-3 env vars)"
fi

echo ""
echo "OPTIONAL SERVICES (Wallet Integration)"
echo "───────────────────────────────────────────────────"

# Check 6: Bitcoin RPC (Optional)
echo -n "Checking Bitcoin RPC... "
if command -v bitcoin-cli &> /dev/null; then
    if bitcoin-cli -testnet ping 2>/dev/null; then
        BLOCKS=$(bitcoin-cli -testnet getblockcount 2>/dev/null)
        print_status "OK" "Bitcoin RPC" "Connected (testnet, block $BLOCKS)"
    else
        print_status "WARN" "Bitcoin RPC" "Installed but not running (optional)"
    fi
else
    print_status "WARN" "Bitcoin RPC" "Not installed (optional - messaging works without it)"
fi

# Check 7: Monero RPC (Optional)
echo -n "Checking Monero RPC... "
if curl -s -X POST http://localhost:28088/json_rpc \
    -d '{"jsonrpc":"2.0","id":"0","method":"get_version"}' \
    -H 'Content-Type: application/json' | grep -q "result" 2>/dev/null; then
    print_status "OK" "Monero RPC" "Connected (port 28088)"
else
    print_status "WARN" "Monero RPC" "Not running (optional - messaging works without it)"
fi

# Check 8: Zcash RPC (Optional)
echo -n "Checking Zcash RPC... "
if command -v zcash-cli &> /dev/null; then
    if zcash-cli -testnet getinfo &>/dev/null; then
        print_status "OK" "Zcash RPC" "Connected (testnet)"
    else
        print_status "WARN" "Zcash RPC" "Installed but not running (optional)"
    fi
else
    print_status "WARN" "Zcash RPC" "Not installed (optional - messaging works without it)"
fi

echo ""
echo "═══════════════════════════════════════════════════"
echo "SUMMARY"
echo "═══════════════════════════════════════════════════"

if [ $ERRORS -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo -e "${GREEN}✅ ALL SYSTEMS OPERATIONAL${NC}"
    echo ""
    echo "Platform is ready for production use!"
    exit 0
elif [ $ERRORS -eq 0 ]; then
    echo -e "${YELLOW}⚠️  DEGRADED - $WARNINGS warnings${NC}"
    echo ""
    echo "Core services operational. Some optional features unavailable."
    exit 0
else
    echo -e "${RED}❌ UNHEALTHY - $ERRORS errors, $WARNINGS warnings${NC}"
    echo ""
    echo "Critical services offline. Platform will not work correctly."
    echo "Fix errors above and re-run health check."
    exit 1
fi
