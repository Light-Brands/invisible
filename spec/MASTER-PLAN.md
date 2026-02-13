# MASTER PLAN — Invisible

**Messages that leave no trace. Privacy that answers to no one.**

**Version:** 1.0.0
**Created:** 2026-02-12
**Status:** Strategic Vision

---

## Mission

Build the world's most private messenger — a communications platform where no message, no contact, no metadata, and no financial transaction can be traced, surveilled, or compelled by any entity. Privacy is not a feature toggle. It's the architecture.

**Core promise:** If we can't read it, neither can they.

---

## The Problem

Every mainstream messenger is fundamentally broken for privacy:

**Signal** — The best of the mainstream, but still requires a phone number. Centralized servers run by a US-based foundation. Metadata (who talks to whom, when, how often) is visible to Signal's infrastructure. Phone number requirement enables contact graph mapping.

**Telegram** — Not end-to-end encrypted by default. Cloud-stored messages readable by Telegram. Russian origin, UAE operations. Has cooperated with government requests. "Secret chats" are opt-in and limited.

**WhatsApp** — Owned by Meta. E2EE for message content, but metadata is harvested for ad targeting. Subject to US legal jurisdiction. Backdoor pressure from multiple governments. Cloud backups break encryption.

**iMessage** — Apple holds encryption keys for iCloud backups (enabled by default). Closed source. US jurisdiction. CSAM scanning controversy showed willingness to add client-side content analysis.

**Session** — Good privacy properties but limited features. No post-quantum. Small network. No financial layer.

**SimpleX** — Excellent zero-identifier design but no mixnet, no cover traffic, no protocol camouflage. Limited to messaging.

Even the best options leak metadata. Metadata is often more dangerous than content — it reveals patterns, relationships, locations, and habits. Courts have ruled that metadata alone can establish probable cause.

---

## The Vision

### Invisible is the messenger that doesn't exist.

Not just encrypted messages — an entire privacy infrastructure:

1. **The network can't see who's talking to whom** — The Scrambler routes every message through a 7-layer obfuscation system: Shamir fragmentation across multiple paths, 5-layer mixnet with batching and shuffling, constant cover traffic, jurisdiction-scrambled routing, protocol camouflage, anonymous dead drops, and temporal scrambling.

2. **Your ISP can't see you're using Invisible** — Ghost VPN wraps all traffic in a WireGuard tunnel to a random global endpoint. Protocol camouflage makes the traffic indistinguishable from normal web browsing to deep packet inspection.

3. **Your device reveals nothing if seized** — All local data is encrypted with a key derived from your 2FA secret (Argon2id KDF). Without the correct credentials, the database is computationally infeasible to decrypt. Panic wipe destroys everything instantly.

4. **No server has anything to give** — Relay nodes are stateless and RAM-only. No message logs, no connection logs, no IP logs, no database. A seized server yields an empty hard drive.

5. **Your money moves as privately as your words** — Shadow Wallet provides non-custodial crypto with Monero (privacy by default), Zcash (zk-SNARKs), and privacy-enhanced Bitcoin. Atomic swaps between chains without exchanges or KYC.

---

## Market Positioning

### Beyond Signal

Signal proved that mainstream E2EE messaging is possible. Invisible takes the next step: **zero-metadata messaging**.

| Dimension | Signal | Invisible |
|-----------|--------|-----------|
| Message encryption | E2EE (Double Ratchet) | E2EE (Double Ratchet + daily rotation) |
| Post-quantum | PQXDH | PQXDH (ML-KEM + X25519) |
| Identity | Phone number required | Zero identifiers |
| Network privacy | Direct connection to Signal servers | 7-layer Scrambler + mandatory VPN |
| Metadata | Visible to Signal servers | Invisible to everyone |
| Cover traffic | None | Constant-rate Sphinx loop traffic |
| Protocol detection | Identifiable by DPI | Camouflaged (obfs5, uTLS, domain fronting) |
| Server architecture | Centralized, persistent | Decentralized, RAM-only, stateless |
| Financial layer | None | Full crypto wallet + swaps + DeFi |
| Jurisdiction | US (Signal Foundation) | Decentralized (community nodes, multi-jurisdiction) |
| Open source | Partial (server closed) | Complete (client + relay + protocol) |

