# Epic 13: DeFi Proxy — Anonymous DeFi Access

## Overview
Provide anonymous access to decentralized finance protocols by proxying all blockchain interactions through the Scrambler, hiding user IP from blockchain nodes and dApp frontends.

## User Stories

### US-13.1: WalletConnect v2 Integration
As a user, I want to connect my existing DeFi wallets through Invisible for anonymous access.

**Acceptance Criteria:**
- Standard WalletConnect v2 protocol support
- Connect MetaMask, Trust Wallet, Phantom, Rabby, etc.
- Session auto-expires on app lock
- No persistent connection state on server
- All WalletConnect relay traffic through Scrambler

### US-13.2: RPC Proxy
As a user, I want all blockchain RPC calls routed through the Scrambler so my IP is hidden.

**Acceptance Criteria:**
- Ethereum/Solana/Polygon RPC calls proxied through Scrambler
- IP never reaches Infura, Alchemy, or any node provider
- Response caching for gas prices, block height (reduce latency)
- Custom RPC endpoints supported
- Supports all EVM chains + Solana
- Constant-rate dummy RPC queries as cover traffic (balance checks, gas prices, block height)
- Real queries replace cover queries — no observable traffic change
- Cover traffic starts on app launch, not when user opens wallet

### US-13.3: Anonymous dApp Interaction
As a user, I want to use Uniswap, Aave, Curve, etc. without anyone knowing my IP or identity.

**Acceptance Criteria:**
- dApp frontend traffic proxied (no direct connection)
- No referrer headers, no cookies, no fingerprinting data
- ENS lookups through Scrambler
- Token balance queries proxied
- Transaction simulation proxied

### US-13.4: Local Transaction Signing
As a user, I want transaction signing to happen only on my device.

**Acceptance Criteria:**
- Private keys never leave device
- Signing happens in memory-locked region
- Signed transaction broadcast through Scrambler
- Mandatory random delay between user approval and blockchain broadcast (temporal scrambling)
- Multi-node broadcast via separate Scrambler exit paths
- No remote signing service

### US-13.5: Multi-Chain Support
As a user, I want anonymous DeFi access across multiple chains.

**Acceptance Criteria:**
- Ethereum mainnet + L2s (Arbitrum, Optimism, Base, zkSync)
- Solana
- Polygon
- Any EVM-compatible chain
- Chain switching within proxy

### US-13.6: Full Scrambler Parity for DeFi
As a user, I want all my DeFi interactions to receive the same 7-layer privacy protection as my messages.

**Acceptance Criteria:**
- All RPC traffic is Sphinx packets through the 5-layer mixnet
- Jurisdiction routing enforced for all DeFi traffic
- Protocol camouflage wraps all RPC calls (DPI cannot distinguish DeFi from browsing)
- Dead drop-style polling for async operations (transaction confirmations, position monitoring)
- Financial cover traffic at constant rate
- No observable difference in network traffic whether user is messaging, trading, or idle

## Technical Requirements
- WalletConnect v2 SDK (Dart/Flutter)
- RPC proxy: Rust relay with Scrambler integration
- ENS resolution: proxied through Scrambler
- Caching: in-memory LRU cache for common queries

## Dependencies
- Epic 10 (Shadow Wallet), Epic 03 (Scrambler), Epic 09 (Ghost VPN)

## Architecture References
- [defi-proxy.md](../architecture/defi-proxy.md), [shadow-wallet.md](../architecture/shadow-wallet.md), [scrambler.md](../architecture/scrambler.md)
