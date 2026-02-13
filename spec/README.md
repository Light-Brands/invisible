# Invisible — Specification Index

**Messages that leave no trace. Privacy that answers to no one.**

This directory contains the complete technical specification for Invisible, an ultra-secure privacy-first messenger.

---

## Strategic Vision

- **[MASTER-PLAN.md](MASTER-PLAN.md)** — Mission, market positioning, competitive landscape, and the complete technical vision

---

## Architecture

Technical deep-dives into every major system component.

### Core Encryption
- **[cryptography.md](architecture/cryptography.md)** — Signal Protocol Double Ratchet, X3DH + PQXDH (post-quantum), AES-256-GCM, Ed25519, daily key rotation, key hierarchy and deletion policy

### Network Obfuscation
- **[scrambler.md](architecture/scrambler.md)** — THE SCRAMBLER: 7-layer obfuscation system (fragmentation, mixnet, cover traffic, jurisdiction routing, protocol camouflage, dead drops, temporal scrambling)
- **[ghost-vpn.md](architecture/ghost-vpn.md)** — Mandatory WireGuard VPN gateway: random global endpoints, ephemeral keys, session timeout, RAM-only nodes

### Identity & Storage
- **[identity-system.md](architecture/identity-system.md)** — Zero-identifier design: no phone/email/username, pairwise anonymous IDs, QR/link contact exchange, plausible deniability
- **[data-models.md](architecture/data-models.md)** — Local-only encrypted storage: SQLCipher schemas, auto-purge system, secure deletion
- **[api-reference.md](architecture/api-reference.md)** — Relay node protocol: packet submission, dead drop retrieval, key bundle management

### Group Communication
- **[group-messaging.md](architecture/group-messaging.md)** — MLS (RFC 9420) group key management, TreeKEM, encrypted member lists

### Security & Privacy
- **[zero-log-doctrine.md](architecture/zero-log-doctrine.md)** — RAM-only relay processing, device isolation, auto-purge policies, secure deletion, local encryption at rest
- **[access-control.md](architecture/access-control.md)** — Mandatory 2FA (TOTP + FIDO2), duress PIN, panic gesture, remote wipe, kill switch, anti-forensics
- **[burn-rooms.md](architecture/burn-rooms.md)** — Self-destructing conversations: timers, ephemeral room keys, dead man's switch, coordinated multi-device burn

### Financial Layer
- **[shadow-wallet.md](architecture/shadow-wallet.md)** — Non-custodial multi-chain crypto wallet: XMR, ZEC, BTC, ETH, stablecoins, in-chat payments
- **[phantom-swap.md](architecture/phantom-swap.md)** — Atomic cross-chain swaps: HTLC protocol, P2P orderbook, XMR Hop for chain analysis resistance
- **[defi-proxy.md](architecture/defi-proxy.md)** — Anonymous DeFi access: WalletConnect v2, RPC proxy through Scrambler, IP-hidden blockchain interaction

---

## Epics

Feature-level specifications with user stories, acceptance criteria, and technical requirements.

### Foundation
- **[epic-00-foundation.md](epics/epic-00-foundation.md)** — Core crypto library, local encrypted storage, device identity generation, daily key rotation

### Communication
- **[epic-01-messaging.md](epics/epic-01-messaging.md)** — 1-on-1 E2EE messaging, Double Ratchet, disappearing messages
- **[epic-02-contacts.md](epics/epic-02-contacts.md)** — QR-code contact exchange, zero-ID identity, key verification
- **[epic-04-groups.md](epics/epic-04-groups.md)** — MLS-based group messaging, encrypted membership
- **[epic-05-media.md](epics/epic-05-media.md)** — Encrypted file/image/voice transfer, streaming encryption, RAM-based viewing
- **[epic-06-calls.md](epics/epic-06-calls.md)** — E2EE voice/video calls via WebRTC + custom SRTP key exchange

### Network & Privacy
- **[epic-03-scrambler.md](epics/epic-03-scrambler.md)** — The Scrambler: all 7 layers (mixnet, fragmentation, cover traffic, jurisdiction routing, protocol camouflage, dead drops, temporal scrambling)
- **[epic-09-ghost-vpn.md](epics/epic-09-ghost-vpn.md)** — Built-in WireGuard VPN, random global endpoints, ephemeral keys, session management
- **[epic-14-relay-nodes.md](epics/epic-14-relay-nodes.md)** — Community relay/VPN/mix node operation, zero-log enforcement, warrant canary

### Security
- **[epic-08-hardening.md](epics/epic-08-hardening.md)** — Anti-forensics, screen capture prevention, device lockdown, secure keyboard
- **[epic-15-burn-rooms.md](epics/epic-15-burn-rooms.md)** — Self-destructing chats, burn timers, ephemeral room keys, dead man's switch
- **[epic-16-access-control.md](epics/epic-16-access-control.md)** — Mandatory 2FA, duress PIN, panic wipe, biometric gating, auto-lock

### Financial Layer
- **[epic-10-shadow-wallet.md](epics/epic-10-shadow-wallet.md)** — Non-custodial multi-chain wallet, seed generation, in-chat payments
- **[epic-11-privacy-coins.md](epics/epic-11-privacy-coins.md)** — Monero/Zcash/Bitcoin privacy integration, ring signatures, stealth addresses
- **[epic-12-phantom-swap.md](epics/epic-12-phantom-swap.md)** — Atomic cross-chain swaps, HTLC engine, P2P orderbook, XMR Hop
- **[epic-13-defi-proxy.md](epics/epic-13-defi-proxy.md)** — WalletConnect v2, RPC proxy through Scrambler, anonymous dApp access

### Connectivity
- **[epic-07-mesh.md](epics/epic-07-mesh.md)** — Bluetooth/WiFi Direct mesh for offline messaging

---

## Brand Identity

- **[01-visual-identity.md](brand/01-visual-identity.md)** — Dark-first design, monochrome palette, minimal UI, typography, logo
- **[02-brand-voice.md](brand/02-brand-voice.md)** — Calm, direct, no-nonsense tone. Trust through transparency, not marketing.
- **[03-product-vision.md](brand/03-product-vision.md)** — The messenger that doesn't exist. Market position, design philosophy, competitive landscape.

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Crypto core | Rust — libsignal-protocol, ring, ML-KEM, sss, zeroize |
| Mobile clients | Flutter/Dart |
| Relay/mix nodes | Rust — Sphinx packets, Loopix-inspired mixing, tokio async |
| VPN | WireGuard — ChaCha20 + Poly1305 + Curve25519 |
| Local storage | SQLCipher (AES-256) + Argon2id KDF |
| Networking | libp2p, Tor fallback |
| Transports | obfs4/obfs5, uTLS fingerprint mimicry |
| Calls | WebRTC + custom SRTP key exchange |
| Wallet | monero-rs, zcash_client_backend, rust-bitcoin, ethers-rs |
| Swaps | COMIT HTLC library, XMR-BTC atomic swap protocol |
| DeFi | WalletConnect v2, custom RPC proxy |
| 2FA | TOTP (RFC 6238) + FIDO2/WebAuthn |
| Build | Reproducible builds for verifiable binaries |