### Competitive Landscape

| Feature | Signal | Telegram | Session | SimpleX | Nym | **Invisible** |
|---------|--------|----------|---------|---------|-----|---------------|
| E2EE default | Yes | No | Yes | Yes | N/A | **Yes** |
| Post-quantum | Yes | No | No | No | No | **Yes** |
| No phone/email | No | No | Yes | Yes | No | **Yes** |
| Mandatory VPN | No | No | No | No | Yes* | **Yes** |
| Mixnet | No | No | No | No | Yes | **Yes** |
| Cover traffic | No | No | No | No | Yes | **Yes** |
| Message fragmentation | No | No | No | No | No | **Yes (Shamir)** |
| Zero identifiers | No | No | No | Yes | No | **Yes** |
| Daily key rotation | No | No | No | No | No | **Yes** |
| Jurisdiction routing | No | No | No | No | No | **Yes** |
| Protocol camouflage | No | No | No | No | No | **Yes** |
| Dead drops | No | No | No | Yes* | No | **Yes** |
| Temporal scrambling | No | No | No | No | Yes* | **Yes** |
| Mesh/offline | No | No | No | No | No | **Yes** |
| Burn rooms | No | Yes* | No | No | No | **Yes** |
| Mandatory 2FA | No | No | No | No | No | **Yes** |
| Panic wipe | No | No | No | No | No | **Yes** |
| Crypto wallet | No | Yes* | No | No | No | **Yes (privacy)** |
| Privacy coin native | No | No | No | No | No | **Yes (XMR/ZEC)** |
| Atomic swaps | No | No | No | No | No | **Yes** |
| DeFi proxy | No | No | No | No | No | **Yes** |
| In-chat payments | No | Yes* | No | No | No | **Yes (anonymous)** |
| Open source (all) | Partial | No | Yes | Yes | Yes | **Yes** |
| Decentralized | No | No | Yes | Yes | Yes | **Yes** |

---

## Architecture Overview

### The Scrambler — 7-Layer Network Obfuscation

Invisible's core differentiator. Every message passes through:

1. **Layer 1: Message Fragmentation** — Shamir's Secret Sharing splits messages into K-of-N shares, each routed through different geographic paths. Information-theoretic security: K-1 shares reveal zero information.

2. **Layer 2: Mixnet** — 5-layer Nym-style mixnet (not Tor onion routing). Sphinx packet format (all packets 2KB, indistinguishable). Each node batches, strips a layer, adds random delay, shuffles, and forwards.

3. **Layer 3: Cover Traffic** — Every client sends constant-rate Sphinx packets. Real messages replace cover packets — no observable traffic pattern change. Observer sees constant stream.

4. **Layer 4: Jurisdiction Routing** — Path selection mandates multi-jurisdiction diversity. No consecutive nodes in same country or intelligence alliance. Avoids Five Eyes clustering.

5. **Layer 5: Protocol Camouflage** — Pluggable transport framework. Default: obfs5 (random bytes). Fallback: uTLS (mimics Chrome/Firefox). Emergency: domain fronting via CDN. DPI cannot identify Invisible traffic.

6. **Layer 6: Dead Drops** — Messages deposited at anonymous relay mailboxes. Sender and recipient never connect to the same node. Dead drop IDs rotate per session.

7. **Layer 7: Temporal Scrambling** — Random delays (Poisson distribution) before entering mixnet. Each mix node adds independent delay. Recipient polls on own schedule. Zero temporal correlation.

