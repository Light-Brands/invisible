# Invisible — Project Plan

**Version:** 1.0.0
**Created:** 2026-02-12
**Status:** Planning

---

## Phase 0: Foundation (Weeks 1-8)

**Goal:** Core cryptographic primitives and local-first architecture.

| Milestone | Description | Epic |
|-----------|-------------|------|
| M0.1 | Rust crypto core — Signal Protocol Double Ratchet, X3DH, Ed25519 | [epic-00](spec/epics/epic-00-foundation.md) |
| M0.2 | Post-quantum key exchange (PQXDH via ML-KEM + X25519) | [epic-00](spec/epics/epic-00-foundation.md) |
| M0.3 | SQLCipher encrypted local storage + Argon2id KDF | [epic-00](spec/epics/epic-00-foundation.md) |
| M0.4 | Device identity generation + per-conversation key management | [epic-00](spec/epics/epic-00-foundation.md) |
| M0.5 | Daily master key rotation system | [epic-00](spec/epics/epic-00-foundation.md) |

**Deliverable:** Crypto library with full test coverage, encrypted local DB, identity system.

---

## Phase 1: Core Messaging (Weeks 9-16)

**Goal:** 1-on-1 encrypted messaging with zero-identifier contacts.

| Milestone | Description | Epic |
|-----------|-------------|------|
| M1.1 | 1-on-1 E2EE messaging (Double Ratchet) | [epic-01](spec/epics/epic-01-messaging.md) |
| M1.2 | Disappearing messages (configurable 24h-90d) | [epic-01](spec/epics/epic-01-messaging.md) |
| M1.3 | QR-code and one-time link contact exchange | [epic-02](spec/epics/epic-02-contacts.md) |
| M1.4 | Pairwise anonymous identifiers | [epic-02](spec/epics/epic-02-contacts.md) |
| M1.5 | Key verification (safety numbers) | [epic-02](spec/epics/epic-02-contacts.md) |
| M1.6 | Mandatory 2FA + biometric gating | [epic-16](spec/epics/epic-16-access-control.md) |

**Deliverable:** Working encrypted messenger with contact exchange, auto-purge, mandatory 2FA.

---

## Phase 2: The Scrambler (Weeks 17-28)

**Goal:** Full 7-layer network obfuscation system.

| Milestone | Description | Epic |
|-----------|-------------|------|
| M2.1 | Sphinx packet format implementation (2KB uniform) | [epic-03](spec/epics/epic-03-scrambler.md) |
| M2.2 | Layer 1: Shamir's Secret Sharing message fragmentation | [epic-03](spec/epics/epic-03-scrambler.md) |
| M2.3 | Layer 2: 5-layer mixnet (Loopix architecture) | [epic-03](spec/epics/epic-03-scrambler.md) |
| M2.4 | Layer 3: Cover traffic (constant-rate Sphinx loop traffic) | [epic-03](spec/epics/epic-03-scrambler.md) |
| M2.5 | Layer 4: Jurisdiction routing (multi-jurisdiction path enforcement) | [epic-03](spec/epics/epic-03-scrambler.md) |
| M2.6 | Layer 5: Protocol camouflage (obfs5, uTLS, domain fronting) | [epic-03](spec/epics/epic-03-scrambler.md) |
| M2.7 | Layer 6: Dead drop architecture (anonymous relay mailboxes) | [epic-03](spec/epics/epic-03-scrambler.md) |
| M2.8 | Layer 7: Temporal scrambling (Poisson-distributed delays) | [epic-03](spec/epics/epic-03-scrambler.md) |
| M2.9 | Relay node software (RAM-only processing) | [epic-14](spec/epics/epic-14-relay-nodes.md) |

**Deliverable:** Complete Scrambler stack routing all messages through mixnet with cover traffic.

---

## Phase 3: Ghost VPN + Hardening (Weeks 29-36)

**Goal:** Mandatory VPN layer, anti-forensics, device lockdown.

| Milestone | Description | Epic |
|-----------|-------------|------|
| M3.1 | Ghost VPN: WireGuard integration, ephemeral keys | [epic-09](spec/epics/epic-09-ghost-vpn.md) |
| M3.2 | Random global endpoint selection (50+ nodes) | [epic-09](spec/epics/epic-09-ghost-vpn.md) |
| M3.3 | Session timeout + auto-lock + max lifetime | [epic-09](spec/epics/epic-09-ghost-vpn.md) |
| M3.4 | Anti-forensics (no thumbnails, no clipboard, blank task switcher) | [epic-08](spec/epics/epic-08-hardening.md) |
| M3.5 | Screen capture prevention (FLAG_SECURE, capture detection) | [epic-08](spec/epics/epic-08-hardening.md) |
| M3.6 | Secure keyboard (optional built-in, no keystroke logging) | [epic-08](spec/epics/epic-08-hardening.md) |
| M3.7 | Duress PIN + panic gesture + remote wipe | [epic-16](spec/epics/epic-16-access-control.md) |

