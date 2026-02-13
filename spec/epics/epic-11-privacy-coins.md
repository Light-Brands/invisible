# Epic 11: Privacy Coins — Monero, Zcash & Bitcoin Privacy Integration

## Overview
Deep integration with privacy-preserving cryptocurrencies, leveraging ring signatures, stealth addresses, zk-SNARKs, and CoinJoin for transaction unlinkability.

## User Stories

### US-11.1: Monero Full Integration
As a user, I want native Monero support with all privacy features enabled by default.

**Acceptance Criteria:**
- Ring signatures (16+ decoys per transaction)
- Stealth addresses (one-time address per transaction)
- RingCT (Pedersen commitments hiding amounts)
- FCMP++ support when available (full chain membership proofs)
- Transaction broadcast through Scrambler (not directly to Monero network)
- Multi-node transaction broadcast via separate Scrambler exit paths (on top of Monero's Dandelion++)
- Financial cover traffic masks when real transactions occur
- View-only wallet option for auditing

### US-11.2: Zcash Shielded Transactions
As a user, I want to send/receive Zcash using only shielded transactions.

**Acceptance Criteria:**
- Only z-address (shielded) transactions supported
- Transparent t-addresses blocked/hidden
- zk-SNARKs proving transaction validity without revealing details
- Sapling/Orchard pool support

### US-11.3: Bitcoin Privacy Enhancement
As a user, I want my Bitcoin transactions to be as private as possible.

**Acceptance Criteria:**
- CoinJoin: mix transaction inputs with other users
- PayJoin: sender + recipient both contribute inputs
- Silent Payments (BIP-352): one-time addresses
- Fresh address for every transaction
- Optional: BTC → XMR → BTC atomic swap for full unlinkability
- CoinJoin coordinator interaction routed through full Scrambler pipeline
- Multi-node broadcast for all Bitcoin transactions via separate Scrambler exits
- Temporal scrambling between CoinJoin rounds to prevent timing analysis

### US-11.4: Privacy Score
As a user, I want to see a privacy rating for my transactions so I know how anonymous they are.

**Acceptance Criteria:**
- Privacy score (1-10) based on: chain, mixing level, address reuse, etc.
- Recommendations to improve privacy (e.g., "Use XMR for maximum privacy")
- Warning if privacy-reducing action detected (e.g., address reuse)

### US-11.5: Financial Traffic Indistinguishability
As a user, I want my financial network traffic to be indistinguishable from my messaging traffic.

**Acceptance Criteria:**
- All blockchain RPC queries are Sphinx packets through the mixnet (same as message packets)
- Financial cover traffic runs at the same rate as messaging cover traffic
- An observer monitoring the Scrambler cannot distinguish "user sent a payment" from "user sent a message" from "cover traffic"
- Opening the wallet tab does not create any observable change in network traffic patterns

## Technical Requirements
- Monero: monero-rs, monero-wallet-rpc integration
- Zcash: zcash_client_backend, librustzcash
- Bitcoin: rust-bitcoin, BDK, CoinJoin coordinator (JoinMarket-style)
- Privacy scoring: custom heuristic engine

## Dependencies
- Epic 10 (Shadow Wallet), Epic 03 (Scrambler)

## Architecture References
- [shadow-wallet.md](../architecture/shadow-wallet.md), [scrambler.md](../architecture/scrambler.md)
