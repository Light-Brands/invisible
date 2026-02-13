# Ghost VPN — Mandatory Encrypted Tunnel (Layer 0)

## Overview
Ghost VPN is Invisible's mandatory front door. Before the Scrambler, before messaging, before the app loads conversations — you must connect through the built-in VPN. NO VPN = NO APP.

## App Launch Sequence
1. User opens Invisible
2. Biometric + 2FA authentication
3. Ghost VPN auto-connects (random endpoint from global pool)
4. WireGuard tunnel established (handshake verified)
5. App loads — messaging available
6. 30-minute inactivity timer starts
7. On timeout: VPN torn down, app locks, session data cleared, next open = new random endpoint

## Protocol: WireGuard
- Modern, audited, minimal attack surface (~4,000 lines of code)
- ChaCha20 encryption, Poly1305 authentication, Curve25519 key exchange
- Minimal latency overhead
- Built into the app (no separate VPN app)

## Connection Properties
- Random endpoint selection from 50+ global nodes
- Never same node twice consecutively
- Ephemeral keys: new WireGuard keypair per session (no persistent VPN identity)
- No VPN account: no username/password for VPN layer
- Zero VPN logs: nodes run RAM-only
- Multi-hop option: route through 2 VPN nodes for additional IP masking

## Global Node Pool
- 50+ endpoints across 30+ countries
- Zero Five Eyes countries as endpoints
- Selection: RANDOM every connection
- Geographic diversity enforced: app won't select VPN node in same country as user
- Community-operated: anyone can run a Ghost VPN node
- Nodes are stateless: RAM-only, no connection logs, no bandwidth logs
- Health-checked: slow/unreliable nodes auto-deprioritized
- Warrant canary system: signed "no warrants received" attestations; absence = exclusion from pool

## Session Management
- 30-minute inactivity timeout (configurable: 5min / 15min / 30min / 1hr)
- On timeout: VPN destroyed, app locked, session keys wiped from RAM
- On reopen: fresh 2FA, new random VPN endpoint, new ephemeral keys
- No "stay connected" option: every session has max lifetime (default 4hr, configurable)
- Hard kill: closing app immediately tears down VPN tunnel

## What This Achieves
Without Ghost VPN: ISP sees "User connected to Invisible relay at IP 1.2.3.4" → knows you use Invisible
With Ghost VPN: ISP sees "VPN traffic to random country" → could be anything. VPN node sees encrypted Sphinx packets, doesn't know who you are.

Combined path: ISP → VPN (random) → Mixnet (5-layer) → Dead Drop → Recipient = 7+ hops with 3 different anonymization technologies.

## VPN Node Infrastructure
- Stateless: RAM-only, zero disk
- Health monitoring by network
- Geographic diversity enforced
- Warrant canary with signed attestations
- Auto-exclusion on canary absence

Cross-references: [scrambler.md](scrambler.md), [zero-log-doctrine.md](zero-log-doctrine.md), [access-control.md](access-control.md)
