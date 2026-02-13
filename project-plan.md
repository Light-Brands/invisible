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

**Goal:** Mandatory VPN layer, cellular metadata protection, anti-forensics, device lockdown.

| Milestone | Description | Epic/Doc |
|-----------|-------------|------|
| M3.1 | Ghost VPN: WireGuard integration, ephemeral keys | [epic-09](spec/epics/epic-09-ghost-vpn.md) |
| M3.2 | Random global endpoint selection (50+ nodes) | [epic-09](spec/epics/epic-09-ghost-vpn.md) |
| M3.3 | Session timeout + auto-lock + max lifetime | [epic-09](spec/epics/epic-09-ghost-vpn.md) |
| M3.4 | **Network Privacy Mode: WiFi-Only mode (cellular data blocking)** | [network-privacy](spec/architecture/network-privacy-mode.md) |
| M3.5 | **Network Privacy Mode: eSIM rotation manager (auto-rotate profiles)** | [network-privacy](spec/architecture/network-privacy-mode.md) |
| M3.6 | **Network Privacy Mode: MAC address randomization enforcement** | [network-privacy](spec/architecture/network-privacy-mode.md) |
| M3.7 | **Network Privacy Mode: Airplane Mode + WiFi quick toggle** | [network-privacy](spec/architecture/network-privacy-mode.md) |
| M3.8 | **Network Privacy Mode: UI (settings screen + status bar toggle)** | [network-privacy](spec/architecture/network-privacy-mode.md) |
| M3.9 | Anti-forensics (no thumbnails, no clipboard, blank task switcher) | [epic-08](spec/epics/epic-08-hardening.md) |
| M3.10 | Screen capture prevention (FLAG_SECURE, capture detection) | [epic-08](spec/epics/epic-08-hardening.md) |
| M3.11 | Secure keyboard (optional built-in, no keystroke logging) | [epic-08](spec/epics/epic-08-hardening.md) |
| M3.12 | Duress PIN + panic gesture + remote wipe | [epic-16](spec/epics/epic-16-access-control.md) |

**Deliverable:** App launches only through VPN, cellular metadata protection (WiFi-only/eSIM rotation/airplane mode), full anti-forensic hardening, panic features.

---

## Phase 4: Groups + Media + Calls (Weeks 37-48)

**Goal:** Full-featured secure communication + admin dashboard for power users.

| Milestone | Description | Epic/Doc |
|-----------|-------------|------|
| M4.1 | MLS-based group messaging (RFC 9420) | [epic-04](spec/epics/epic-04-groups.md) |
| M4.2 | Encrypted member lists | [epic-04](spec/epics/epic-04-groups.md) |
| M4.3 | Encrypted file/image/voice transfer | [epic-05](spec/epics/epic-05-media.md) |
| M4.4 | RAM-based media viewer (no disk writes) | [epic-05](spec/epics/epic-05-media.md) |
| M4.5 | E2EE voice/video calls (WebRTC + custom SRTP) | [epic-06](spec/epics/epic-06-calls.md) |
| M4.6 | Burn Rooms — self-destructing chats | [epic-15](spec/epics/epic-15-burn-rooms.md) |
| M4.7 | **Admin Dashboard: UI framework (tabs, navigation, responsive)** | [admin-dashboard](spec/architecture/admin-dashboard.md) |
| M4.8 | **Admin Dashboard: Configuration tab (service toggles, privacy presets)** | [admin-dashboard](spec/architecture/admin-dashboard.md) |
| M4.9 | **Admin Dashboard: Status dashboard (real-time monitoring, health checks)** | [admin-dashboard](spec/architecture/admin-dashboard.md) |
| M4.10 | **Admin Dashboard: Metrics collection and visualization (graphs, stats)** | [admin-dashboard](spec/architecture/admin-dashboard.md) |
| M4.11 | **Admin Dashboard: Activity logging system (events, filters, export)** | [admin-dashboard](spec/architecture/admin-dashboard.md) |
| M4.12 | **Admin Dashboard: Message send indicator (layer-by-layer progress)** | [admin-dashboard](spec/architecture/admin-dashboard.md) |

**Deliverable:** Group messaging, media sharing, voice/video calls, burn rooms, comprehensive admin dashboard with full visibility and control over all privacy features.

---

## Phase 5: Shadow Wallet + Crypto (Weeks 49-64)

**Goal:** Built-in privacy-first financial layer with fortress-grade security.

