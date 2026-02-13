# Invisible

**Messages that leave no trace. Privacy that answers to no one.**

Invisible is a maximally secure, privacy-first messenger that goes beyond Signal and Telegram. Zero-trust architecture, zero-metadata collection, zero-compromise on privacy.

## What This Is

A complete private communications platform built on:

- **End-to-end encryption** with post-quantum resistance (X3DH + PQXDH + Double Ratchet)
- **The Scrambler** — 7-layer network obfuscation (mixnet, fragmentation, cover traffic, jurisdiction routing, protocol camouflage, dead drops, temporal scrambling)
- **Ghost VPN** — mandatory built-in WireGuard tunnel with random global endpoints
- **Zero identifiers** — no phone number, no email, no username. Ever.
- **Shadow Wallet** — non-custodial privacy-first crypto payments (XMR, ZEC, BTC, ETH)
- **Zero-Log Doctrine** — RAM-only relay nodes, no disk writes, nothing to seize
- **Burn Rooms** — self-destructing conversations with ephemeral keys
- **Mandatory 2FA** — architecturally required, no bypass

## Privacy Parity — Messages and Money Get the Same Protection

Every privacy layer that protects your messages also protects your money. No exceptions.

| Protection Layer | Messages | Payments | Swaps | DeFi |
|---|---|---|---|---|
| **Ghost VPN (Layer 0)** — mandatory WireGuard tunnel, random global endpoint | Yes | Yes | Yes | Yes |
| **Shamir Fragmentation (Layer 1)** — split into K-of-N shares across separate paths | Yes | Yes | Yes | Yes |
| **5-Layer Mixnet (Layer 2)** — Sphinx packets, batch-shuffle-forward | Yes | Yes | Yes | Yes |
| **Cover Traffic (Layer 3)** — constant-rate stream, real ops replace dummies | Yes | Yes | Yes | Yes |
| **Jurisdiction Routing (Layer 4)** — multi-country paths, no Five Eyes clustering | Yes | Yes | Yes | Yes |
| **Protocol Camouflage (Layer 5)** — obfs5/uTLS/domain fronting, invisible to DPI | Yes | Yes | Yes | Yes |
| **Dead Drops (Layer 6)** — anonymous relay mailboxes, sender/recipient never meet | Yes | Yes | Yes | Yes |
| **Temporal Scrambling (Layer 7)** — Poisson-distributed random delays at every hop | Yes | Yes | Yes | Yes |
| **Multi-Node Broadcast** — sent to N nodes via N separate Scrambler exits | N/A | Yes | Yes | Yes |
| **Timing Decorrelation** — chat notification and on-chain tx deliberately desynchronized | N/A | Yes | Yes | Yes |
| **Financial Cover Traffic** — dummy RPC queries from app launch, indistinguishable from real | N/A | Yes | Yes | Yes |
| **No Direct Blockchain Connection** — every RPC call through Ghost VPN + full Scrambler | N/A | Yes | Yes | Yes |
| **E2EE (Double Ratchet + PQXDH)** — post-quantum encrypted content | Yes | Yes | Yes | N/A |
| **Zero Identifiers** — no phone, no email, no username, no wallet address exposed | Yes | Yes | Yes | Yes |
| **Local-Only Storage** — encrypted at rest, auto-purge, no cloud sync | Yes | Yes | Yes | Yes |
| **Mandatory 2FA** — required for app access, part of encryption key derivation | Yes | Yes | Yes | Yes |
| **Panic Wipe** — duress PIN / gesture destroys all data including wallet | Yes | Yes | Yes | Yes |

**The principle:** If an adversary can't learn it from your messages, they can't learn it from your money either.

---

## Directory Structure

```
brands/invisible/
├── metadata.json              # Brand registry
├── README.md                  # This file
├── project-plan.md            # Development phases
└── spec/
    ├── MASTER-PLAN.md         # Strategic vision
    ├── README.md              # Spec index
    ├── architecture/
    │   ├── cryptography.md    # Encryption stack
    │   ├── scrambler.md       # 7-layer network obfuscation
    │   ├── identity-system.md # Zero-identifier design
    │   ├── data-models.md     # Local-only storage schemas
    │   ├── api-reference.md   # Relay node protocol
    │   ├── group-messaging.md # MLS-based groups
    │   ├── ghost-vpn.md       # Mandatory VPN gateway
    │   ├── zero-log-doctrine.md # RAM-only infrastructure
    │   ├── access-control.md  # 2FA + panic features
    │   ├── burn-rooms.md      # Self-destructing chats
    │   ├── shadow-wallet.md   # Crypto wallet
    │   ├── phantom-swap.md    # Atomic cross-chain swaps
    │   └── defi-proxy.md      # Anonymous DeFi access
    ├── epics/
    │   ├── epic-00-foundation.md
    │   ├── epic-01-messaging.md
    │   ├── epic-02-contacts.md
    │   ├── epic-03-scrambler.md
    │   ├── epic-04-groups.md
    │   ├── epic-05-media.md
    │   ├── epic-06-calls.md
    │   ├── epic-07-mesh.md
    │   ├── epic-08-hardening.md
    │   ├── epic-09-ghost-vpn.md
    │   ├── epic-10-shadow-wallet.md
    │   ├── epic-11-privacy-coins.md
    │   ├── epic-12-phantom-swap.md
    │   ├── epic-13-defi-proxy.md
    │   ├── epic-14-relay-nodes.md
    │   ├── epic-15-burn-rooms.md
    │   └── epic-16-access-control.md
    └── brand/
        ├── 01-visual-identity.md
        ├── 02-brand-voice.md
        └── 03-product-vision.md
```

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Crypto core | Rust (libsignal-protocol, ring, ML-KEM, sss) |
| Mobile clients | Flutter/Dart |
| Relay/mix nodes | Rust (Sphinx packet format, Loopix-inspired) |
| VPN | WireGuard (ChaCha20 + Poly1305 + Curve25519) |
| Local storage | SQLCipher + Argon2id KDF |
| Networking | libp2p, Tor fallback |
| Transports | obfs4/obfs5, uTLS fingerprint mimicry |
| Voice/video | WebRTC + custom SRTP key exchange |
| Wallet | monero-rs, zcash_client_backend, rust-bitcoin, ethers-rs |
| Swaps | COMIT HTLC library, XMR-BTC atomic swap protocol |
| DeFi | WalletConnect v2, custom RPC proxy |
| 2FA | TOTP (RFC 6238) + FIDO2/WebAuthn |

## Purpose

Invisible is built for internal team use. All features — full Scrambler, Ghost VPN, Shadow Wallet, Burn Rooms — are available to every team member with no feature gating.

If we choose to monetize in the future, the model would be open-core: all privacy features remain free, with optional paid tiers for priority relay infrastructure, team administration, and SLA guarantees.

## Links

- [Master Plan](spec/MASTER-PLAN.md) — strategic vision and competitive positioning
- [Spec Index](spec/README.md) — complete specification navigator
- [Architecture](spec/architecture/) — technical deep-dives
- [Epics](spec/epics/) — feature development specs
- [Brand Identity](spec/brand/) — visual identity and voice
