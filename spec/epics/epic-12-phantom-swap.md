# Epic 12: Phantom Swap — Atomic Cross-Chain Swaps

## Overview
Implement trustless cross-chain cryptocurrency swaps using Hash Time-Locked Contracts (HTLCs) with anonymous P2P orderbook discovery through the Scrambler.

## User Stories

### US-12.1: BTC-XMR Atomic Swap
As a user, I want to swap Bitcoin for Monero without any exchange or KYC.

**Acceptance Criteria:**
- HTLC-based atomic swap protocol
- Trustless: both or neither party gets funds
- No intermediary, no exchange account
- Time-locked: automatic refund on timeout
- Swap negotiation routed through Scrambler (anonymous)
- HTLC creation/claim transactions broadcast through full 7-layer Scrambler (not just negotiation)
- Multi-node broadcast for both BTC and XMR legs
- Random delay between swap agreement and on-chain HTLC creation

### US-12.2: P2P Orderbook
As a user, I want to find swap counterparties without revealing my identity.

**Acceptance Criteria:**
- Decentralized orderbook distributed through Scrambler
- Orders are anonymous (no identity of maker/taker)
- Order matching done client-side
- Orders include: pair, amount, rate, expiry
- Stale orders auto-expire
- No central server coordinates swaps

### US-12.3: XMR Hop
As a user, I want to route swaps through Monero for maximum chain analysis resistance.

**Acceptance Criteria:**
- Multi-hop swap: source chain → XMR → destination chain
- Example: BTC → XMR (hold) → XMR → ETH
- Breaks chain analysis trail between source and destination
- User configurable hold period before second leg
- Automatic execution of second leg after hold
- Minimum mandatory random delay between swap legs (protocol-enforced, not just user choice)
- Each leg broadcast via independent Scrambler path through different jurisdictions

### US-12.4: Multi-Chain Swaps
As a user, I want to swap between any supported chains.

**Acceptance Criteria:**
- BTC <-> XMR, ETH <-> XMR, BTC <-> ZEC
- Any <-> Any via XMR Hop
- Custom HTLC contracts for EVM chains
- Fee estimation from real-time chain data (fetched through Scrambler)

### US-12.5: Swap History
As a user, I want my swap history to be private and auto-purging.

**Acceptance Criteria:**
- Swap history stored locally only (encrypted)
- Follows same auto-purge policy as messages
- No swap history on any server
- Swap details (counterparty, amounts) encrypted at rest

### US-12.6: Swap Cover Traffic
As a user, I want my swap activity to be hidden within constant cover traffic.

**Acceptance Criteria:**
- Constant-rate dummy swap monitoring queries indistinguishable from real HTLC monitoring
- Dummy orderbook queries mixed with real ones
- Observer cannot tell "user is actively swapping" from "wallet is running normally"
- Orderbook messages padded to uniform Sphinx packet size

## Technical Requirements
- HTLC: COMIT network library (Rust)
- XMR<->BTC: proven atomic swap protocol
- EVM HTLCs: custom Solidity contracts
- Orderbook: gossip protocol through Scrambler
- Timeout handling: automatic refund

## Dependencies
- Epic 10 (Shadow Wallet), Epic 11 (Privacy Coins), Epic 03 (Scrambler)

## Architecture References
- [phantom-swap.md](../architecture/phantom-swap.md), [shadow-wallet.md](../architecture/shadow-wallet.md)