### Phase 5a: Wallet Foundation + Critical Hardening (Weeks 49-52)

| Milestone | Description | Epic/Doc |
|-----------|-------------|------|
| M5.1 | Non-custodial wallet generation + key management | [epic-10](spec/epics/epic-10-shadow-wallet.md) |
| M5.2 | **Hardware security module integration (iOS Secure Enclave, Android StrongBox)** | [hardening](spec/architecture/shadow-wallet-hardening.md) |
| M5.3 | **Transaction verification layer (address, amount, fee, replay protection)** | [hardening](spec/architecture/shadow-wallet-hardening.md) |
| M5.4 | **Enhanced memory protection (mlock, memory encryption, secure zeroing)** | [hardening](spec/architecture/shadow-wallet-hardening.md) |

**Deliverable:** Wallet foundation with hardware-protected keys and comprehensive transaction verification.

### Phase 5b: Privacy Coins + User Protection (Weeks 53-56)

| Milestone | Description | Epic/Doc |
|-----------|-------------|------|
| M5.5 | Monero integration (ring signatures, stealth addresses, subaddresses) | [epic-11](spec/epics/epic-11-privacy-coins.md) |
| M5.6 | Zcash shielded + Bitcoin privacy (CoinJoin/PayJoin, Silent Payments) | [epic-11](spec/epics/epic-11-privacy-coins.md) |
| M5.7 | In-chat payments (send/receive/split, timing decorrelation) | [epic-10](spec/epics/epic-10-shadow-wallet.md) |
| M5.8 | **Anti-phishing protection (identity verification, suspicious patterns)** | [hardening](spec/architecture/shadow-wallet-hardening.md) |
| M5.9 | **Smart contract security (approval limits, simulation, verification)** | [hardening](spec/architecture/shadow-wallet-hardening.md) |
| M5.10 | **Multi-signature wallet coordination via messenger** | [hardening](spec/architecture/shadow-wallet-hardening.md) |

**Deliverable:** Privacy coins integrated with user-facing security protections.

### Phase 5c: Advanced Features + Backup (Weeks 57-60)

| Milestone | Description | Epic/Doc |
|-----------|-------------|------|
| M5.11 | Phantom Swap — atomic cross-chain swaps (HTLC) | [epic-12](spec/epics/epic-12-phantom-swap.md) |
| M5.12 | XMR Hop for maximum unlinkability | [epic-12](spec/epics/epic-12-phantom-swap.md) |
| M5.13 | DeFi proxy — WalletConnect v2, anonymous RPC | [epic-13](spec/epics/epic-13-defi-proxy.md) |
| M5.14 | **Shamir seed backup (SLIP-39) + social recovery** | [hardening](spec/architecture/shadow-wallet-hardening.md) |
| M5.15 | **Privacy coin enhancements (Monero view keys, auto-CoinJoin, Zcash enforcement)** | [hardening](spec/architecture/shadow-wallet-hardening.md) |
| M5.16 | **Broadcast verification + fallback mechanism** | [hardening](spec/architecture/shadow-wallet-hardening.md) |

**Deliverable:** Full crypto wallet with atomic swaps, DeFi access, and advanced backup options.

### Phase 5d: Defense in Depth + Testing (Weeks 61-64)

| Milestone | Description | Epic/Doc |
|-----------|-------------|------|
| M5.17 | Ethereum + stablecoins (ZK-rollup privacy layers) | [epic-10](spec/epics/epic-10-shadow-wallet.md) |
| M5.18 | **Side-channel attack mitigation (constant-time ops, cache protection)** | [hardening](spec/architecture/shadow-wallet-hardening.md) |
| M5.19 | **Comprehensive wallet security testing** | [hardening](spec/architecture/shadow-wallet-hardening.md) |
| M5.20 | **Wallet-specific security audit preparation** | [hardening](spec/architecture/shadow-wallet-hardening.md) |

**Deliverable:** Fortress-grade wallet with defense-in-depth security, ready for audit.

**Phase 5 Complete Deliverable:** Full crypto wallet with privacy coins, atomic swaps, anonymous DeFi access, hardware-protected keys, comprehensive transaction verification, anti-phishing protection, smart contract security, Shamir backup, and side-channel protections.

---

## Phase 6: Mesh + Community Nodes (Weeks 65-76)

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

## Phase 7: Audit + Launch (Weeks 77-88)

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
                                    [16 weeks - includes hardening]
                                               │
                                               ▼
                                    Phase 6 (Mesh/Nodes)
                                               │
                                               ▼
                                    Phase 7 (Audit/Launch)