**See:** [scrambler.md](architecture/scrambler.md)

### Ghost VPN — Layer 0

Mandatory WireGuard tunnel that auto-connects on app launch. Random global endpoint from 50+ nodes (zero Five Eyes). Ephemeral keys per session. 30-minute inactivity timeout. No VPN = No App.

**See:** [ghost-vpn.md](architecture/ghost-vpn.md)

### Encryption Stack

- **Key exchange:** X3DH + PQXDH (ML-KEM-1024 + X25519 hybrid)
- **Message encryption:** Signal Protocol Double Ratchet → AES-256-GCM per-message keys
- **Daily rotation:** Master key DH exchange every 24 hours (defense-in-depth)
- **Signatures:** Ed25519 for identity and authentication
- **Local storage:** SQLCipher (AES-256) with Argon2id-derived key
- **Forward secrecy + Post-compromise security** at every layer

**See:** [cryptography.md](architecture/cryptography.md)

### Zero-Identifier Identity

No phone number, no email, no username — ever. Identity is a device-local Ed25519 key pair. Each conversation uses a unique pairwise identifier. Contact exchange via QR code or one-time link only.

**See:** [identity-system.md](architecture/identity-system.md)

### Zero-Log Doctrine

No remote system stores any data. Relay nodes process packets in RAM only. Dead drops are ephemeral RAM queues (72h TTL). Crash = zero recoverable data. Seized server = empty hard drive. Each device is an island — no cloud sync, no cross-device history, no backup.

**See:** [zero-log-doctrine.md](architecture/zero-log-doctrine.md)

### Shadow Wallet

Non-custodial multi-chain crypto wallet. Monero (XMR) as primary currency (privacy by default). Zcash shielded, Bitcoin with CoinJoin/PayJoin/Silent Payments, Ethereum via ZK rollups. In-chat payments, group bill splitting. Phantom Swap for trustless atomic cross-chain swaps. XMR Hop for breaking chain analysis. Anonymous DeFi access via RPC proxy through Scrambler.

**Financial Privacy Parity Principle:** Every financial operation — transaction broadcasts, balance queries, swap negotiations, DeFi interactions — receives the same full 7-layer Scrambler protection as messages. No direct connection to any blockchain node, ever. Financial cover traffic runs at constant rate so an observer cannot distinguish "user sent a payment" from "user sent a message" from "cover traffic." Transaction broadcasts are temporally scrambled and multi-node broadcast via separate Scrambler exit paths. In-chat payment notifications and on-chain transactions are deliberately desynchronized to prevent timing correlation.

**See:** [shadow-wallet.md](architecture/shadow-wallet.md), [phantom-swap.md](architecture/phantom-swap.md), [defi-proxy.md](architecture/defi-proxy.md)

### Mandatory 2FA + Hardening

2FA is architecturally required — the TOTP secret is an input to the database encryption KDF. Biometric + TOTP/FIDO2 on every app open. Duress PIN silently wipes data. Panic gesture for immediate wipe. Anti-forensic measures: no thumbnails, no clipboard, blank task switcher, RAM-only media viewer, secure keyboard.

**See:** [access-control.md](architecture/access-control.md), [burn-rooms.md](architecture/burn-rooms.md)

---

## Purpose and Monetization

### Internal Team Use

Invisible is built for internal team use. Every feature — full Scrambler, Ghost VPN, Shadow Wallet, Burn Rooms, Phantom Swap — is available to every team member. There is no feature gating. Privacy is not a premium.

### If We Choose to Monetize

Should Invisible be released publicly, the model would be open-core:

#### Free Tier (Personal)
- Full E2EE messaging with all Scrambler layers
- Ghost VPN
- Shadow Wallet (basic send/receive)
- Burn rooms
- All privacy features — no feature gating on privacy