**Deliverable:** App launches only through VPN, full anti-forensic hardening, panic features.

---

## Phase 4: Groups + Media + Calls (Weeks 37-48)

**Goal:** Full-featured secure communication.

| Milestone | Description | Epic |
|-----------|-------------|------|
| M4.1 | MLS-based group messaging (RFC 9420) | [epic-04](spec/epics/epic-04-groups.md) |
| M4.2 | Encrypted member lists | [epic-04](spec/epics/epic-04-groups.md) |
| M4.3 | Encrypted file/image/voice transfer | [epic-05](spec/epics/epic-05-media.md) |
| M4.4 | RAM-based media viewer (no disk writes) | [epic-05](spec/epics/epic-05-media.md) |
| M4.5 | E2EE voice/video calls (WebRTC + custom SRTP) | [epic-06](spec/epics/epic-06-calls.md) |
| M4.6 | Burn Rooms — self-destructing chats | [epic-15](spec/epics/epic-15-burn-rooms.md) |

**Deliverable:** Group messaging, media sharing, voice/video calls, burn rooms.

---

## Phase 5: Shadow Wallet + Crypto (Weeks 49-60)

**Goal:** Built-in privacy-first financial layer.

| Milestone | Description | Epic |
|-----------|-------------|------|
| M5.1 | Non-custodial wallet generation + key management | [epic-10](spec/epics/epic-10-shadow-wallet.md) |
| M5.2 | Monero integration (ring signatures, stealth addresses) | [epic-11](spec/epics/epic-11-privacy-coins.md) |
| M5.3 | Zcash shielded + Bitcoin privacy (CoinJoin/PayJoin) | [epic-11](spec/epics/epic-11-privacy-coins.md) |
| M5.4 | In-chat payments (send/receive/split) | [epic-10](spec/epics/epic-10-shadow-wallet.md) |
| M5.5 | Phantom Swap — atomic cross-chain swaps (HTLC) | [epic-12](spec/epics/epic-12-phantom-swap.md) |
| M5.6 | XMR Hop for maximum unlinkability | [epic-12](spec/epics/epic-12-phantom-swap.md) |
| M5.7 | DeFi proxy — WalletConnect v2, anonymous RPC | [epic-13](spec/epics/epic-13-defi-proxy.md) |

**Deliverable:** Full crypto wallet with privacy coins, atomic swaps, anonymous DeFi access.

---

## Phase 6: Mesh + Community Nodes (Weeks 61-72)

**Goal:** Offline capability and decentralized infrastructure.

| Milestone | Description | Epic |
|-----------|-------------|------|
| M6.1 | Bluetooth/WiFi Direct mesh messaging | [epic-07](spec/epics/epic-07-mesh.md) |
| M6.2 | Store-and-forward for offline contacts | [epic-07](spec/epics/epic-07-mesh.md) |
| M6.3 | Community relay/mix/VPN node operation | [epic-14](spec/epics/epic-14-relay-nodes.md) |
| M6.4 | Warrant canary system | [epic-14](spec/epics/epic-14-relay-nodes.md) |
| M6.5 | Node health monitoring + auto-deprioritization | [epic-14](spec/epics/epic-14-relay-nodes.md) |

**Deliverable:** Mesh networking, community-operated infrastructure, fully decentralized.

---

## Phase 7: Audit + Launch (Weeks 73-84)

**Goal:** Security audit, reproducible builds, public launch.

| Milestone | Description |
|-----------|-------------|
| M7.1 | Independent cryptographic audit (e.g., NCC Group, Trail of Bits) |
| M7.2 | Reproducible build pipeline for verifiable binaries |
| M7.3 | Open-source release (all client + relay code) |
| M7.4 | Public beta launch |
| M7.5 | Bug bounty program |
| M7.6 | General availability |

**Deliverable:** Audited, open-source, publicly available Invisible messenger.

---

## Dependencies

```
Phase 0 (Foundation) ──→ Phase 1 (Messaging) ──→ Phase 2 (Scrambler)
                                    │                      │
                                    ▼                      ▼
                          Phase 4 (Groups/Media)   Phase 3 (VPN/Hardening)
                                    │                      │
                                    └──────────┬───────────┘
                                               ▼
                                    Phase 5 (Shadow Wallet)
                                               │
                                               ▼
                                    Phase 6 (Mesh/Nodes)
                                               │
                                               ▼
                                    Phase 7 (Audit/Launch)
```