```

---

## Shadow Wallet Security Hardening

**Phase 5 now integrates comprehensive wallet security hardening throughout development.**

The Shadow Wallet already implements the full 7-layer Scrambler protection for all financial operations (transaction fragmentation, financial cover traffic, multi-node broadcast, temporal scrambling, network isolation). Phase 5 adds critical application-layer hardening to match messenger security:

### Critical (P0) - Weeks 49-52
- **Hardware Security Module Integration** - Keys in Secure Enclave/StrongBox, never in app memory
- **Transaction Verification Layer** - Prevents clipboard hijacking, address poisoning, replay attacks
- **Enhanced Memory Protection** - mlock, memory encryption, secure zeroing

### High Priority (P1) - Weeks 53-56
- **Anti-Phishing Protection** - Identity verification, suspicious pattern detection
- **Smart Contract Security** - Block unlimited approvals, transaction simulation, contract verification
- **Multi-Signature Coordination** - Secure co-signer coordination via messenger

### Medium Priority (P2) - Weeks 57-60
- **Seed Backup Hardening** - Shamir Secret Sharing (SLIP-39), social recovery
- **Privacy Coin Enhancements** - Monero view keys, auto-CoinJoin, Zcash shielded enforcement
- **Broadcast Verification** - Confirm tx in mempool, fallback mechanism

### Defense in Depth (P3) - Weeks 61-64
- **Side-Channel Mitigation** - Constant-time ops, cache-timing protection
- **Security Testing** - Comprehensive wallet security validation
- **Audit Preparation** - Documentation and threat modeling

### Documentation

- **[WALLET-HARDENING-SUMMARY.md](WALLET-HARDENING-SUMMARY.md)** - Executive summary
- **[SHADOW-WALLET-SECURITY-ANALYSIS.md](SHADOW-WALLET-SECURITY-ANALYSIS.md)** - Detailed gap analysis
- **[spec/architecture/shadow-wallet-hardening.md](spec/architecture/shadow-wallet-hardening.md)** - Technical specifications

**Result:** Wallet security will match messenger security at all layers - both network-level (already complete via Scrambler) and application-level (added through hardening).

---

## Network Privacy Mode (Cellular Metadata Protection)

**Phase 3 now includes protection against cellular network metadata leakage.**

Even with Ghost VPN encrypting all traffic, cellular carriers can still track users via IMSI/IMEI identifiers, phone numbers, and cell tower triangulation. Network Privacy Mode closes this gap with multiple protection strategies:

### The Problem

**What cellular carriers can see (even with VPN):**
- IMSI (SIM card ID) - Persistent tracking across sessions
- IMEI (Hardware ID) - Survives SIM changes
- Phone number - Direct link to real identity
- Cell tower location - Continuous physical tracking
- VPN usage fingerprinting - Identifies privacy tool users

### The Solutions

**1. WiFi-Only Mode (M3.4)**
- Disables cellular data completely
- Only allows traffic over WiFi connections
- Cellular carrier sees NO data traffic
- Prevents VPN usage fingerprinting
- Can use public WiFi for additional anonymity

**2. eSIM Rotation (M3.5)**
- Automatically rotates between multiple eSIM profiles
- Each rotation = new IMSI + new phone number + new carrier
- Rotation strategies: Per session, daily, weekly, or manual
- Breaks long-term cellular tracking
- Works with anonymous eSIM providers (Silent.link, Hushed, Airalo)

**3. Airplane Mode + WiFi (M3.7)**
- Cellular radio completely disabled (no IMSI/IMEI broadcast)
- WiFi enabled for connectivity
- Zero cellular metadata exposure
- Cannot be tracked via cell towers or IMSI catchers

**4. MAC Address Randomization (M3.6)**
- Enforces WiFi MAC randomization (prevents access point tracking)
- Different MAC per WiFi network
- Breaks WiFi-based device tracking

### Threat Mitigation

| Threat | Ghost VPN Alone | + WiFi-Only | + eSIM Rotation | + Airplane+WiFi |
|--------|-----------------|-------------|-----------------|-----------------|
| Content surveillance | ✓ | ✓ | ✓ | ✓ |
| IP address leak | ✓ | ✓ | ✓ | ✓ |
| **IMSI tracking** | ⚠️ | ⚠️ | ✓ Rotated | ✓ Hidden |
| **Phone # linkage** | ⚠️ | ⚠️ | ✓ Rotated | ✓ Hidden |
| **Cell tower location** | ⚠️ | ⚠️ | ⚠️ | ✓ Hidden |
| **VPN fingerprinting** | ⚠️ | ✓ | ⚠️ | ✓ |
| **Long-term profiling** | ⚠️ | ✓ | ✓ | ✓ |

### Documentation

- **[NETWORK-PRIVACY-MODE-SUMMARY.md](NETWORK-PRIVACY-MODE-SUMMARY.md)** - Executive summary
- **[spec/architecture/network-privacy-mode.md](spec/architecture/network-privacy-mode.md)** - Technical specification

**Result:** Complete network anonymity from cellular carrier to internet destination - closes the cellular metadata gap that VPN alone cannot address.

---

## Admin Dashboard (Privacy Control Center)

**Phase 4 includes a comprehensive admin dashboard for power users.**

The Admin Dashboard provides complete transparency and control over every privacy protection in Invisible. Users can see exactly what's active, configure services independently, monitor real-time status, and understand performance metrics.

### Key Features

**1. Configuration Tab**
- Privacy level presets (Paranoid, High, Standard, Low) - one-tap switching
- Individual service toggles (enable/disable each layer independently)
- Fine-grained configuration for each protection layer
- Warnings when disabling critical services
- Save and apply changes with confirmation

**2. Status Dashboard Tab**
- Real-time monitoring of all active services
- Health status for each protection layer
- Connection details (VPN endpoint, network mode, etc.)
- Service metrics (latency, throughput, success rates)
- Auto-refresh every 2 seconds
- Overall system health indicator

**3. Privacy Level Presets**

| Level | Latency | Services | Use Case |
|-------|---------|----------|----------|
| **Paranoid** | 30-90s | ALL max settings | Journalists, activists, whistleblowers |
| **High** (default) | 5-45s | ALL balanced | Privacy-conscious users |
| **Standard** | 2-20s | Most enabled | Everyday secure messaging |
| **Low** | 1-8s | Essential only | Speed priority, low threat |

**4. Message Send Indicator**
- Real-time progress as message routes through Scrambler
- Layer-by-layer status (encrypted, fragmented, mixed, etc.)
- Shows active protections for each message
- Compact indicator in message bubble (tap to expand)
- Delivery time and path information

**5. Service Toggle System**
- Enable/disable individual services
- Core services locked (E2EE, Ghost VPN)
- Optional services configurable (cover traffic, eSIM, wallet features)
- Dependency warnings (e.g., dead drops require mixnet)
- Immediate visual feedback

**6. Metrics & Performance**
- Historical statistics (last 24h, 7d, 30d)
- Latency distribution graphs
- Success rates and uptime
- Bandwidth usage
- Jurisdiction distribution
- Export reports (CSV, JSON)

**7. Activity Logs**
- Detailed event log of all system activity
- Filterable by service, time, log level
- Shows VPN rotations, eSIM changes, message delivery, security events
- Export logs for auditing
- Privacy-preserving (no message content logged)

### Benefits

- ✅ **Complete transparency** - See exactly what protections are active
- ✅ **Full control** - Configure every service independently
- ✅ **Real-time monitoring** - Track system health and performance
- ✅ **Informed decisions** - Understand privacy vs speed tradeoffs
- ✅ **Power user features** - Advanced config, metrics, logs
- ✅ **Education** - Learn what each layer does

### User Experience

**Quick Access:**
- Status bar icon shows current privacy level
- Tap to open admin dashboard
- Swipe between tabs (Config, Status, Metrics, Logs)

**Message Indicator:**
```
Sending message...
✓ Encrypted (Double Ratchet + PQ)
✓ Fragmented (3-of-5 shares)
⏳ Routing through mixnet (Layer 2/5)
✓ Cover traffic active
...
Estimated delivery: 12-18s
```

**After sending:**
```
You: Hey, can we meet at 3pm?
🛡️ [7 layers] 14:32  ← Tap for details
```

### Documentation

- **[ADMIN-DASHBOARD-SUMMARY.md](ADMIN-DASHBOARD-SUMMARY.md)** - Executive summary
- **[spec/architecture/admin-dashboard.md](spec/architecture/admin-dashboard.md)** - Full specification (UI mockups, implementation details)

**Result:** Power users get complete visibility and control over the privacy stack. Casual users can use simple presets. Everyone understands what protections are active and how the system is performing.
