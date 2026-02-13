# Admin Dashboard — Privacy Control Center

> **Purpose:** Comprehensive admin interface for configuring, monitoring, and controlling all
> privacy features. Provides real-time status visibility, service toggles, privacy level
> presets, and message send indicators showing active protections.

---

## Table of Contents

1. [Overview](#overview)
2. [Dashboard Architecture](#dashboard-architecture)
3. [Configuration Tab](#configuration-tab)
4. [Status Dashboard Tab](#status-dashboard-tab)
5. [Privacy Level Presets](#privacy-level-presets)
6. [Message Send Indicator](#message-send-indicator)
7. [Service Toggle System](#service-toggle-system)
8. [Real-Time Monitoring](#real-time-monitoring)
9. [Implementation Specification](#implementation-specification)
10. [User Interface Mockups](#user-interface-mockups)

---

## Overview

The Admin Dashboard is a power-user interface that provides:

- **Full visibility** into every privacy protection layer
- **Fine-grained control** over each service (enable/disable)
- **Real-time status** monitoring of all active protections
- **Privacy presets** for quick configuration (Paranoid, High, Standard, Low)
- **Message indicators** showing which protections are active when sending
- **Performance metrics** for each layer (latency, throughput, success rate)

### Design Principles

1. **Transparency** - User sees exactly what protections are active
2. **Control** - User can enable/disable any service independently
3. **Feedback** - Real-time status updates and indicators
4. **Presets** - Quick configuration via privacy level presets
5. **Education** - Explanations of what each service does

---

## Dashboard Architecture

```
┌──────────────────────────────────────────────────────────┐
│  ADMIN DASHBOARD                                         │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Tabs:  [Configuration] [Status] [Metrics] [Logs]       │
│                                                          │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  CONFIGURATION TAB                                       │
│  ┌────────────────────────────────────────────────────┐ │
│  │  Privacy Level: [High ▼]                           │ │
│  │  Quick Presets: [Paranoid] [High] [Standard] [Low]│ │
│  └────────────────────────────────────────────────────┘ │
│                                                          │
│  Layer 0: Ghost VPN                      [✓] ENABLED    │
│  ├─ Random endpoint selection            [✓]            │
│  ├─ WireGuard encryption                 [✓]            │
│  └─ Session timeout (2min)               [✓]            │
│                                                          │
│  Layer 1: Message Fragmentation          [✓] ENABLED    │
│  ├─ Shamir Secret Sharing (3-of-5)       [✓]            │
│  └─ Independent path routing             [✓]            │
│                                                          │
│  Layer 2: Mixnet                         [✓] ENABLED    │
│  ├─ 5-layer mixing                       [✓]            │
│  ├─ Sphinx packet format                 [✓]            │
│  └─ Batch processing + delays            [✓]            │
│                                                          │
│  Layer 3: Cover Traffic                  [✓] ENABLED    │
│  ├─ Constant-rate packets (1/sec)        [✓]            │
│  ├─ Loop traffic (60%)                   [✓]            │
│  └─ Drop traffic (40%)                   [✓]            │
│                                                          │
│  [... continues for all layers ...]                     │
│                                                          │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  STATUS DASHBOARD TAB                                    │
│  ┌────────────────────────────────────────────────────┐ │
│  │  OVERALL STATUS:  ✓ ALL SYSTEMS OPERATIONAL        │ │
│  │  Privacy Level:   HIGH                             │ │
│  │  Active Services: 42 / 45                          │ │
│  └────────────────────────────────────────────────────┘ │
│                                                          │
│  Ghost VPN:            ✓ CONNECTED                       │
│  Endpoint:             Zurich, Switzerland               │
│  Latency:              23ms                              │
│  Last rotation:        18min ago                         │
│                                                          │
│  Scrambler:            ✓ ACTIVE                          │
│  Messages queued:      3                                 │
│  Cover traffic rate:   1.0 pkt/sec                       │
│  Mix nodes healthy:    47 / 50                           │
│                                                          │
│  Network Privacy:      ✓ WIFI-ONLY MODE                 │
│  Connection:           WiFi ("Home Network")             │
│  MAC randomized:       ✓                                 │
│  Cellular blocked:     ✓                                 │
│                                                          │
│  [... real-time status for all services ...]            │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

---

## Configuration Tab

### Privacy Level Presets

Quick-select buttons for common configurations:

```
┌──────────────────────────────────────────────────────────┐
│  Privacy Level Presets                                   │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  [  PARANOID  ] [    HIGH    ] [  STANDARD  ] [   LOW  ] │
│       ▲              (current)                           │
│       └─ Click to switch to Paranoid mode               │
│                                                          │
│  Current Level: HIGH                                     │
│  - Maximum anonymity                                     │
│  - 30-90 second message latency                          │
│  - All protection layers enabled                         │
│  - WiFi-only mode (cellular blocked)                     │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

**Privacy Level Definitions:**

| Level | Latency | Services | Use Case |
|-------|---------|----------|----------|
| **Paranoid** | 30-90s | ALL enabled, max settings | Journalists, activists, whistleblowers |
| **High** (default) | 5-45s | ALL enabled, balanced | Standard privacy-conscious users |
| **Standard** | 2-20s | Most enabled, reduced delays | Everyday secure messaging |
| **Low** | 1-8s | Essential only, minimal delays | Low-threat, speed priority |

### Layer-by-Layer Configuration

Each privacy layer is individually configurable:

```
┌──────────────────────────────────────────────────────────┐
│  Layer 0: Ghost VPN                      [✓] ENABLED     │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  ┌─ Random Endpoint Selection           [✓] ON          │
│  │   Current endpoint: Zurich, CH                       │
│  │   Rotate on: [Every session ▼]                       │
│  │   [ Rotate Now ]                                     │
│  │                                                      │
│  ┌─ WireGuard Encryption                [✓] ON          │
│  │   Key rotation: Every 2 minutes                      │
│  │   Protocol version: WireGuard 1.0                    │
│  │                                                      │
│  ┌─ Session Timeout                     [✓] ON          │
│  │   Max session time: [120 minutes ▼]                  │
│  │   Auto-lock on timeout: [✓]                          │
│  │                                                      │
│  └─ VPN Kill Switch                     [✓] ON          │
│      No traffic allowed without VPN                     │
│                                                          │
│  [Show Advanced Options ▼]                              │
│                                                          │
└──────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────┐
│  Layer 1: Message Fragmentation          [✓] ENABLED     │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  ┌─ Shamir Secret Sharing               [✓] ON          │
│  │   Threshold (K): [3 ▼]  Total (N): [5 ▼]            │
│  │   Any 3 of 5 shares can reconstruct message          │
│  │   Information-theoretic security                     │
│  │                                                      │
│  ┌─ Independent Path Routing             [✓] ON          │
│  │   Each share takes different path through mixnet     │
│  │   Minimizes correlation risk                         │
│  │                                                      │
│  └─ Packet Size Uniformity               [✓] ON          │
│      All packets exactly 2048 bytes                     │
│                                                          │
└──────────────────────────────────────────────────────────┘

[... Similar cards for each layer ...]
```

### Network Privacy Mode Configuration

```
┌──────────────────────────────────────────────────────────┐
│  Network Privacy Mode                    [✓] ENABLED     │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Current Mode: [WiFi-Only ▼]                            │
│  ○ Normal (cellular + WiFi)                             │
│  ● WiFi-Only                                            │
│  ○ Airplane Mode + WiFi                                 │
│  ○ Cellular with eSIM Rotation                          │
│                                                          │
│  ┌─ eSIM Rotation                       [✓] ON          │
│  │   Rotation strategy: [Daily ▼]                       │
│  │   Profiles installed: 3                              │
│  │   Current: Profile A (Carrier X)                     │
│  │   Last rotated: 18 hours ago                         │
│  │   [ Rotate Now ]                                     │
│  │                                                      │
│  ┌─ MAC Randomization                   [✓] ON          │
│  │   Randomize per network: [✓]                         │
│  │   Current MAC: ab:cd:ef:12:34:56                     │
│  │   Factory MAC hidden: [✓]                            │
│  │                                                      │
│  └─ Auto WiFi-Only Mode                 [✓] ON          │
│      [✓] Enable at home                                 │
│      [✓] Enable at saved locations                      │
│      [ ] Enable on all known WiFi                       │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

### Shadow Wallet Security Configuration

```
┌──────────────────────────────────────────────────────────┐
│  Shadow Wallet Security                  [✓] ENABLED     │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  ┌─ Hardware Security Module            [✓] ON          │
│  │   Keys stored in: Secure Enclave (iOS)               │
│  │   Biometric required: [✓] Face ID                    │
│  │   Hardware available: ✓                              │
│  │                                                      │
│  ┌─ Transaction Verification            [✓] ON          │
│  │   Address validation: [✓]                            │
│  │   Amount verification: [✓]                           │
│  │   Fee outlier detection: [✓]                         │
│  │   Replay protection: [✓]                             │
│  │                                                      │
│  ┌─ Anti-Phishing Protection            [✓] ON          │
│  │   Identity verification: [✓]                         │
│  │   Suspicious pattern detection: [✓]                  │
│  │   First payment confirmation: [✓]                    │
│  │                                                      │
│  ┌─ Smart Contract Security             [✓] ON          │
│  │   Block unlimited approvals: [✓]                     │
│  │   Transaction simulation: [✓]                        │
│  │   Contract verification: [✓]                         │
│  │                                                      │
│  └─ Seed Backup                         [✓] SHAMIR      │
│      Backup type: [Shamir 2-of-3 ▼]                     │
│      [ View Recovery Shares ]                           │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

### Save & Apply Button

```
┌──────────────────────────────────────────────────────────┐
│                                                          │
│  Changes pending: 3 services modified                    │
│                                                          │
│  [ Discard Changes ]          [ Save & Apply ]          │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

---

## Status Dashboard Tab

Real-time monitoring of all active protections:

```
┌──────────────────────────────────────────────────────────┐
│  STATUS DASHBOARD                        Last update: 2s │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  ╔════════════════════════════════════════════════════╗ │
│  ║  OVERALL STATUS:  ✓ ALL SYSTEMS OPERATIONAL        ║ │
│  ║  Privacy Level:   HIGH                             ║ │
│  ║  Active Services: 42 / 45                          ║ │
│  ║  Uptime:          14h 32m                          ║ │
│  ╚════════════════════════════════════════════════════╝ │
│                                                          │
│  ─────────────────────────────────────────────────────── │
│  Network & Connectivity                                  │
│  ─────────────────────────────────────────────────────── │
│                                                          │
│  🌐 Ghost VPN                            ✓ CONNECTED     │
│     Endpoint:       Zurich, Switzerland                  │
│     IP Address:     203.0.113.42                         │
│     Latency:        23ms                                 │
│     Throughput:     ↓ 1.2 Mbps  ↑ 0.8 Mbps              │
│     Last rotation:  18min ago                            │
│     Next rotation:  Auto (42min)                         │
│     Encrypted:      ✓ WireGuard                          │
│                                                          │
│  📱 Network Privacy                      ✓ WIFI-ONLY     │
│     Mode:           WiFi-Only                            │
│     Connection:     WiFi "Home Network"                  │
│     MAC Address:    ab:cd:ef:12:34:56 (randomized)       │
│     Cellular:       ✗ BLOCKED                            │
│     eSIM:           N/A (WiFi-only mode)                 │
│                                                          │
│  ─────────────────────────────────────────────────────── │
│  Scrambler (7 Layers)                                    │
│  ─────────────────────────────────────────────────────── │
│                                                          │
│  🔀 Layer 1: Fragmentation               ✓ ACTIVE        │
│     Messages fragmented: 127                             │
│     Shares generated:    635 (5 per message)             │
│     Reconstruction rate: 100%                            │
│                                                          │
│  🌀 Layer 2: Mixnet                      ✓ ACTIVE        │
│     Mix nodes healthy:   47 / 50                         │
│     Paths active:        5                               │
│     Avg mixing delay:    3.2s                            │
│     Packets in transit:  12                              │
│                                                          │
│  🎭 Layer 3: Cover Traffic               ✓ ACTIVE        │
│     Packet rate:         1.0 pkt/sec (constant)          │
│     Loop packets sent:   2,847 (60%)                     │
│     Drop packets sent:   1,898 (40%)                     │
│     Real packets sent:   127 (<1%)                       │
│     Indistinguishable:   ✓ Yes                           │
│                                                          │
│  🌍 Layer 4: Jurisdiction Routing        ✓ ACTIVE        │
│     Countries per path:  5 minimum                       │
│     Five Eyes nodes:     Max 1 per path                  │
│     Current paths:       CH→IS→RO→PA→SG                  │
│                          CH→IE→PL→CR→NZ                  │
│                          ... (3 more)                    │
│                                                          │
│  🎭 Layer 5: Protocol Camouflage         ✓ ACTIVE        │
│     Transport:           obfs5                           │
│     DPI resistance:      ✓ Unidentifiable                │
│     Fallback available:  uTLS, domain fronting           │
│                                                          │
│  📮 Layer 6: Dead Drops                  ✓ ACTIVE        │
│     Active dead drops:   5                               │
│     Shares deposited:    635                             │
│     Shares retrieved:    508                             │
│     Pending retrieval:   127                             │
│     TTL remaining:       Avg 18h                         │
│                                                          │
│  ⏱️ Layer 7: Temporal Scrambling         ✓ ACTIVE        │
│     Pre-delay:           Poisson(5s)                     │
│     Avg total delay:     12.3s                           │
│     Min delay:           2.1s                            │
│     Max delay:           41.7s                           │
│                                                          │
│  ─────────────────────────────────────────────────────── │
│  Shadow Wallet                                           │
│  ─────────────────────────────────────────────────────── │
│                                                          │
│  💰 Wallet Security                      ✓ HARDENED      │
│     Keys in HSM:         ✓ Secure Enclave                │
│     Transaction verif:   ✓ Active                        │
│     Anti-phishing:       ✓ Active                        │
│     Smart contract sec:  ✓ Active                        │
│     Financial Scrambler: ✓ Full 7-layer                  │
│     Txs broadcast:       42                              │
│     Multi-node fanout:   5 nodes per tx                  │
│                                                          │
│  ─────────────────────────────────────────────────────── │
│  Security Features                                       │
│  ─────────────────────────────────────────────────────── │
│                                                          │
│  🔐 End-to-End Encryption                ✓ ACTIVE        │
│     Double Ratchet:      ✓ Per-message keys              │
│     Post-quantum:        ✓ ML-KEM-1024 + X25519          │
│     Daily key rotation:  ✓ Last: 8h ago                  │
│     Forward secrecy:     ✓ Guaranteed                    │
│     PCS:                 ✓ Guaranteed                    │
│                                                          │
│  🗑️ Auto-Purge                           ✓ ACTIVE        │
│     Mode:                Ghost (24 hours)                │
│     Messages purged:     1,847                           │
│     Next purge:          18h 23m                         │
│     Secure overwrite:    ✓ 3-pass                        │
│                                                          │
│  🚨 Anti-Forensics                       ✓ ACTIVE        │
│     No thumbnails:       ✓                               │
│     No clipboard:        ✓                               │
│     Blank task switcher: ✓                               │
│     Screen capture:      ✗ BLOCKED                       │
│     Secure keyboard:     ✓ Active                        │
│                                                          │
│  🔒 Access Control                       ✓ ACTIVE        │
│     Biometric:           ✓ Face ID                       │
│     2FA:                 ✓ TOTP                          │
│     Duress PIN:          ✓ Configured                    │
│     Panic gesture:       ✓ Enabled                       │
│     Remote wipe:         ✓ Ready                         │
│                                                          │
└──────────────────────────────────────────────────────────┘

[ Refresh ]  [ Export Report ]  [ View Logs ]
```

### Status Indicators

**Color-coded status:**
- 🟢 ✓ Green: Active and healthy
- 🟡 ⚠ Yellow: Active with warnings
- 🔴 ✗ Red: Error or disabled
- ⚪ ○ Gray: Not applicable

**Real-time updates:**
- Dashboard refreshes every 2 seconds
- Critical alerts show immediately
- Metrics update in real-time

---

## Privacy Level Presets

### Preset Definitions

```rust
pub enum PrivacyLevel {
    Paranoid,
    High,
    Standard,
    Low,
}

pub struct PrivacyPreset {
    level: PrivacyLevel,
    description: String,
    latency: String,
    services: ServiceConfiguration,
}

impl PrivacyPreset {
    pub fn paranoid() -> Self {
        PrivacyPreset {
            level: PrivacyLevel::Paranoid,
            description: "Maximum anonymity - All protections enabled".to_string(),
            latency: "30-90 seconds".to_string(),
            services: ServiceConfiguration {
                // Ghost VPN
                ghost_vpn_enabled: true,
                vpn_endpoint_rotation: RotationFrequency::EverySession,

                // Network Privacy
                network_mode: NetworkMode::AirplaneModeWithWiFi,
                esim_rotation: RotationStrategy::PerSession,
                mac_randomization: true,

                // Scrambler
                message_fragmentation: true,
                shamir_k: 3,
                shamir_n: 5,
                mixnet_enabled: true,
                mixnet_layers: 5,
                cover_traffic_enabled: true,
                cover_traffic_rate: 1.0, // pkt/sec
                jurisdiction_routing: true,
                min_jurisdictions: 5,
                protocol_camouflage: ProtocolCamouflage::Obfs5,
                dead_drops_enabled: true,
                temporal_scrambling: true,
                temporal_delay_mean: 15.0, // seconds
                urgency_mode: UrgencyMode::Maximum,

                // Wallet
                wallet_hsm: true,
                transaction_verification: true,
                anti_phishing: true,
                smart_contract_security: true,

                // Security
                auto_purge_hours: 24,
                anti_forensics: true,
                biometric_required: true,
                two_factor_required: true,
            }
        }
    }

    pub fn high() -> Self {
        // Default - balanced security and usability
        // Similar to Paranoid but:
        // - Network mode: WiFi-Only (not airplane)
        // - eSIM rotation: Daily (not per-session)
        // - Temporal delay: 5s (not 15s)
        // - Urgency mode: High (not Maximum)
    }

    pub fn standard() -> Self {
        // Reduced latency, most protections active
        // - Temporal delay: 2s
        // - Urgency mode: Standard
        // - Network mode: Normal
        // - eSIM rotation: Weekly
    }

    pub fn low() -> Self {
        // Essential protections only
        // - Cover traffic: 0.5 pkt/sec
        // - Temporal delay: 0.5s
        // - Urgency mode: LowLatency
        // - Minimal delays
        // - Network mode: Normal
    }
}
```

### Preset Switching

```
When user selects new preset:

1. Show confirmation dialog:
   ┌────────────────────────────────────────┐
   │  Switch to PARANOID mode?              │
   │                                        │
   │  This will enable:                     │
   │  ✓ All protection layers (max)         │
   │  ✓ Airplane Mode + WiFi only           │
   │  ✓ eSIM rotation per session           │
   │  ✓ Maximum delays (30-90s latency)     │
   │                                        │
   │  Your messages will be:                │
   │  • Slower (30-90 second delivery)      │
   │  • More anonymous (maximum protection) │
   │                                        │
   │  [ Cancel ]  [ Switch to Paranoid ]    │
   └────────────────────────────────────────┘

2. Apply configuration:
   - Update all service settings
   - Reconnect VPN if needed
   - Switch network mode
   - Update Scrambler parameters

3. Show confirmation:
   ┌────────────────────────────────────────┐
   │  ✓ Switched to PARANOID mode           │
   │                                        │
   │  All maximum protections active.       │
   │  Message latency: 30-90 seconds        │
   │                                        │
   │  [ OK ]                                │
   └────────────────────────────────────────┘
```

---

## Message Send Indicator

### Visual Feedback When Sending Messages

When user sends a message, show real-time indicator of active protections:

```
┌──────────────────────────────────────────────────────────┐
│  Sending message...                                      │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  ✓ Encrypted (Double Ratchet + Post-Quantum)            │
│  ✓ Fragmented (3-of-5 Shamir shares)                    │
│  ⏳ Routing through mixnet (Layer 2/5)                   │
│  ✓ Cover traffic active (1.0 pkt/sec)                   │
│  ✓ Multi-jurisdiction routing (5 countries)             │
│  ✓ Protocol camouflage (obfs5)                          │
│  ⏳ Depositing at dead drops (3/5 complete)              │
│  ⏱️ Temporal delay: 3.2s remaining                       │
│                                                          │
│  Current path: CH → IS → RO → PA → SG                   │
│                                                          │
│  Estimated delivery: 12-18 seconds                       │
│                                                          │
└──────────────────────────────────────────────────────────┘

Legend:
✓ = Complete
⏳ = In progress
⏱️ = Waiting
```

### Compact Indicator (In Conversation)

After sending, show compact indicator in message bubble:

```
┌────────────────────────────────────────┐
│  You: Hey, can we meet at 3pm?        │
│  🛡️ [7 layers] 14:32                  │  ← Click to expand
└────────────────────────────────────────┘
         │
         └─ Click expands to:

┌────────────────────────────────────────┐
│  Protection Layers Applied:            │
│  ✓ E2EE (Double Ratchet + PQ)          │
│  ✓ Fragmentation (3-of-5 Shamir)       │
│  ✓ Mixnet (5 layers)                   │
│  ✓ Cover traffic                       │
│  ✓ Jurisdiction routing (5 countries)  │
│  ✓ Protocol camouflage (obfs5)         │
│  ✓ Dead drops                          │
│  ✓ Temporal scrambling (5.3s delay)    │
│                                        │
│  Network: Ghost VPN (Zurich, CH)       │
│  Sent: 14:32:07                        │
│  Delivered: 14:32:19 (12s latency)     │
└────────────────────────────────────────┘
```

### Settings: Message Indicator Verbosity

```
┌──────────────────────────────────────────────────────────┐
│  Message Send Indicators                                 │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Show indicator when sending:                            │
│  ○ Always (show full indicator every send)              │
│  ● On request (tap message to see details)              │
│  ○ Never (no indicators)                                │
│                                                          │
│  Indicator style:                                        │
│  ● Compact (🛡️ [7 layers])                              │
│  ○ Detailed (list all layers)                           │
│                                                          │
│  Show layer-by-layer progress:                          │
│  [✓] Yes (animated progress as message routes)          │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

---

## Service Toggle System

### Independent Service Control

Each service can be toggled independently:

```
┌──────────────────────────────────────────────────────────┐
│  Service Controls                                        │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Core Services (Cannot disable)                          │
│  ├─ [🔒] End-to-End Encryption                          │
│  └─ [🔒] Ghost VPN                                      │
│                                                          │
│  Scrambler Layers (Recommended)                          │
│  ├─ [✓] Layer 1: Message Fragmentation                  │
│  ├─ [✓] Layer 2: Mixnet                                 │
│  ├─ [✓] Layer 3: Cover Traffic                          │
│  ├─ [✓] Layer 4: Jurisdiction Routing                   │
│  ├─ [✓] Layer 5: Protocol Camouflage                    │
│  ├─ [✓] Layer 6: Dead Drops                             │
│  └─ [✓] Layer 7: Temporal Scrambling                    │
│                                                          │
│  Network Privacy (Optional)                              │
│  ├─ [✓] WiFi-Only Mode                                  │
│  ├─ [ ] eSIM Rotation                                   │
│  └─ [✓] MAC Randomization                               │
│                                                          │
│  Wallet Security (Optional for non-wallet users)         │
│  ├─ [✓] Hardware Security Module                        │
│  ├─ [✓] Transaction Verification                        │
│  ├─ [✓] Anti-Phishing Protection                        │
│  └─ [✓] Smart Contract Security                         │
│                                                          │
│  Privacy Features (Configurable)                         │
│  ├─ [✓] Auto-Purge (24h)                                │
│  ├─ [✓] Anti-Forensics                                  │
│  ├─ [✓] Screen Capture Blocking                         │
│  └─ [✓] Secure Keyboard                                 │
│                                                          │
└──────────────────────────────────────────────────────────┘

When user toggles a service:

1. If disabling critical service:
   ┌────────────────────────────────────────┐
   │  ⚠️ Disable Cover Traffic?             │
   │                                        │
   │  This will:                            │
   │  • Reduce anonymity significantly      │
   │  • Expose traffic timing patterns      │
   │  • Make real messages distinguishable  │
   │                                        │
   │  Only disable if:                      │
   │  - You're on metered connection        │
   │  - Speed is critical                   │
   │  - You accept reduced privacy          │
   │                                        │
   │  [ Cancel ]  [ Disable Anyway ]        │
   └────────────────────────────────────────┘

2. If enabling resource-intensive service:
   ┌────────────────────────────────────────┐
   │  Enable eSIM Rotation?                 │
   │                                        │
   │  This requires:                        │
   │  • Multiple eSIM profiles installed    │
   │  • Data plan for each profile          │
   │  • eSIM-capable device                 │
   │                                        │
   │  You have: 0 eSIM profiles installed   │
   │                                        │
   │  [ Cancel ]  [ Set Up eSIMs ]          │
   └────────────────────────────────────────┘
```

### Dependency Warnings

Some services depend on others:

```rust
pub struct ServiceDependencies {
    service: Service,
    depends_on: Vec<Service>,
    required_by: Vec<Service>,
}

// Example: Dead Drops depend on Mixnet
if user disables Mixnet:
    warn!("Dead Drops require Mixnet");
    prompt: "Disable Dead Drops too?"

// Example: Wallet features require wallet
if !wallet_enabled && user enables transaction_verification:
    warn!("Transaction verification requires Shadow Wallet");
    prompt: "Enable Shadow Wallet first?"
```

---

## Real-Time Monitoring

### Metrics Tab

Performance and health metrics for each service:

```
┌──────────────────────────────────────────────────────────┐
│  METRICS & PERFORMANCE                  Last 24 hours    │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Ghost VPN                                               │
│  ├─ Uptime:          99.8%                               │
│  ├─ Avg latency:     24ms                                │
│  ├─ Reconnections:   2                                   │
│  ├─ Data transferred: ↓ 1.2 GB  ↑ 0.8 GB                │
│  └─ Endpoints used:  7 (rotated)                         │
│                                                          │
│  Scrambler                                               │
│  ├─ Messages sent:       1,847                           │
│  ├─ Shares generated:    9,235 (5 per message)           │
│  ├─ Reconstruction rate: 100%                            │
│  ├─ Avg delivery time:   12.3s                           │
│  ├─ Mix nodes used:      47                              │
│  ├─ Cover traffic sent:  86,400 packets                  │
│  └─ Dead drops active:   5 simultaneous                  │
│                                                          │
│  Network Privacy                                         │
│  ├─ Mode switches:       3 (WiFi-only → Normal)          │
│  ├─ Cellular blocked:    18h 32m                         │
│  ├─ eSIM rotations:      N/A (WiFi-only mode)            │
│  ├─ WiFi networks:       2 ("Home", "Office")            │
│  └─ MAC randomized:      ✓ Both networks                 │
│                                                          │
│  Shadow Wallet                                           │
│  ├─ Transactions:        42 sent                         │
│  ├─ Multi-node fanout:   5 nodes per tx                  │
│  ├─ Verifications:       42 / 42 passed                  │
│  ├─ Phishing blocked:    1 attempt                       │
│  ├─ Smart contract sims: 8 simulated                     │
│  └─ HSM signatures:      42 (all in hardware)            │
│                                                          │
│  ─────────────────────────────────────────────────────── │
│                                                          │
│  📊 Latency Distribution (Messages)                      │
│                                                          │
│  0-5s:    ████████░░░░░░░░░░░░ 35%                      │
│  5-10s:   ████████████░░░░░░░░ 48%                      │
│  10-20s:  ████░░░░░░░░░░░░░░░░ 14%                      │
│  20-45s:  ██░░░░░░░░░░░░░░░░░░  3%                      │
│  45s+:    ░░░░░░░░░░░░░░░░░░░░ <1%                      │
│                                                          │
│  Median: 8.2s  |  95th percentile: 18.7s                │
│                                                          │
│  ─────────────────────────────────────────────────────── │
│                                                          │
│  🌍 Jurisdiction Distribution (Mix Nodes)                │
│                                                          │
│  Switzerland:  ████████████░░ 32%                        │
│  Iceland:      ███████░░░░░░░ 18%                        │
│  Romania:      ██████░░░░░░░░ 15%                        │
│  Panama:       █████░░░░░░░░░ 12%                        │
│  Singapore:    █████░░░░░░░░░ 12%                        │
│  Other (8):    ████░░░░░░░░░░ 11%                        │
│                                                          │
│  Five Eyes nodes: 2.3% (within limit)                   │
│                                                          │
└──────────────────────────────────────────────────────────┘

[ Export CSV ]  [ View Graphs ]  [ Reset Stats ]
```

### Logs Tab

Detailed activity logs:

```
┌──────────────────────────────────────────────────────────┐
│  ACTIVITY LOGS                          Filter: [All ▼]  │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  14:32:19  ✓ Message delivered (12.3s latency)           │
│  14:32:07  ⏳ Message sent, entering Scrambler            │
│  14:30:42  ✓ Ghost VPN reconnected (Zurich, CH)          │
│  14:30:41  ⚠ Ghost VPN connection interrupted            │
│  14:28:15  ✓ eSIM rotated (Profile A → Profile B)        │
│  14:15:03  ✓ Network mode switched (Normal → WiFi-Only)  │
│  14:12:38  ✓ Transaction broadcast (5 nodes)             │
│  14:12:35  ✓ Transaction verification passed             │
│  14:12:33  ⏳ User initiated transaction (0.5 ETH)        │
│  13:58:22  ⚠ Phishing attempt blocked (suspicious addr)  │
│  13:45:17  ✓ Auto-purge completed (84 messages)          │
│  12:30:05  ✓ Daily key rotation completed                │
│  11:15:42  ✓ Biometric authentication (Face ID)          │
│                                                          │
│  [ Load More ]                                           │
│                                                          │
│  Filters:                                                │
│  [✓] Info  [✓] Warning  [✓] Error                       │
│  Service: [All ▼]  Time: [Last 24h ▼]                   │
│                                                          │
└──────────────────────────────────────────────────────────┘

[ Export Logs ]  [ Clear Logs ]
```

---

## Implementation Specification

### Data Models

```rust
pub struct AdminDashboard {
    configuration: ConfigurationManager,
    status: StatusMonitor,
    metrics: MetricsCollector,
    logs: ActivityLogger,
}

pub struct ConfigurationManager {
    current_preset: PrivacyLevel,
    services: HashMap<ServiceId, ServiceConfig>,
    pending_changes: Vec<ConfigChange>,
}

pub struct StatusMonitor {
    last_update: DateTime<Utc>,
    service_statuses: HashMap<ServiceId, ServiceStatus>,
    overall_health: HealthStatus,
}

pub enum ServiceStatus {
    Active { healthy: bool, details: StatusDetails },
    Inactive,
    Error { message: String },
}

pub struct MetricsCollector {
    time_range: TimeRange,
    metrics: HashMap<ServiceId, ServiceMetrics>,
}

pub struct ServiceMetrics {
    uptime: Duration,
    success_rate: f64,
    avg_latency: Duration,
    throughput: DataRate,
    custom_metrics: HashMap<String, MetricValue>,
}

pub struct ActivityLogger {
    logs: VecDeque<LogEntry>,
    max_entries: usize,
    filters: LogFilters,
}

pub struct LogEntry {
    timestamp: DateTime<Utc>,
    level: LogLevel,
    service: ServiceId,
    message: String,
    details: Option<serde_json::Value>,
}
```

### UI Components

```rust
// Configuration Tab
pub struct ConfigurationTab {
    preset_selector: PresetSelector,
    layer_configs: Vec<LayerConfigCard>,
    save_button: Button,
}

impl ConfigurationTab {
    pub fn render(&self) -> Html {
        html! {
            <div class="config-tab">
                <PresetSelector
                    current={self.preset_selector.current}
                    on_change={self.on_preset_change}
                />

                {for self.layer_configs.iter().map(|config| {
                    config.render()
                })}

                <SaveButton
                    changes={self.pending_changes.len()}
                    on_click={self.save_configuration}
                />
            </div>
        }
    }

    fn on_preset_change(&mut self, preset: PrivacyLevel) {
        // Load preset configuration
        let config = PrivacyPreset::from_level(preset);

        // Apply to all services
        self.apply_preset(config);

        // Mark as pending changes
        self.mark_changes_pending();
    }
}

// Status Dashboard Tab
pub struct StatusDashboardTab {
    status_monitor: Arc<Mutex<StatusMonitor>>,
    refresh_interval: Duration,
}

impl StatusDashboardTab {
    pub fn render(&self) -> Html {
        let status = self.status_monitor.lock().unwrap();

        html! {
            <div class="status-dashboard">
                <OverallStatus health={status.overall_health} />

                <ServiceStatusCard
                    service="Ghost VPN"
                    status={status.get("ghost_vpn")}
                />

                <ServiceStatusCard
                    service="Scrambler"
                    status={status.get("scrambler")}
                />

                // ... more status cards

                <RefreshButton interval={self.refresh_interval} />
            </div>
        }
    }

    pub async fn auto_refresh(&mut self) {
        loop {
            tokio::time::sleep(self.refresh_interval).await;
            self.update_status().await;
        }
    }
}

// Message Send Indicator
pub struct MessageSendIndicator {
    message_id: MessageId,
    layers_complete: Vec<LayerId>,
    total_layers: usize,
}

impl MessageSendIndicator {
    pub fn render(&self) -> Html {
        html! {
            <div class="send-indicator">
                <h3>{"Sending message..."}</h3>

                {for LAYERS.iter().map(|layer| {
                    let complete = self.layers_complete.contains(&layer.id);
                    html! {
                        <LayerIndicator
                            name={layer.name}
                            status={if complete { "complete" } else { "pending" }}
                        />
                    }
                })}

                <EstimatedDelivery time={self.estimate_delivery()} />
            </div>
        }
    }

    pub fn update_progress(&mut self, layer_id: LayerId) {
        self.layers_complete.push(layer_id);
        self.re_render();
    }
}
```

### Real-Time Updates

```rust
pub struct DashboardUpdater {
    status_monitor: Arc<Mutex<StatusMonitor>>,
    event_rx: mpsc::Receiver<SystemEvent>,
}

impl DashboardUpdater {
    pub async fn run(&mut self) {
        while let Some(event) = self.event_rx.recv().await {
            match event {
                SystemEvent::ServiceStatusChanged { service, status } => {
                    let mut monitor = self.status_monitor.lock().unwrap();
                    monitor.update_service_status(service, status);
                }

                SystemEvent::MessageSent { id, layer } => {
                    // Update send indicator for this message
                    self.update_send_indicator(id, layer);
                }

                SystemEvent::MetricUpdated { service, metric, value } => {
                    // Update metrics display
                    self.update_metric(service, metric, value);
                }

                // ... handle other events
            }
        }
    }
}
```

---

## User Interface Mockups

### Main Dashboard (Desktop)

```
┌─────────────────────────────────────────────────────────────────────┐
│  Invisible                                          Admin Dashboard  │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  [Configuration] [Status] [Metrics] [Logs]                          │
│  ───────────────────────────────────────────────────────────────────│
│                                                                     │
│  Privacy Level: HIGH                                                │
│  [  PARANOID  ] [    HIGH    ] [  STANDARD  ] [   LOW  ]           │
│                      ▲                                              │
│                      └─ Current                                     │
│                                                                     │
│  ╔═══════════════════════════════════════════════════════════════╗ │
│  ║  Layer 0: Ghost VPN                      [✓] ENABLED          ║ │
│  ╠═══════════════════════════════════════════════════════════════╣ │
│  ║  ├─ Random endpoint selection            [✓] ON               ║ │
│  ║  ├─ WireGuard encryption                 [✓] ON               ║ │
│  ║  ├─ Session timeout (2min)               [✓] ON               ║ │
│  ║  └─ VPN kill switch                      [✓] ON               ║ │
│  ╚═══════════════════════════════════════════════════════════════╝ │
│                                                                     │
│  ╔═══════════════════════════════════════════════════════════════╗ │
│  ║  Layer 1: Message Fragmentation          [✓] ENABLED          ║ │
│  ╠═══════════════════════════════════════════════════════════════╣ │
│  ║  ├─ Shamir Secret Sharing (3-of-5)       [✓] ON               ║ │
│  ║  └─ Independent path routing             [✓] ON               ║ │
│  ╚═══════════════════════════════════════════════════════════════╝ │
│                                                                     │
│  [... 5 more layers ...]                                           │
│                                                                     │
│  ╔═══════════════════════════════════════════════════════════════╗ │
│  ║  Network Privacy Mode                    [✓] ENABLED          ║ │
│  ╠═══════════════════════════════════════════════════════════════╣ │
│  ║  Current Mode: WiFi-Only                                      ║ │
│  ║  ├─ eSIM Rotation                        [ ] OFF              ║ │
│  ║  └─ MAC Randomization                    [✓] ON               ║ │
│  ╚═══════════════════════════════════════════════════════════════╝ │
│                                                                     │
│  [ Discard Changes ]                    [ Save & Apply ]           │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### Status Dashboard (Mobile)

```
┌───────────────────────────┐
│  ☰  Invisible      [⚙]   │
├───────────────────────────┤
│  Status Dashboard         │
├───────────────────────────┤
│                           │
│  ╔═══════════════════════╗│
│  ║  ✓ ALL SYSTEMS OK     ║│
│  ║  Privacy: HIGH        ║│
│  ║  Services: 42/45      ║│
│  ╚═══════════════════════╝│
│                           │
│  🌐 Ghost VPN             │
│  ✓ CONNECTED              │
│  Zurich, CH  |  23ms      │
│                           │
│  📱 Network Privacy        │
│  ✓ WIFI-ONLY              │
│  "Home Network"           │
│                           │
│  🔀 Scrambler              │
│  ✓ ACTIVE (7 layers)      │
│  Packets: 12 in transit   │
│                           │
│  💰 Shadow Wallet          │
│  ✓ HARDENED               │
│  HSM: Secure Enclave      │
│                           │
│  🔐 E2EE                   │
│  ✓ ACTIVE                 │
│  Double Ratchet + PQ      │
│                           │
│  🗑️ Auto-Purge             │
│  ✓ ACTIVE (24h)           │
│  Next: 18h 23m            │
│                           │
│  [ Refresh ]  [ Details ] │
│                           │
└───────────────────────────┘
```

---

## Cross-References

- [ghost-vpn.md](ghost-vpn.md) - Ghost VPN architecture
- [scrambler.md](scrambler.md) - 7-layer Scrambler system
- [network-privacy-mode.md](network-privacy-mode.md) - Cellular metadata protection
- [shadow-wallet-hardening.md](shadow-wallet-hardening.md) - Wallet security
- [cryptography.md](cryptography.md) - Encryption systems

---

*The Admin Dashboard provides complete transparency and control over every privacy
protection in Invisible. Users can see exactly what protections are active, configure
them independently, switch between privacy levels, and monitor real-time status. This
visibility builds trust and empowers users to make informed privacy decisions.*