#### Paid Tier (Teams / Enterprise)
- Priority relay nodes (lower latency, higher throughput)
- Team key management and administration
- Admin controls for organizational deployment
- SLA guarantees for relay infrastructure
- Advanced Phantom Swap (higher volume, priority matching)
- Priority support channel
- Custom node deployment assistance

#### Infrastructure Economics
- Community node operators earn fees from routing traffic
- Fee structure incentivizes running high-quality, high-uptime nodes
- Revenue flows to infrastructure operators, not a central company
- Sustainable decentralization through economic incentives

---

## Tech Stack

| Layer | Technology | Why |
|-------|-----------|-----|
| Crypto core | Rust (libsignal-protocol, ring, ML-KEM) | Memory safety, performance, no GC |
| Mobile clients | Flutter/Dart | Cross-platform, single codebase |
| Relay/mix nodes | Rust (tokio async) | Performance, safety, small binary |
| VPN protocol | WireGuard | Audited, minimal, fast |
| Packet format | Sphinx (2KB uniform) | Proven mixnet packet format |
| Local storage | SQLCipher + Argon2id | Encrypted SQLite, memory-hard KDF |
| Networking | libp2p | Proven P2P networking stack |
| Transports | obfs5, uTLS | DPI resistance |
| Calls | WebRTC + custom SRTP | Standard media + custom key exchange |
| Wallet | monero-rs, rust-bitcoin, ethers-rs | Native Rust chain libraries |
| Swaps | COMIT HTLC | Proven atomic swap infrastructure |
| DeFi | WalletConnect v2 | Standard wallet connection |
| 2FA | TOTP (RFC 6238) + FIDO2 | Standard, proven second factors |
| Build | Reproducible builds | Verifiable binaries, trust through transparency |

---

## Design Philosophy

### Privacy is the architecture, not a feature
Every technical decision starts with: "What does an adversary learn?" If the answer is anything other than "nothing," redesign.

### Zero trust — including ourselves
The system is designed so that Invisible's developers, if compelled, have nothing to provide. No keys, no logs, no metadata, no user list. We cannot betray users' privacy because we never have access to it.

### Usability without compromise
The hardest problem in privacy tools is usability. Invisible must be as easy to use as iMessage while providing stronger privacy than anything available. If privacy requires effort from the user, we've failed.

### Open everything
All client code, relay code, protocol specs, and cryptographic implementations are open source. Reproducible builds ensure the binary matches the source. Trust is verified, never assumed.

### Defense in depth
No single layer provides security alone. Every mechanism (encryption, mixnet, VPN, cover traffic, jurisdiction routing, protocol camouflage, dead drops, temporal scrambling) adds independent protection. An adversary must defeat ALL layers simultaneously.

### Financial privacy parity
Money moves with the same protection as words. Every financial operation — transaction broadcasts, balance queries, swap negotiations, DeFi interactions — passes through the full 7-layer Scrambler stack. There is no "lite mode" for financial traffic. If the messenger protects against a threat, the wallet protects against it too.

---

## Specification Index

All technical specifications are documented in the [spec directory](README.md):

- **Architecture:** 13 documents covering every technical subsystem
- **Epics:** 17 feature specifications with user stories and acceptance criteria
- **Brand:** 3 documents defining visual identity, voice, and product vision
- **Project Plan:** Phased development from foundation through audit and launch

---

## What Success Looks Like

We can't track users. We don't know how many people use Invisible. That's the point.

Success is measured by:
- **Network health:** Number and diversity of community relay/mix/VPN nodes
- **Uptime:** Network availability and latency metrics
- **Security:** Results of independent cryptographic audits
- **Community:** Open-source contributors, protocol adoption by other projects
- **Resilience:** Absence of successful deanonymization attacks
- **Censorship resistance:** Availability in countries that attempt to block privacy tools
- **Sustainability:** Infrastructure economics that could keep the network running independently, if we choose to scale beyond internal use

---

*Zero-knowledge. Zero-trust. Zero-compromise.*
