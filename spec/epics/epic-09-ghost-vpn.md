# Epic 09: Ghost VPN — Built-In WireGuard VPN

## Overview
Implement the mandatory Ghost VPN: a built-in WireGuard tunnel that auto-connects on app launch with random global endpoints, ephemeral keys, and automatic session management.

## User Stories

### US-09.1: Mandatory VPN Connection
As a user, I want the app to automatically connect through a VPN before any messaging is available.

**Acceptance Criteria:**
- VPN connects automatically after 2FA authentication
- No messaging features available until VPN handshake completes
- Connection status clearly indicated in UI
- NO VPN = NO APP — enforced at transport layer

### US-09.2: Random Global Endpoint
As a user, I want each VPN session to use a different random server location.

**Acceptance Criteria:**
- Random selection from 50+ global endpoints
- Never same node twice consecutively
- Geographic diversity: never same country as user
- Zero Five Eyes country endpoints
- Node health-checked (slow nodes deprioritized)

### US-09.3: Ephemeral VPN Keys
As a user, I want no persistent VPN identity across sessions.

**Acceptance Criteria:**
- New WireGuard keypair generated per session
- No VPN account (no username/password)
- Keys destroyed on session end
- No linkability between sessions

### US-09.4: Session Timeout
As a user, I want VPN sessions to auto-expire for security.

**Acceptance Criteria:**
- Inactivity timeout: configurable (5min / 15min / 30min / 1hr), default 30min
- Max session lifetime: configurable, default 4hr (even if active)
- On timeout: VPN torn down, app locked, session keys wiped
- On reopen: fresh 2FA + new endpoint + new keys
- Hard kill: closing app immediately tears down tunnel

### US-09.5: Multi-Hop VPN
As a user, I want an option to route through 2 VPN nodes for additional privacy.

**Acceptance Criteria:**
- Optional 2-hop VPN routing
- Different countries for each hop
- Slight latency increase clearly communicated
- First hop and second hop use independent ephemeral keys

## Technical Requirements
- WireGuard: wireguard-rs or platform WireGuard APIs
- Protocol: ChaCha20, Poly1305, Curve25519
- Node discovery: signed node list fetched on launch
- Connection management: automatic reconnect on network change

## Dependencies
- Epic 00 (Foundation), Epic 16 (Access Control) — for 2FA integration

## Architecture References
- [ghost-vpn.md](../architecture/ghost-vpn.md), [scrambler.md](../architecture/scrambler.md)
